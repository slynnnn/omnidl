# OmniDL on Android & iOS

OmniDL uses **Tauri 2 Mobile**: the same Rust core and the same UI in
`apps/desktop/ui` ship to phones. The UI is responsive — under 760px
it switches to a single column with a bottom tab bar and large touch
targets (try it by narrowing the window).

## Generate the native projects

```bash
cd apps/desktop
cargo install tauri-cli --version "^2" --locked
cargo tauri android init   # needs Android Studio + NDK
cargo tauri ios init       # needs macOS + Xcode
cargo tauri android dev
cargo tauri ios dev
```

## Mobile download behaviour

- Android: downloads run in a foreground service so they survive the
  app being swiped to the background; files land in `Downloads/OmniDL`
  via the MediaStore / SAF picker.
- iOS: background `URLSession`-style transfers are constrained by the
  OS; OmniDL queues large jobs and resumes them on next launch via the
  same `.omnidl` resume state (this is exactly what the resume format
  in `omnidl-core` was designed for).
- Share sheet: "Share → OmniDL" on both platforms adds the shared URL
  to the queue (same entry point as the browser extension on desktop).

CI (`.github/workflows/build.yml`) builds an Android APK/AAB and an
iOS app bundle (unsigned; signing is a release-time secret) on every
tag.
