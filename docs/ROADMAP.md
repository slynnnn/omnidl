# Roadmap

## v0.1 — this release
- [x] Core: source router, dynamic segmentation planner with stealing,
      resume state with validators, token-bucket limiter, categories
      (13 unit tests, `cargo test --workspace`)
- [x] Engine: HLS playlist parser, yt-dlp/gallery-dl/ffmpeg command
      builders, segmented-drive contract with transport trait
- [x] CLI with IDM-style switches
- [x] App shell (Tauri 2) + full UI (desktop & mobile layouts)
- [x] Browser extension (MV3): capture, context menu, video button
- [x] CI matrix: Windows / macOS / Linux / Android / iOS

## v0.2 — make it download for real, everywhere
- [ ] tokio+reqwest transport (rustls), proxy/cookies/auth headers
- [ ] Wire app commands to the engine; progress events to the UI
- [ ] yt-dlp/ffmpeg auto-download on first use + self-update channel
- [ ] Android: Tauri Mobile build signed in CI; iOS: build + TestFlight notes
- [ ] FTP engine, HLS download + merge

## v0.3 — the IDM-class comforts
- [ ] Queues & scheduler (start windows, overnight mode, wake, shutdown/sleep after)
- [ ] Per-download window with per-connection graph (SwiftDL-style)
- [ ] Link grabber + Site Grabber (filtered mirror)
- [ ] Archive auto-extract, checksum verify, virus-scan hook
- [ ] Speed-limit schedules (e.g. cap while the user is active)

## v0.4 — torrents & hosts
- [ ] librqbit integration: file picker, sequential mode, RSS, share-ratio limits
- [ ] Debrid providers (Real-Debrid, AllDebrid, Premiumize) via official APIs
- [ ] VPN interface binding + kill switch

## v0.5 — beyond the desktop
- [ ] Phone remote (paired, LAN) & headless mode for NAS/Docker
- [ ] MCP server + local REST API for scripts and AI assistants
- [ ] i18n (start: zh-CN, zh-TW, en, ja; RTL support after)
