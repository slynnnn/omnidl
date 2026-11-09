//! Minimal HLS (m3u8) parsing: master playlists pick the highest
//! bandwidth variant; media playlists yield ordered segment URLs.

#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    pub bandwidth: u64,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MediaPlaylist {
    pub segments: Vec<String>,
    pub is_live: bool,
    pub target_duration: f64,
}

fn resolve(base: &str, rel: &str) -> String {
    if rel.starts_with("http://") || rel.starts_with("https://") {
        return rel.to_string();
    }
    if let Some(prefix) = base.rsplit('/').next().map(|_| &base[..base.rfind('/').unwrap() + 1]) {
        if rel.starts_with('/') {
            // Absolute path: keep scheme + host from base.
            if let Some(host_end) = base[base.find("://").map(|i| i + 3).unwrap_or(0)..]
                .find('/')
            {
                let root = &base[..base.find("://").map(|i| i + 3).unwrap_or(0) + host_end];
                return format!("{root}{rel}");
            }
        }
        return format!("{prefix}{rel}");
    }
    rel.to_string()
}

pub fn parse_master(text: &str, base_url: &str) -> Vec<Variant> {
    let mut out = Vec::new();
    let mut pending_bw: Option<u64> = None;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with("#EXT-X-STREAM-INF:") {
            pending_bw = line
                .split("BANDWIDTH=")
                .nth(1)
                .and_then(|v| v.split(',').next())
                .and_then(|v| v.trim().parse().ok());
        } else if !line.is_empty() && !line.starts_with('#') {
            if let Some(bw) = pending_bw.take() {
                out.push(Variant {
                    bandwidth: bw,
                    url: resolve(base_url, line),
                });
            }
        }
    }
    out.sort_by_key(|v| v.bandwidth);
    out
}

pub fn best_variant(text: &str, base_url: &str) -> Option<Variant> {
    parse_master(text, base_url).into_iter().last()
}

pub fn parse_media(text: &str, base_url: &str) -> MediaPlaylist {
    let mut segments = Vec::new();
    let mut target = 0.0;
    for line in text.lines() {
        let line = line.trim();
        if let Some(v) = line.strip_prefix("#EXT-X-TARGETDURATION:") {
            target = v.trim().parse().unwrap_or(0.0);
        } else if !line.is_empty() && !line.starts_with('#') {
            segments.push(resolve(base_url, line));
        }
    }
    MediaPlaylist {
        is_live: !text.contains("#EXT-X-ENDLIST"),
        segments,
        target_duration: target,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MASTER: &str = "#EXTM3U\n#EXT-X-STREAM-INF:BANDWIDTH=800000,RESOLUTION=640x360\nlow.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=5000000,RESOLUTION=1920x1080\nhi.m3u8\n";
    const MEDIA: &str = "#EXTM3U\n#EXT-X-TARGETDURATION:6\n#EXTINF:6.0,\nseg1.ts\n#EXTINF:6.0,\nseg2.ts\n#EXT-X-ENDLIST\n";

    #[test]
    fn picks_highest_bandwidth() {
        let v = best_variant(MASTER, "https://cdn.example.com/live/master.m3u8").unwrap();
        assert_eq!(v.bandwidth, 5_000_000);
        assert_eq!(v.url, "https://cdn.example.com/live/hi.m3u8");
    }

    #[test]
    fn parses_media_segments() {
        let m = parse_media(MEDIA, "https://cdn.example.com/live/hi.m3u8");
        assert_eq!(m.segments.len(), 2);
        assert_eq!(m.segments[0], "https://cdn.example.com/live/seg1.ts");
        assert!(!m.is_live);
        assert_eq!(m.target_duration, 6.0);
    }
}
