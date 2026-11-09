//! Dynamic segmentation planner.
//!
//! A file of known size is divided into `n` byte ranges. Each range is
//! fetched with its own HTTP Range connection and written directly into a
//! pre-allocated final file (no joining step). When a worker finishes its
//! range it *steals* the tail half of the largest in-flight range.

use std::fmt;

/// Half-open byte range `[start, end)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

impl Range {
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
    pub fn len(&self) -> u64 {
        self.end.saturating_sub(self.start)
    }
    pub fn is_empty(&self) -> bool {
        self.end <= self.start
    }
    /// Split off the tail half: `self` keeps the head, returns the tail.
    pub fn split_half(&mut self) -> Option<Range> {
        let len = self.len();
        if len < 2 {
            return None;
        }
        let mid = self.start + len / 2;
        let tail = Range::new(mid, self.end);
        self.end = mid;
        Some(tail)
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end.saturating_sub(1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeState {
    Pending,
    Active,
    Done,
}

#[derive(Debug, Clone)]
struct Slot {
    range: Range,
    downloaded: u64,
    state: RangeState,
}

/// Plans and rebalances download segments for one file.
#[derive(Debug, Clone)]
pub struct SegmentPlanner {
    total: u64,
    slots: Vec<Slot>,
}

impl SegmentPlanner {
    /// Divide `total` bytes into up to `connections` ranges (1..=32,
    /// clamped like IDM/SwiftDL-class tools). Files smaller than the
    /// connection count get fewer ranges; unknown/zero size gets one.
    pub fn new(total: u64, connections: u32) -> Self {
        let n = connections.clamp(1, 32) as u64;
        if total == 0 {
            return Self {
                total,
                slots: vec![Slot {
                    range: Range::new(0, 0),
                    downloaded: 0,
                    state: RangeState::Pending,
                }],
            };
        }
        let n = n.min(total).max(1);
        let base = total / n;
        let mut slots = Vec::with_capacity(n as usize);
        for i in 0..n {
            let start = i * base;
            let end = if i == n - 1 { total } else { start + base };
            slots.push(Slot {
                range: Range::new(start, end),
                downloaded: 0,
                state: RangeState::Pending,
            });
        }
        Self { total, slots }
    }

    pub fn total(&self) -> u64 {
        self.total
    }

    /// Hand out the next pending range, marking it active.
    pub fn next_pending(&mut self) -> Option<Range> {
        let slot = self
            .slots
            .iter_mut()
            .find(|s| s.state == RangeState::Pending)?;
        slot.state = RangeState::Active;
        Some(slot.range)
    }

    /// Record progress inside `range` (absolute byte offset reached).
    pub fn record_progress(&mut self, range: Range, reached: u64) {
        if let Some(s) = self.slots.iter_mut().find(|s| s.range.start == range.start) {
            s.downloaded = reached.saturating_sub(s.range.start).min(s.range.len());
            if reached >= s.range.end {
                s.state = RangeState::Done;
            }
        }
    }

    pub fn mark_done(&mut self, range: Range) {
        if let Some(s) = self.slots.iter_mut().find(|s| s.range.start == range.start) {
            s.downloaded = s.range.len();
            s.state = RangeState::Done;
        }
    }

    /// Dynamic rebalancing: a worker that finished early steals half of
    /// the largest *remaining* portion of any active range. Returns the
    /// stolen tail range, already marked active.
    pub fn steal_largest(&mut self) -> Option<Range> {
        // Find active slot with the most bytes still missing.
        let mut best: Option<usize> = None;
        let mut best_missing = 0u64;
        for (i, s) in self.slots.iter().enumerate() {
            if s.state == RangeState::Active {
                let frontier = s.range.start + s.downloaded;
                let missing = s.range.end.saturating_sub(frontier);
                if missing > best_missing {
                    best_missing = missing;
                    best = Some(i);
                }
            }
        }
        let idx = best?;
        if best_missing < 2 {
            return None;
        }
        let s = &mut self.slots[idx];
        let frontier = s.range.start + s.downloaded;
        let mut remaining = Range::new(frontier, s.range.end);
        // Shrink the victim to its already-covered head plus nothing more:
        // the victim keeps [start, frontier + half), thief takes the rest.
        let tail = remaining.split_half()?;
        s.range.end = tail.start;
        let stolen = Slot {
            range: tail,
            downloaded: 0,
            state: RangeState::Active,
        };
        self.slots.push(stolen);
        Some(tail)
    }

    pub fn downloaded(&self) -> u64 {
        self.slots.iter().map(|s| s.downloaded).sum()
    }

    pub fn is_complete(&self) -> bool {
        self.total > 0 && self.downloaded() >= self.total
    }

    /// The still-missing ranges — exactly what gets persisted for resume.
    pub fn missing_ranges(&self) -> Vec<Range> {
        let mut out = Vec::new();
        for s in &self.slots {
            if s.state != RangeState::Done {
                let from = s.range.start + s.downloaded;
                if from < s.range.end {
                    out.push(Range::new(from, s.range.end));
                }
            }
        }
        out.sort();
        out
    }

    /// Per-connection view for the UI: (range, downloaded, state).
    pub fn connections(&self) -> Vec<(Range, u64, RangeState)> {
        self.slots
            .iter()
            .map(|s| (s.range, s.downloaded, s.state))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_evenly_and_covers_file() {
        let p = SegmentPlanner::new(1000, 8);
        let conns = p.connections();
        assert_eq!(conns.len(), 8);
        assert_eq!(conns[0].0, Range::new(0, 125));
        assert_eq!(conns[7].0.end, 1000);
        let covered: u64 = conns.iter().map(|c| c.0.len()).sum();
        assert_eq!(covered, 1000);
    }

    #[test]
    fn clamps_connections_to_32() {
        let p = SegmentPlanner::new(1_000_000, 999);
        assert_eq!(p.connections().len(), 32);
    }

    #[test]
    fn steal_takes_half_of_largest_remaining() {
        let mut p = SegmentPlanner::new(100, 2); // [0,50) [50,100)
        let a = p.next_pending().unwrap();
        let b = p.next_pending().unwrap();
        p.record_progress(a, 10); // a missing 40
        p.record_progress(b, 90); // b missing 10
        let stolen = p.steal_largest().unwrap();
        // largest remaining is a: frontier 10..50, half tail = 30..50
        assert_eq!(stolen, Range::new(30, 50));
    }

    #[test]
    fn resume_missing_ranges() {
        let mut p = SegmentPlanner::new(100, 2);
        let a = p.next_pending().unwrap();
        p.record_progress(a, 50);
        assert_eq!(p.missing_ranges(), vec![Range::new(50, 100)]);
        assert!(!p.is_complete());
        let b = p.next_pending().unwrap();
        p.mark_done(b);
        assert!(p.is_complete());
    }
}
