//! OmniDL core library.
//!
//! Design notes (clean-room, see docs/CLEANROOM.md):
//! - The segmentation planner implements *dynamic file segmentation* as
//!   publicly documented for IDM-class download managers: split a file into
//!   ranges, and when a connection finishes early it steals half of the
//!   largest remaining range so no connection idles near the end.
//! - Resume state is a list of *missing ranges*, persisted by the caller
//!   about once per second, so a crash loses at most ~1s of progress.
//! - The router decides which backend handles a URL: segmented HTTP,
//!   yt-dlp sidecar (1800+ video/audio sites), gallery-dl sidecar,
//!   BitTorrent, HLS/DASH, FTP, Debrid unrestrict, or the site grabber.

pub mod category;
pub mod limiter;
pub mod resume;
pub mod router;
pub mod segment;

pub use category::{categorise, Category};
pub use limiter::TokenBucket;
pub use resume::ResumeState;
pub use router::{route, Backend, Route};
pub use segment::{Range, SegmentPlanner};
