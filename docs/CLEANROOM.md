# Clean-room & licensing statement

OmniDL is an **independent implementation**. It was designed from the
public documentation, help pages, screenshots and feature lists of
existing download managers (IDM, SwiftDL, Mister Download Manager) and
from public protocol standards (HTTP RFC 9110/9112, HLS RFC 8216,
BitTorrent BEPs).

- **No code was copied** from Mister Download Manager (GPL-3.0) — its
  feature set informed requirements only. Copying GPL code would have
  forced OmniDL to be GPL; OmniDL is MIT instead.
- No code was copied from SwiftDL either (MIT would have allowed it);
  the engine in `crates/` is written from scratch so the whole codebase
  has one clear provenance.
- IDM is closed-source shareware: nothing of it was decompiled,
  disassembled or bypassed. "IDM-compatible" means only that OmniDL
  accepts IDM's documented command-line switches and keeps the
  workflows users know (categories, queues, per-download window).
  "Internet Download Manager" and "IDM" are trademarks of Tonec Inc.,
  named here only for comparison; OmniDL is not affiliated with Tonec.
- **yt-dlp** (Unlicense), **gallery-dl** and **ffmpeg** are *external
  programs* that OmniDL downloads on first use (or uses from the
  system) and runs as separate processes — the same arrangement used
  by SwiftDL and Mister Download Manager. They are not vendored into
  this repository and keep their own licences.
- BitTorrent support is planned on **librqbit** (Apache-2.0) as a
  separate crate.

Only download content you have the right to download. DRM-protected
streams (Netflix and similar) are out of scope and will not be
supported.
