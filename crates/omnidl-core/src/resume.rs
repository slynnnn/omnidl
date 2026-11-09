//! Resume state: the set of missing ranges plus the metadata needed to
//! trust them (URL, total size, ETag / Last-Modified validators).
//!
//! The caller persists this about once per second while downloading and
//! on pause/quit, so a crash, reboot or dropped connection resumes with
//! at most ~1 second of re-download. Serialised as a compact line-based
//! text format (no external crates): one `key=value` per line, ranges as
//! comma-separated `start-end`.

use crate::segment::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResumeState {
    pub url: String,
    pub total: u64,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub missing: Vec<Range>,
}

impl ResumeState {
    pub fn new(url: impl Into<String>, total: u64, missing: Vec<Range>) -> Self {
        Self {
            url: url.into(),
            total,
            etag: None,
            last_modified: None,
            missing,
        }
    }

    pub fn downloaded(&self) -> u64 {
        let missing: u64 = self.missing.iter().map(|r| r.len()).sum();
        self.total.saturating_sub(missing)
    }

    pub fn is_complete(&self) -> bool {
        self.total > 0 && self.missing.is_empty()
    }

    /// Validators decide whether a saved state can still be trusted: if
    /// the server's ETag changed, the file changed and resume is unsafe.
    pub fn validators_match(&self, etag: Option<&str>, last_modified: Option<&str>) -> bool {
        match (&self.etag, etag) {
            (Some(saved), Some(now)) => return saved == now,
            _ => {}
        }
        match (&self.last_modified, last_modified) {
            (Some(saved), Some(now)) => saved == now,
            _ => true, // nothing recorded: nothing to contradict
        }
    }

    pub fn to_text(&self) -> String {
        let mut s = String::new();
        s.push_str("omnidl-resume-v1\n");
        s.push_str(&format!("url={}\n", self.url));
        s.push_str(&format!("total={}\n", self.total));
        if let Some(e) = &self.etag {
            s.push_str(&format!("etag={e}\n"));
        }
        if let Some(m) = &self.last_modified {
            s.push_str(&format!("last_modified={m}\n"));
        }
        let ranges: Vec<String> = self
            .missing
            .iter()
            .map(|r| format!("{}-{}", r.start, r.end))
            .collect();
        s.push_str(&format!("missing={}\n", ranges.join(",")));
        s
    }

    pub fn from_text(text: &str) -> Option<Self> {
        let mut url = None;
        let mut total = None;
        let mut etag = None;
        let mut last_modified = None;
        let mut missing = Vec::new();
        for line in text.lines() {
            let Some((k, v)) = line.split_once('=') else {
                continue; // header line and blanks
            };
            match k {
                "url" => url = Some(v.to_string()),
                "total" => total = v.parse().ok(),
                "etag" => etag = Some(v.to_string()),
                "last_modified" => last_modified = Some(v.to_string()),
                "missing" => {
                    if !v.is_empty() {
                        for part in v.split(',') {
                            let (a, b) = part.split_once('-')?;
                            missing.push(Range::new(a.parse().ok()?, b.parse().ok()?));
                        }
                    }
                }
                _ => {}
            }
        }
        Some(Self {
            url: url?,
            total: total?,
            etag,
            last_modified,
            missing,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let mut st = ResumeState::new("https://example.com/f.iso", 1000, vec![Range::new(500, 1000)]);
        st.etag = Some("\"abc123\"".into());
        let text = st.to_text();
        let back = ResumeState::from_text(&text).unwrap();
        assert_eq!(st, back);
        assert_eq!(back.downloaded(), 500);
        assert!(back.validators_match(Some("\"abc123\""), None));
        assert!(!back.validators_match(Some("\"different\""), None));
    }
}
