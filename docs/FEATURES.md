# Feature matrix — where OmniDL stands against its references

References studied 2026-11-09: IDM 6.43 (paid, Windows-only, ~US$24.95
lifetime), SwiftDL (MIT, Rust+Tauri), Mister Download Manager (GPL-3.0,
Rust+egui), yt-dlp (Unlicense). OmniDL is a clean-room implementation —
see CLEANROOM.md.

| Capability | IDM (paid) | SwiftDL | Mister DM | **OmniDL** |
|---|---|---|---|---|
| Dynamic segmentation (up to 32 conn.) | ✅ | ✅ | ✅ | ✅ core, tested |
| Resume after crash/reboot (~1s loss) | ✅ | ✅ | ✅ | ✅ core, tested |
| Pre-download probe (name/size/resume) | ✅ | ✅ | ✅ | ✅ engine contract |
| Browser capture w/ cookies (ext.) | ✅ | ✅ | ✅ | ✅ extension/ |
| Video sites (yt-dlp, 1800+) | partial | ✅ | ✅ | ✅ sidecar |
| Quality picker 4K→audio, subtitles, SponsorBlock | — | ✅ | ✅ | ✅ sidecar args, tested |
| Torrents & magnets, file picker | — | ✅ | ✅ | 🚧 router + rqbit planned |
| Image galleries (gallery-dl) | — | — | ✅ | ✅ sidecar |
| HLS / DASH streams | partial | — | ✅ | ✅ parser, tested |
| File hosts via Debrid | — | — | ✅ | 🚧 router done, API next |
| Site Grabber (mirror a site, filtered) | ✅ | — | ✅ (link grabber) | 🚧 roadmap |
| Queues & scheduler, wake/shutdown | ✅ | partial | ✅ | 🚧 roadmap |
| Speed limit per-task & global, on schedule | ✅ | ✅ | ✅ | ✅ limiter core |
| Auto categories & folders | ✅ | ✅ | ✅ | ✅ core, tested |
| Link grabber (batch links from a page) | ✅ | — | ✅ | 🚧 roadmap |
| Archive auto-extract (zip/7z/rar) | partial | — | ✅ | 🚧 roadmap |
| Proxy, cookies, auth, custom UA/referrer | ✅ | ✅ | ✅ | 🚧 engine next |
| VPN binding + kill switch | — | — | ✅ | 🚧 roadmap |
| Phone remote / headless NAS mode | — | — | ✅ | 🚧 roadmap |
| CLI with IDM switches + local API / MCP | ✅ CLI | ✅ CLI | ✅ | ✅ CLI shipped |
| Windows | ✅ | ✅ | ✅ | ✅ Tauri |
| macOS | ❌ | ✅ | ✅ (exp.) | ✅ Tauri |
| Linux | ❌ | ✅ | ✅ | ✅ Tauri |
| **Android** | ❌ | ❌ | ❌ | ✅ **Tauri Mobile** |
| **iOS** | ❌ | ❌ | ❌ | ✅ **Tauri Mobile** |
| Price | paid | free | free | **free, MIT** |
| Ads / account / telemetry | reg. nag | none | none | **none** |

✅ shipped in this repo · 🚧 designed, on the roadmap · — not offered

## "Any source" — the router in one view

Paste anything; OmniDL picks the engine and tells you which:

- Direct file (HTTP/HTTPS) → segmented engine
- FTP / FTPS → FTP engine
- `magnet:` / `.torrent` → BitTorrent
- `.m3u8` → HLS, `.mpd` → DASH (segments → ffmpeg merge)
- YouTube, Bilibili, TikTok, X, Instagram, Twitch, Vimeo, SoundCloud,
  1800+ more → yt-dlp
- Pixiv and other galleries → gallery-dl
- File-host links → Debrid unrestrict → segmented engine
- A page full of links → Link grabber → batch queue
