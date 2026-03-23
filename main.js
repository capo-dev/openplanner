// ─────────────────────────────────────────────────────────────────
//  main.js  —  OpenPlanner Electron Main Process
// ─────────────────────────────────────────────────────────────────
const { app, BrowserWindow, ipcMain, shell } = require('electron')
const path   = require('path')
const https  = require('https')
const http   = require('http')
const { spawn, exec } = require('child_process')
const Store  = require('electron-store')

const store = new Store({ name: 'openplanner-data' })
// ── Window ────────────────────────────────────────────────────────
function createWindow() {
  const win = new BrowserWindow({
    width: 1300, height: 840, minWidth: 1020, minHeight: 640,
    frame: false, backgroundColor: '#0d0d14',
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true, nodeIntegration: false
    },
    icon: path.join(__dirname, 'assets', 'icon.ico'),
    show: false
  })
  win.loadFile('index.html')
  win.once('ready-to-show', () => win.show())
}

// ── Window controls ───────────────────────────────────────────────
ipcMain.on('win-minimize', () => BrowserWindow.getFocusedWindow()?.minimize())
ipcMain.on('win-maximize', () => {
  const w = BrowserWindow.getFocusedWindow()
  w?.isMaximized() ? w.unmaximize() : w.maximize()
})
ipcMain.on('win-close', () => BrowserWindow.getFocusedWindow()?.close())

// ── Persistent store ──────────────────────────────────────────────
ipcMain.handle('store:get',    (_e, key)        => store.get(key))
ipcMain.handle('store:set',    (_e, key, value) => store.set(key, value))
ipcMain.handle('store:delete', (_e, key)        => store.delete(key))

// ── Ollama: check if running ──────────────────────────────────────
ipcMain.handle('ollama:check', () => {
  return new Promise(resolve => {
    const req = http.get('http://localhost:11434/api/tags', res => {
      let raw = ''
      res.on('data', c => raw += c)
      res.on('end', () => {
        try { resolve({ running: true, models: JSON.parse(raw).models || [] }) }
        catch  { resolve({ running: true, models: [] }) }
      })
    })
    req.on('error', () => resolve({ running: false, models: [] }))
    req.setTimeout(2500, () => { req.destroy(); resolve({ running: false, models: [] }) })
  })
})

// ── Ollama: pull model with streaming progress ────────────────────
ipcMain.handle('ollama:pull', (event, model) => {
  return new Promise((resolve, reject) => {
    const body = JSON.stringify({ name: model, stream: true })
    const req  = http.request({
      hostname: 'localhost', port: 11434,
      path: '/api/pull', method: 'POST',
      headers: { 'Content-Type': 'application/json', 'Content-Length': Buffer.byteLength(body) }
    }, res => {
      res.on('data', chunk => {
        try {
          const lines = chunk.toString().split('\n').filter(Boolean)
          for (const line of lines) {
            const d = JSON.parse(line)
            event.sender.send('ollama:pull-progress', d)
            if (d.status === 'success') resolve({ success: true })
          }
        } catch {}
      })
      res.on('end', () => resolve({ success: true }))
    })
    req.on('error', e => reject(e))
    req.write(body)
    req.end()
  })
})

// ── Ollama: generate (streaming back to renderer) ─────────────────
ipcMain.handle('ollama:generate', (event, { model, system, prompt }) => {
  return new Promise((resolve, reject) => {
    const body = JSON.stringify({
      model: model || 'llama3',
      system,
      prompt,
      stream: true
    })
    const req = http.request({
      hostname: 'localhost', port: 11434,
      path: '/api/generate', method: 'POST',
      headers: { 'Content-Type': 'application/json', 'Content-Length': Buffer.byteLength(body) }
    }, res => {
      let full = ''
      res.on('data', chunk => {
        try {
          const lines = chunk.toString().split('\n').filter(Boolean)
          for (const line of lines) {
            const d = JSON.parse(line)
            if (d.response) {
              full += d.response
              event.sender.send('ollama:stream-token', d.response)
            }
          }
        } catch {}
      })
      res.on('end', () => resolve({ text: full }))
    })
    req.on('error', reject)
    req.write(body)
    req.end()
  })
})

// ── Open external browser ─────────────────────────────────────────
ipcMain.handle('shell:open', (_e, url) => shell.openExternal(url))

// ── Google OAuth ──────────────────────────────────────────────────
ipcMain.handle('google:auth-url', () => {
  const clientId = store.get('googleClientId')
  if (!clientId) return null
  const params = new URLSearchParams({
    client_id: clientId, redirect_uri: 'urn:ietf:wg:oauth:2.0:oob',
    response_type: 'code', scope: 'https://www.googleapis.com/auth/calendar',
    access_type: 'offline', prompt: 'consent'
  })
  return `https://accounts.google.com/o/oauth2/auth?${params}`
})

ipcMain.handle('google:exchange-code', (_e, code) => {
  const clientId = store.get('googleClientId'), clientSecret = store.get('googleClientSecret')
  if (!clientId || !clientSecret) return { error: 'Missing credentials' }
  return new Promise((resolve, reject) => {
    const body = new URLSearchParams({
      code, client_id: clientId, client_secret: clientSecret,
      redirect_uri: 'urn:ietf:wg:oauth:2.0:oob', grant_type: 'authorization_code'
    }).toString()
    const req = https.request({
      hostname: 'oauth2.googleapis.com', path: '/token', method: 'POST',
      headers: { 'Content-Type': 'application/x-www-form-urlencoded', 'Content-Length': Buffer.byteLength(body) }
    }, res => {
      let raw = ''
      res.on('data', c => raw += c)
      res.on('end', () => {
        try {
          const data = JSON.parse(raw)
          if (data.access_token) {
            store.set('googleToken', data.access_token)
            if (data.refresh_token) store.set('googleRefreshToken', data.refresh_token)
          }
          resolve(data)
        } catch (e) { reject(e) }
      })
    })
    req.on('error', reject); req.write(body); req.end()
  })
})

// ── App lifecycle ─────────────────────────────────────────────────
app.whenReady().then(createWindow)
app.on('window-all-closed', () => { if (process.platform !== 'darwin') app.quit() })