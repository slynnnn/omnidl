# OmniDL

### Every download, at full speed, from any source.

The free, open-source, all-source download manager for
**Windows · macOS · Linux · Android · iOS** — one codebase, MIT licensed,
no ads, no account, no telemetry.

[Features](#features) · [Download](#download) · [How it works](#how-it-works) ·
[Build from source](#build-from-source) · [中文说明](#中文说明)

---

## Why OmniDL

Browsers download through **one connection**, and many servers cap each
connection. Paid tools like IDM fix that on Windows only. OmniDL takes
the best ideas of **IDM** (dynamic segmentation, queues, scheduler,
categories), **SwiftDL** (Rust + Tauri, per-connection view, browser
extension), **Mister Download Manager** (torrents, galleries, debrid,
remote control) and **yt-dlp** (1800+ video sites) — and ships them as
one clean-room, free app on **all five platforms, phones included**.

Paste *anything* — OmniDL routes it to the right engine and tells you
which, in plain words, before anything starts:

| You paste | OmniDL uses |
|---|---|
| A direct file link | Segmented HTTP engine (up to 32 connections) |
| YouTube / Bilibili / TikTok / X / Twitch / 1800+ sites | yt-dlp, quality picker 4K → audio-only |
| `magnet:` or a `.torrent` | BitTorrent, pick the files you want |
| An `.m3u8` / `.mpd` stream | HLS/DASH segment engine + ffmpeg merge |
| A Pixiv / image gallery page | gallery-dl |
| A file-host link | Debrid unrestrict → segmented engine |
| An FTP address | FTP engine |
| A page full of links | Link grabber → batch queue |

## Features

- **Fast by design** — dynamic segmentation: when a connection finishes
  its part it steals half of the largest remaining part, so nothing
  idles near the end. Files are pre-allocated and written in place —
  no joining step.
- **Resume anything** — progress (the missing-ranges list) is saved
  every second; crash, reboot or dropped Wi-Fi costs ~1 second, and
  ETag validators stop you resuming against a changed file.
- **Know before you download** — a probe shows the real filename, size
  and resumability first; pick folder, category, quality, connections,
  or *Download later*.
- **Browser extension** — Chrome, Edge, Brave, Vivaldi, Opera and
  Firefox hand downloads to OmniDL *with your cookies*, add a
  right-click action and a **Download video** button on players.
  Hold `Alt` to keep one download in the browser.
- **Queues, scheduler, limits** — concurrent-download caps, per-task
  and global speed limits (token bucket), overnight schedules.
- **Tidy by default** — automatic categories: Video, Music, Programs,
  Compressed, Documents, Images, Torrents.
- **CLI included** — `omnidl <url>`, and it accepts IDM-style switches
  (`/d /p /f /n /q /s /m`) so existing scripts keep working.
- **Minimal, familiar UI** — left categories, one add bar, progress
  cards with a per-connection segment strip, light/dark following the
  system; on phones the same UI becomes a single column with a bottom
  tab bar. `Ctrl K` searches everything.

Status: **v0.1** — core engine, router, CLI, UI and extension are in
this repo with tests; see [docs/ROADMAP.md](docs/ROADMAP.md) for what
lands next and [docs/FEATURES.md](docs/FEATURES.md) for the full
comparison against IDM / SwiftDL / Mister Download Manager.

## Download

Grab installers from the [Releases](https://github.com/slynnnn/omnidl/releases)
page (built by GitHub Actions for every tag):

| Platform | Package |
|---|---|
| Windows 10/11 | `OmniDL_x.y.z_x64-setup.exe` |
| macOS (Apple silicon + Intel) | `OmniDL_x.y.z_universal.dmg` |
| Linux | `.AppImage` · `.deb` · `.rpm` |
| Android | `.apk` / Play `.aab` |
| iOS | TestFlight / unsigned `.ipa` (see `apps/mobile/README.md`) |

## How it works

1. **Probe** — one ranged request reveals name, size, resumability.
2. **Pre-allocate** — the final file is created at full size.
3. **Split** — byte ranges are fetched in parallel (HTTP Range).
4. **Steal** — idle connections take half of the largest remaining range.
5. **Persist** — missing ranges are saved ~1×/s; that *is* the resume file.

Details: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) ·
Clean-room & licences: [docs/CLEANROOM.md](docs/CLEANROOM.md)

## Build from source

```bash
git clone https://github.com/slynnnn/omnidl
cd omnidl
cargo test --workspace          # core + engine: no extra deps needed
cargo run -p omnidl-cli -- https://example.com/big.iso --route-only

# App (desktop & mobile) — needs Tauri 2 prerequisites for your OS
cargo install tauri-cli --version "^2" --locked
cd apps/desktop && cargo tauri dev
cargo tauri android init && cargo tauri android dev   # Android
cargo tauri ios init && cargo tauri ios dev           # iOS (macOS host)
```

| Path | What it is |
|---|---|
| `crates/omnidl-core` | Router, segmentation planner, resume, limiter, categories (pure std) |
| `crates/omnidl-engine` | HTTP drive contract, HLS parser, yt-dlp/gallery-dl/ffmpeg sidecars |
| `crates/omnidl-cli` | `omnidl` command-line tool (IDM-switch compatible) |
| `apps/desktop` | Tauri 2 app: `src-tauri` (Rust) + `ui` (the interface) |
| `apps/mobile` | Android/iOS notes — same app via Tauri Mobile |
| `extension` | Browser extension (Manifest V3) |
| `tools` | Throttled benchmark server |
| `docs` | Architecture, feature matrix, roadmap, clean-room statement |

## 中文说明

OmniDL 是一个免费开源的**全能来源下载器**：把 IDM（付费版）的动态分段、
队列调度、自动分类，SwiftDL 的 Rust+Tauri 架构与浏览器扩展，
Mister Download Manager 的种子/图库/Debrid/远程控制，以及 yt-dlp 的
1800+ 视频站支持，整合成一个干净室实现的五端应用
（Windows / macOS / Linux / Android / iOS，同一套代码）。

任何链接粘贴进来即可：直链走多线程分段（最高 32 连接、动态抢片、
每秒存进度、断点续传），视频站走 yt-dlp（4K 到仅音频、字幕、
SponsorBlock），磁力/种子走 BitTorrent，m3u8/mpd 走流媒体引擎，
图库走 gallery-dl，网盘走 Debrid 解锁后分段下载。界面简约大气：
左侧分类、一根添加栏、带分段条的进度卡，手机上自动变单列＋底部导航，
完全贴合 IDM 用户的使用习惯，但免费、无广告、无账号。

## Contributing

Issues and PRs welcome. Please run `cargo test --workspace` first.

## License

MIT. yt-dlp, gallery-dl and ffmpeg are external programs run as
separate processes and keep their own licences — see
[docs/CLEANROOM.md](docs/CLEANROOM.md). Only download content you have
the right to download; DRM-protected streams are out of scope.
