# OmniDL Architecture

One Rust core, one UI, five platforms.

```
            ┌────────────────────────────────────────────┐
            │  UI (apps/desktop/ui — HTML/CSS/JS)        │
            │  sidebar · add bar · cards · sheets        │
            └───────────────┬────────────────────────────┘
                            │ Tauri commands / events
            ┌───────────────┴────────────────────────────┐
            │  App (Tauri 2)                             │
            │  Windows · macOS · Linux · Android · iOS   │
            │  tray · native messaging · scheduler       │
            └───────────────┬────────────────────────────┘
                            │
        ┌───────────────────┼─────────────────────────┐
        │                   │                         │
┌───────┴────────┐  ┌───────┴─────────┐      ┌────────┴─────────┐
│ omnidl-core    │  │ omnidl-engine   │      │ sidecars         │
│ router         │  │ HTTP segments   │      │ yt-dlp (video)   │
│ segment plan   │  │ HLS/DASH        │      │ gallery-dl (img) │
│ resume state   │  │ FTP             │      │ ffmpeg (merge)   │
│ token bucket   │  │ torrent (rqbit) │      │                  │
│ categories     │  │ debrid unrestrict│     └──────────────────┘
└────────────────┘  └─────────────────┘
        ▲
        │ Native Messaging
┌───────┴────────┐
│ Browser ext.   │  Chrome · Edge · Brave · Vivaldi · Opera · Firefox
└────────────────┘
```

## Why Tauri 2

- SwiftDL proved Rust + Tauri for desktop download managers (small,
  fast, native tray/windows).
- Tauri 2 is the only stack here that also targets **Android and iOS**
  from the same codebase and the same web UI, so all five platforms
  stay one product instead of five ports.
- The core crates are pure Rust with no UI dependencies: the CLI, a
  future headless/NAS mode and an MCP server reuse them unchanged.

## Download flow (segmented HTTP)

1. **Probe** — one ranged request reveals real filename, size, ETag and
   whether the server accepts ranges. The user sees this *before*
   confirming (IDM's "Download File Info" habit, kept).
2. **Pre-allocate** — the final file is created at full size; every
   connection writes its own range directly. No joining step.
3. **Split & steal** — `SegmentPlanner` divides the file; a connection
   that finishes early steals half of the largest remaining range
   (`steal_largest`), so the last bytes never crawl on one connection.
4. **Persist** — the missing-ranges list is written ~1×/second to a
   `.omnidl` sidecar file; pause, crash and reboot all resume from it.
   ETag/Last-Modified validators guard against the file changing.
5. **Limit** — one token bucket per download plus one global bucket
   implement per-task and global speed limits (token bucket lives in
   core so limits mean the same in CLI, app and headless modes).

## Source routing

`omnidl_core::route` maps any input to a backend: segmented HTTP, FTP,
BitTorrent (magnet/.torrent), HLS, DASH, yt-dlp (1800+ video/audio
sites), gallery-dl (galleries), Debrid unrestrict (file hosts), or the
site grabber. The UI shows the chosen route in plain words before the
download starts — no surprises about *how* something will be fetched.

## Scheduling, queues, remote

- Queues with start/stop windows, "download overnight" schedules, and
  post-completion actions (open folder, extract archive, sleep/shutdown
  the machine) live in the app layer, driven by the same engine events.
- A local REST API + paired-phone web UI (same model as Mister Download
  Manager's remote) and an MCP server for AI assistants are planned
  crates on top of `omnidl-core`; see ROADMAP.
