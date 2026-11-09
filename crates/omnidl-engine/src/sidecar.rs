//! Command construction for external tools. Pure functions so they can
//! be unit-tested without the tools installed.

/// What the user picked in the "New download" sheet for a video page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Quality {
    Best,
    P2160,
    P1080,
    P720,
    P480,
    AudioOnly,
}

impl Quality {
    fn format_selector(&self) -> &'static str {
        match self {
            Quality::Best => "bv*+ba/b",
            Quality::P2160 => "bv*[height<=2160]+ba/b[height<=2160]",
            Quality::P1080 => "bv*[height<=1080]+ba/b[height<=1080]",
            Quality::P720 => "bv*[height<=720]+ba/b[height<=720]",
            Quality::P480 => "bv*[height<=480]+ba/b[height<=480]",
            Quality::AudioOnly => "ba/bestaudio",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct YtDlpOptions {
    pub playlist: bool,
    pub subtitles: bool,
    pub sponsorblock_remove: bool,
    pub cookies_from_browser: Option<String>,
}

/// Build the yt-dlp argument vector (program name excluded).
pub fn ytdlp_args(
    url: &str,
    quality: &Quality,
    out_dir: &str,
    opts: &YtDlpOptions,
) -> Vec<String> {
    let mut a: Vec<String> = vec![
        "--newline".into(),
        "--no-mtime".into(),
        "--progress-template".into(),
        "%(progress._percent_str)s|%(progress._speed_str)s|%(progress._eta_str)s".into(),
        "-f".into(),
        quality.format_selector().into(),
        "--merge-output-format".into(),
        "mp4".into(),
        "-o".into(),
        format!("{out_dir}/%(title)s [%(id)s].%(ext)s"),
    ];
    if *quality == Quality::AudioOnly {
        a.push("-x".into());
        a.push("--audio-format".into());
        a.push("mp3".into());
    }
    if !opts.playlist {
        a.push("--no-playlist".into());
    }
    if opts.subtitles {
        a.push("--write-subs".into());
        a.push("--write-auto-subs".into());
        a.push("--sub-langs".into());
        a.push("all,-live_chat".into());
        a.push("--embed-subs".into());
    }
    if opts.sponsorblock_remove {
        a.push("--sponsorblock-remove".into());
        a.push("all".into());
    }
    if let Some(browser) = &opts.cookies_from_browser {
        a.push("--cookies-from-browser".into());
        a.push(browser.clone());
    }
    a.push(url.to_string());
    a
}

/// Build the gallery-dl argument vector (program name excluded).
pub fn gallery_dl_args(url: &str, out_dir: &str) -> Vec<String> {
    vec![
        "--no-mtime".into(),
        "-d".into(),
        out_dir.to_string(),
        url.to_string(),
    ]
}

/// ffmpeg concat command to merge HLS/DASH segments (or re-mux a
/// downloaded stream) without re-encoding.
pub fn ffmpeg_merge_args(list_file: &str, out_file: &str) -> Vec<String> {
    vec![
        "-y".into(),
        "-f".into(),
        "concat".into(),
        "-safe".into(),
        "0".into(),
        "-i".into(),
        list_file.to_string(),
        "-c".into(),
        "copy".into(),
        out_file.to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ytdlp_args_carry_quality_and_url() {
        let args = ytdlp_args(
            "https://video.example/watch/1",
            &Quality::P1080,
            "/dl",
            &YtDlpOptions::default(),
        );
        assert!(args.iter().any(|x| x.contains("height<=1080")));
        assert_eq!(args.last().unwrap(), "https://video.example/watch/1");
        assert!(args.iter().any(|x| x == "--no-playlist"));
    }

    #[test]
    fn audio_only_extracts_mp3() {
        let args = ytdlp_args(
            "https://video.example/watch/1",
            &Quality::AudioOnly,
            "/dl",
            &YtDlpOptions::default(),
        );
        assert!(args.iter().any(|x| x == "-x"));
        assert!(args.iter().any(|x| x == "mp3"));
    }
}
