//! Segmented HTTP driver contract.
//!
//! The driver is written against the [`Transport`] trait so the planner,
//! resume and limiter logic (all in `omnidl-core`, all unit-tested) stay
//! independent of the actual socket implementation. The production
//! transport in the app uses tokio + reqwest with rustls; a std-only
//! transport is used in tests and by the CLI for plain HTTP.

use omnidl_core::{Range, ResumeState, SegmentPlanner};

/// Server probe result: what the "New download" sheet shows *before*
/// anything is downloaded (name, size, resumability).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Probe {
    pub final_url: String,
    pub filename: String,
    pub total: Option<u64>,
    pub accept_ranges: bool,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

impl Probe {
    /// A saved resume state is usable only if the server still offers
    /// ranges and its validators still match.
    pub fn resume_state_usable(&self, state: &ResumeState) -> bool {
        self.accept_ranges
            && Some(state.total) == self.total
            && state.validators_match(self.etag.as_deref(), self.last_modified.as_deref())
    }
}

/// Byte-range transport: one blocking call fetches exactly one range
/// into the pre-allocated output file at the right offset.
pub trait Transport {
    fn fetch_range(&mut self, url: &str, range: Range) -> Result<Vec<u8>, String>;
}

/// What a finished (or paused) driver run reports to the UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunReport {
    pub downloaded: u64,
    pub complete: bool,
    pub missing: Vec<Range>,
}

/// Drive a planner to completion (or until the transport errors),
/// letting idle workers steal work — the dynamic-segmentation loop.
pub fn drive<T: Transport>(
    transport: &mut T,
    url: &str,
    total: u64,
    connections: u32,
    on_segment: &mut dyn FnMut(Range, &[u8]),
) -> Result<RunReport, String> {
    let mut planner = SegmentPlanner::new(total, connections);
    loop {
        if planner.is_complete() {
            break;
        }
        let next = planner
            .next_pending()
            .or_else(|| planner.steal_largest());
        let Some(range) = next else { break };
        let data = transport.fetch_range(url, range)?;
        if data.len() as u64 != range.len() {
            return Err(format!(
                "short read on {range}: got {} of {} bytes",
                data.len(),
                range.len()
            ));
        }
        on_segment(range, &data);
        planner.record_progress(range, range.end);
    }
    Ok(RunReport {
        downloaded: planner.downloaded(),
        complete: planner.is_complete(),
        missing: planner.missing_ranges(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use omnidl_core::SegmentPlanner;

    struct FakeTransport;
    impl Transport for FakeTransport {
        fn fetch_range(&mut self, _url: &str, range: Range) -> Result<Vec<u8>, String> {
            Ok(vec![7u8; range.len() as usize])
        }
    }

    #[test]
    fn drive_downloads_everything() {
        let mut t = FakeTransport;
        let mut got = 0u64;
        let report = drive(&mut t, "https://example.com/f.bin", 1000, 8, &mut |_, d| {
            got += d.len() as u64;
        })
        .unwrap();
        assert!(report.complete);
        assert_eq!(report.downloaded, 1000);
        assert_eq!(got, 1000);
        assert!(report.missing.is_empty());
        let _ = SegmentPlanner::new(10, 2); // planner stays core-side
    }
}
