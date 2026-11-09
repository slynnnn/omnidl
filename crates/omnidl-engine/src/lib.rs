//! OmniDL engine.
//!
//! The heavy lifting is split by backend:
//! - HTTP(S): the segmented driver in [`http`] (planner lives in core).
//! - Video/audio sites: [`sidecar`] builds yt-dlp invocations. yt-dlp is
//!   run as a *separate program* (its own Unlicense terms), downloaded on
//!   first use and kept up to date — the same arrangement SwiftDL and
//!   Mister Download Manager use.
//! - Galleries: gallery-dl sidecar, same pattern.
//! - HLS/DASH: [`hls`] parses playlists; segments then go through the
//!   same queue/limiter/resume machinery and are merged with ffmpeg.
//! - BitTorrent: provided by the `omnidl-torrent` module in the app
//!   (librqbit, Apache-2.0) — magnet/.torrent parsing lives there.

pub mod hls;
pub mod http;
pub mod sidecar;
