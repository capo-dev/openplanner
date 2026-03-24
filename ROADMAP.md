 # OpenPlanner — Product Roadmap
> Local-first, open-source social media planning & auto-posting for YouTube Shorts, Instagram Reels, and TikTok.

---

## ✅ Phase 0 — Foundation (Complete)
> Core app shell is built and running.

- [x] Electron.js app scaffold (Windows + macOS)
- [x] Four UI themes: Midnight, Nord, Sakura, Slate
- [x] Google Calendar sync via OAuth 2.0
- [x] AI Composer (Ollama + Llama 3 local AI)
- [x] Ollama setup wizard
- [x] Instagram Reels view (connect account, scheduled posts, best times)
- [x] YouTube Shorts placeholder view
- [x] electron-store for local data persistence

---

## 🔧 Phase 1 — Complete Platform Views
> Goal: All 3 platform views fully built and consistent before wiring up any APIs.

- [ ] **YouTube Shorts view** — full UI matching Instagram Reels view
  - Connect account UI
  - Scheduled posts list
  - Best posting times display
  - Post composer (title, description, hashtags)
- [ ] **TikTok view** — full UI
  - Connect account UI
  - Scheduled posts list
  - Best posting times display
  - Post composer (caption, hashtags, sounds)
- [ ] **Unified Dashboard** — overview of all 3 platforms in one view
  - Upcoming posts across all platforms
  - Quick-compose shortcut
  - Platform health indicators (connected / disconnected)

---

## 🗓️ Phase 2 — Post Queue & Scheduler Engine
> Goal: A reliable local scheduler that queues and fires posts at the right time — platform agnostic.

- [ ] **Post Queue System**
  - Create, edit, delete queued posts
  - Store all post data locally via electron-store
  - Queue view (list + calendar toggle)
- [ ] **Scheduler Engine**
  - Background process that checks clock and triggers posts
  - Runs silently while app is open
  - Retry logic for failed posts (3 attempts, then flag)
- [ ] **Notification System**
  - Desktop notification on successful post
  - Alert on failed post with reason
  - 24hr and 1hr pre-post reminders
- [ ] **Batch Scheduling**
  - Schedule multiple posts at once
  - Suggest optimal times per platform automatically
  - Bulk upload via CSV

---

## 🔌 Phase 3 — Platform API Integrations (Auto-Posting)
> Goal: Actually publish content to each platform automatically.

### 3a — Instagram Reels (Meta Graph API)
- [ ] OAuth 2.0 authentication flow
- [ ] Upload video to Instagram container
- [ ] Publish Reel with caption + hashtags
- [ ] Handle token refresh

### 3b — YouTube Shorts (YouTube Data API v3)
- [ ] OAuth 2.0 authentication flow
- [ ] Upload video file
- [ ] Set title, description, hashtags, category
- [ ] Mark as Short (vertical, under 60s)
- [ ] Handle token refresh

### 3c — TikTok (Content Posting API)
- [ ] OAuth 2.0 authentication flow
- [ ] Upload video via TikTok Content Posting API
- [ ] Set caption + hashtags
- [ ] Handle token refresh
- [ ] Handle TikTok API approval process (requires app review)

---

## 🤖 Phase 4 — AI Composer Enhancements
> Goal: Make the AI Composer more powerful and platform-aware.

- [ ] **Per-platform script generation** (Shorts vs Reels vs TikTok)
- [ ] **Hook generator** — generate 5 hook variations per idea
- [ ] **Caption optimizer** — paste a caption, get an improved version
- [ ] **Hashtag research tool** — suggest trending + niche hashtags
- [ ] **Content repurposing** — turn one idea into posts for all 3 platforms
- [ ] **Content calendar AI** — generate a full week/month of post ideas
- [ ] **Tone selector** — Authentic / Educational / Entertaining / Inspirational

---

## 📊 Phase 5 — Analytics Dashboard
> Goal: Show basic performance data per post per platform.

- [ ] **Per-post stats** — views, likes, comments, shares, saves
- [ ] **Platform overview** — aggregate stats per channel
- [ ] **Best performing posts** — sorted by engagement
- [ ] **Best time insights** — derived from your own post history
- [ ] **Export report** — CSV or PDF export of analytics

---

## 💡 Phase 6 — Content Idea Library
> Goal: A local "Create" space inspired by Buffer's Create feature.

- [ ] **Idea board** — save raw ideas, links, notes
- [ ] **Draft system** — move ideas → drafts → scheduled → published
- [ ] **Content pillars** — organize ideas by topic/theme
- [ ] **Inspiration capture** — save a URL or note from anywhere via quick-add

---

## 🎨 Phase 7 — Polish & Quality of Life
> Goal: Make the app feel complete and delightful.

- [ ] **Onboarding flow** — guided setup for new users
- [ ] **More themes** — community-submitted themes
- [ ] **Keyboard shortcuts** — power user navigation
- [ ] **Settings overhaul** — per-platform preferences, defaults, timezone
- [ ] **Import from Buffer/Later** — migrate existing scheduled posts
- [ ] **Auto-updater** — in-app update notifications via GitHub releases

---

## 🚀 Phase 8 — Distribution & Community
> Goal: Ship it and grow an open-source community around it.

- [ ] **GitHub releases** — packaged .exe and .dmg builds
- [ ] **README overhaul** — full setup guide, screenshots, feature list
- [ ] **Contributor guide** — CONTRIBUTING.md, issue templates
- [ ] **Project website / landing page**
- [ ] **Discord or community forum**

---

## Platform API Notes
| Platform | API | Difficulty | Notes |
|---|---|---|---|
| Instagram Reels | Meta Graph API | Medium | Requires Facebook Developer App |
| YouTube Shorts | YouTube Data API v3 | Medium | Requires Google Cloud project |
| TikTok | Content Posting API | Hard | Requires TikTok app review/approval |

---

*Last updated: March 2026 — OpenPlanner is MIT licensed and open source.*
*GitHub: https://github.com/capo-dev/openplanner*