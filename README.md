# OpenPlanner

> Free. Open source. Runs entirely on your machine.

OpenPlanner is a free, local desktop application for planning and scheduling short-form video content across **TikTok**, **YouTube Shorts**, and **Instagram Reels**. No subscriptions, no cloud sync, no paywalls — ever. Your content, your data, your device.

Built as a direct response to the wave of overpriced, data-hungry social media scheduling tools, OpenPlanner gives creators and small businesses a powerful alternative that respects their privacy and their wallet.

---

## Features

- **Video Upload & Scheduling** — Drag and drop your video, fill in the details, and schedule it across one or multiple platforms in a single flow.
- **Visual Content Calendar** — A full monthly calendar view with color-coded post pills. Click any day to view or schedule posts. Toggle between calendar and list view.
- **Multi-Platform Support** — TikTok, YouTube Shorts, and Instagram Reels. Connect each platform with your credentials and manage scheduled posts per platform.
- **Hashtag System** — Add hashtags manually, save them as reusable presets, and apply presets across future posts.
- **Timezone Wheel** — Set your posting timezone with a native scroll wheel — no dropdowns.
- **Google Calendar Sync** — Optionally sync scheduled posts directly to Google Calendar with color coding and automatic reminders.
- **Four Built-in Themes** — Midnight, Nord, Sakura, and Slate. Switch anytime from Settings.
- **100% Local Storage** — All posts, credentials, and settings are stored on your device via electron-store. Nothing is ever sent to an external database.

---

## Screenshots

> Coming soon — contributions welcome!

---

## Tech Stack

| Layer | Technology |
|---|---|
| Desktop Framework | [Electron.js](https://electronjs.org) |
| UI | Vanilla HTML, CSS, JavaScript |
| Local Storage | [electron-store](https://github.com/sindresorhus/electron-store) |
| Calendar Integration | Google Calendar API (OAuth 2.0) |
| Installer | electron-builder → `.exe` (Windows) / `.dmg` (macOS) |
| License | MIT |

---

## Requirements

### Windows
- Windows 10 or Windows 11 (64-bit)
- [Node.js](https://nodejs.org/) v18 or later

### macOS
- macOS 11 (Big Sur) or later
- Intel or Apple Silicon (M1/M2/M3) — both supported natively
- [Node.js](https://nodejs.org/) v18 or later

---

## Getting Started

```bash
git clone https://github.com/capo-dev/openplanner.git
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

## Connecting Platforms

### Instagram Reels
Create a Meta Developer app and generate a long-lived access token via the Instagram Graph API. [Setup guide →](https://developers.facebook.com/docs/instagram-api/getting-started)

### YouTube Shorts
Create a Google Cloud project, enable the YouTube Data API v3, and generate an OAuth 2.0 Client ID. [Setup guide →](https://developers.google.com/youtube/v3/getting-started)

### TikTok
TikTok's Content Posting API requires official app review before auto-posting is enabled. Start early — approval can take several weeks. [Setup guide →](https://developers.tiktok.com/products/content-posting-api)

---

## Project Structure

```
openplanner/
├── main.js          # Electron main process (window, IPC, Google OAuth)
├── preload.js       # Context bridge (exposes APIs to renderer)
├── index.html       # Entire app UI, styles, and logic
├── package.json     # Build config and dependencies (Windows + macOS)
└── assets/
    ├── icon.ico     # Windows app icon (256x256)
    ├── icon.icns    # macOS app icon
    ├── entitlements.mac.plist
    └── dmg-background.png
```

---

## Contributing

OpenPlanner is open source and all contributions are welcome.

1. Fork this repo
2. Create a branch: `git checkout -b feature/your-feature-name`
3. Commit your changes: `git commit -m 'Add your feature'`
4. Push to your fork: `git push origin feature/your-feature-name`
5. Open a Pull Request

Please open an issue first for any large changes so we can align on approach before you build.

---

## Roadmap

- [ ] Batch calendar generation — full weekly/monthly content plans
- [ ] Export to CSV / Markdown

---

## License

MIT — free to use, modify, and distribute. See [LICENSE](./LICENSE) for details.

---

> OpenPlanner is an independent open-source project. It is not affiliated with Google, Instagram, TikTok, or YouTube.
