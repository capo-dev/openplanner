# OpenPlanner

> Free. Open source. Runs entirely on your machine.

OpenPlanner is a free, local desktop application for planning and scheduling short-form video content for **YouTube Shorts** and **Instagram Reels**. Available on Windows and macOS. No subscriptions, no cloud sync, no paywalls — ever. Your content, your data, your device.

Built as a direct response to the wave of overpriced, data-hungry social media scheduling tools, OpenPlanner gives creators and small businesses a powerful alternative that respects their privacy and their wallet.

---

## Features

- **AI Content Generation** — Powered by [Ollama](https://ollama.com) running locally on your device. Generate platform-optimized scripts, hooks, captions, hashtags, and posting schedules using Llama 3. Zero API costs, zero data leaving your machine.
- **Visual Content Calendar** — A full monthly calendar view with color-coded post pills. Purple for Reels, red for Shorts, amber for both. Click any day to instantly schedule a post.
- **Google Calendar Sync** — Optionally sync scheduled posts directly to Google Calendar with color coding, caption notes, and automatic 24hr + 1hr reminders.
- **Four Built-in Themes** — Midnight, Nord, Sakura, and Slate. Switch anytime from Settings.
- **100% Local Storage** — All posts, settings, and credentials are stored on your device via electron-store. Nothing is ever sent to an external database.
- **Built-in Setup Wizard** — Detects Ollama on first launch, walks through the Llama 3 download, and gets you generating content in minutes.
- **Native Apple Silicon Support** — Builds natively for M1, M2, and M3 Macs. No Rosetta required.

---

## Screenshots

> Coming soon — contributions welcome!

---

## Tech Stack

| Layer | Technology |
|---|---|
| Desktop Framework | [Electron.js](https://electronjs.org) |
| UI | Vanilla HTML, CSS, JavaScript |
| AI Engine | [Ollama](https://ollama.com) + Llama 3 (local) |
| Local Storage | [electron-store](https://github.com/sindresorhus/electron-store) |
| Calendar Integration | Google Calendar API (OAuth 2.0) |
| Installer | electron-builder → `.exe` (Windows) / `.dmg` (macOS) |
| License | MIT |

---

## Requirements

### Windows
- Windows 10 or Windows 11 (64-bit)
- [Node.js](https://nodejs.org/) v18 or later
- [Ollama](https://ollama.com/download) installed and running

### macOS
- macOS 11 (Big Sur) or later
- Intel or Apple Silicon (M1/M2/M3) — both supported natively
- [Node.js](https://nodejs.org/) v18 or later
- [Ollama](https://ollama.com/download) installed and running

---

## Getting Started

```bash
git clone https://github.com/capo-dev/openplanner
cd openplanner
npm install
npm start
```

---

## Build

```bash
# Windows — produces .exe installer
npm run build:win

# macOS — produces .dmg for Intel + Apple Silicon
npm run build:mac

# Both platforms at once
npm run build:all

# Windows portable (no install required)
npm run build:portable
```

> ⚠️ Windows builds must be run on a Windows machine.
> ⚠️ macOS builds must be run on a macOS machine.

### Build outputs

| Command | Output | Platform |
|---|---|---|
| `build:win` | `dist/OpenPlanner Setup 1.0.0.exe` | Windows |
| `build:portable` | `dist/OpenPlanner 1.0.0.exe` | Windows (portable) |
| `build:mac` | `dist/OpenPlanner-1.0.0.dmg` | macOS Intel |
| `build:mac` | `dist/OpenPlanner-1.0.0-arm64.dmg` | macOS Apple Silicon |

---

## AI Setup — Ollama + Llama 3

OpenPlanner runs AI 100% locally using Ollama. No API keys required.

1. Download and install Ollama from [ollama.com/download](https://ollama.com/download)
2. Launch OpenPlanner — the built-in Setup Wizard will detect Ollama and offer to download Llama 3 automatically
3. Or pull it manually via terminal:

```bash
ollama pull llama3
```

The green dot in the sidebar confirms Ollama is running and ready.

### Supported models

| Model | Size | Notes |
|---|---|---|
| `llama3` | ~4.7 GB | Default — best quality |
| `mistral` | ~4.1 GB | Great balance of speed and quality |
| `phi3` | ~2.3 GB | Lightweight, faster on older hardware |

---

## Google Calendar Setup (Optional)

1. Go to [console.cloud.google.com](https://console.cloud.google.com)
2. Create a project and enable the **Google Calendar API**
3. Go to **Credentials → Create Credentials → OAuth 2.0 Client ID**
4. Select **Desktop App** as the application type
5. Copy your **Client ID** and **Client Secret**
6. Open OpenPlanner → **Settings → Google Calendar**
7. Paste in your credentials and click **Connect Google Calendar**
8. Complete the browser sign-in and paste the authorization code back into the app

---

## Project Structure

```
openplanner/
├── main.js                        # Electron main process (window, IPC, Ollama, Google OAuth)
├── preload.js                     # Context bridge (exposes APIs to renderer)
├── index.html                     # Entire app UI, styles, and logic
├── package.json                   # Build config and dependencies (Windows + macOS)
└── assets/
    ├── icon.ico                   # Windows app icon (256x256)
    ├── icon.icns                  # macOS app icon
    ├── entitlements.mac.plist     # macOS security entitlements
    └── dmg-background.png         # Optional: 540×380px DMG background
```

> 💡 Need an `.icns` file for macOS? Convert any `.png` at [cloudconvert.com/png-to-icns](https://cloudconvert.com/png-to-icns)

---

## Contributing

OpenPlanner is open source and all contributions are welcome — bug fixes, new themes, feature suggestions, translations, or documentation improvements.

1. Fork this repo
2. Create a branch: `git checkout -b feature/your-feature-name`
3. Commit your changes: `git commit -m 'Add your feature'`
4. Push to your fork: `git push origin feature/your-feature-name`
5. Open a Pull Request

Please open an issue first for any large changes so we can align on the approach before you build.

---

## Roadmap

- [ ] Batch calendar generation — full weekly/monthly content plans
- [ ] Export to CSV / Markdown

---

## License

MIT — free to use, modify, and distribute. See [LICENSE](./LICENSE) for details.

---

> OpenPlanner is an independent open-source project. It is not affiliated with Anthropic, Google, Instagram, or YouTube.
