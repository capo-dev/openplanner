const { contextBridge, ipcRenderer } = require('electron')

contextBridge.exposeInMainWorld('op', {
  store: {
    get:    (key)        => ipcRenderer.invoke('store:get', key),
    set:    (key, value) => ipcRenderer.invoke('store:set', key, value),
    delete: (key)        => ipcRenderer.invoke('store:delete', key)
  },
  ollama: {
    check:               ()      => ipcRenderer.invoke('ollama:check'),
    pull:                (model) => ipcRenderer.invoke('ollama:pull', model),
    generate:            (params)=> ipcRenderer.invoke('ollama:generate', params),
    onStream:            (cb)    => ipcRenderer.on('ollama:stream-token', (_e, t) => cb(t)),
    onPullProgress:      (cb)    => ipcRenderer.on('ollama:pull-progress', (_e, d) => cb(d)),
    removeStreamListeners: ()    => ipcRenderer.removeAllListeners('ollama:stream-token'),
    removePullListeners:   ()    => ipcRenderer.removeAllListeners('ollama:pull-progress')
  },
  google: {
    authUrl:      ()     => ipcRenderer.invoke('google:auth-url'),
    exchangeCode: (code) => ipcRenderer.invoke('google:exchange-code', code),
    openBrowser:  (url)  => ipcRenderer.invoke('shell:open', url)
  },
  shell: { open: (url) => ipcRenderer.invoke('shell:open', url) },
  win: {
    minimize: () => ipcRenderer.send('win-minimize'),
    maximize: () => ipcRenderer.send('win-maximize'),
    close:    () => ipcRenderer.send('win-close')
  }
})