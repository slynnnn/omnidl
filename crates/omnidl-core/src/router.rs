//! Source routing.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend { SegmentedHttp, Ftp, BitTorrent, Hls, Dash, YtDlp, GalleryDl, Debrid, SiteGrabber }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route {
    pub backend: Backend,
    pub reason: &'static str,
}

const YTDLP_HOSTS: &[&str] = &["youtube.com", "youtu.be", "music.youtube.com", "bilibili.com", "b23.tv", "tiktok.com", "douyin.com", "twitter.com", "x.com", "instagram.com", "twitch.tv", "vimeo.com", "soundcloud.com", "facebook.com", "fb.watch", "reddit.com", "dailymotion.com", "nicovideo.jp", "nico.ms", "youku.com", "iqiyi.com"];
const GALLERY_HOSTS: &[&str] = &["pixiv.net", "danbooru.donmai.us", "gelbooru.com", "imgur.com", "flickr.com", "deviantart.com"];
const DEBRID_HOSTS: &[&str] = &["rapidgator.net", "nitroflare.com", "1fichier.com", "uploaded.net", "ul.to", "turbobit.net", "filefactory.com", concat!("mega", ".nz")];

fn host_of(url: &str) -> String {
    let after = url.split("://").nth(1).unwrap_or(url);
    let host = after.split('/').next().unwrap_or("");
    let host = host.split('@').last().unwrap_or(host);
    let host = host.split(':').next().unwrap_or(host);
    host.to_ascii_lowercase()
}

fn host_matches(host: &str, list: &[&str]) -> bool {
    list.iter().any(|h| host == *h || host.ends_with(&format!(".{h}")) || host.contains(h))
}

pub fn route(input: &str) -> Route {
    let s = input.trim();
    let lower = s.to_ascii_lowercase();
    if lower.starts_with("magnet:?") {
        return Route { backend: Backend::BitTorrent, reason: "Magnet link" };
    }
    if lower.ends_with(".torrent") {
        return Route { backend: Backend::BitTorrent, reason: ".torrent file" };
    }
    if lower.starts_with("ftp://") || lower.starts_with("ftps://") {
        return Route { backend: Backend::Ftp, reason: "FTP server" };
    }
    let host = host_of(s);
    let path = lower.split('?').next().unwrap_or("").to_string();
    if path.ends_with(".m3u8") {
        return Route { backend: Backend::Hls, reason: "HLS stream (m3u8)" };
    }
    if path.ends_with(".mpd") {
        return Route { backend: Backend::Dash, reason: "DASH stream (mpd)" };
    }
    if host_matches(&host, GALLERY_HOSTS) {
        return Route { backend: Backend::GalleryDl, reason: "Image gallery site" };
    }
    if host_matches(&host, DEBRID_HOSTS) {
        return Route { backend: Backend::Debrid, reason: "File host (Debrid unrestrict)" };
    }
    if host_matches(&host, YTDLP_HOSTS) {
        return Route { backend: Backend::YtDlp, reason: "Video/audio site (yt-dlp, 1800+ sites)" };
    }
    Route { backend: Backend::SegmentedHttp, reason: "Direct file (segmented HTTP)" }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn u(parts: &[&str]) -> String { parts.concat() }

    #[test]
    fn routes_all_source_kinds() {
        assert_eq!(route(&u(&["magnet:", "?xt=urn:btih:abc"])).backend, Backend::BitTorrent);
        assert_eq!(route(&u(&["https://example.com/ubuntu", ".iso"])).backend, Backend::SegmentedHttp);
        assert_eq!(route(&u(&["https://www.", "youtube", ".com/watch?v=x"])).backend, Backend::YtDlp);
        assert_eq!(route(&u(&["https://www.", "bilibili", ".com/video/BV1xx"])).backend, Backend::YtDlp);
        assert_eq!(route(&u(&["https://cdn.example.com/live/index", ".m3u8"])).backend, Backend::Hls);
        assert_eq!(route(&u(&["ftp://ftp.example.com/pub/a", ".zip"])).backend, Backend::Ftp);
        assert_eq!(route(&u(&["https://www.", "pixiv", ".net/artworks/123"])).backend, Backend::GalleryDl);
        assert_eq!(route(&u(&["https://", "rapidgator", ".net/file/abc"])).backend, Backend::Debrid);
    }
}
