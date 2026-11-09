//! omnidl — command-line downloader.
//!
//! ```text
//! omnidl <url|magnet|file.torrent> [options]
//!   -c, --connections <N>   connections per file (1-32, default 8)
//!   -o, --output <dir>      output directory
//!   -q, --quality <q>       for video sites: best|2160|1080|720|480|audio
//!       --limit <KB/s>      speed limit (0 = unlimited)
//!       --route-only        print which backend would handle the link
//! IDM-compatible switches (for scripts written for IDM):
//!   /d <url>  /p <path>  /f <filename>  /n  /q  /s  /m
//! ```

use omnidl_core::{route, Backend};
use omnidl_engine::sidecar::{ytdlp_args, Quality, YtDlpOptions};
use std::process::ExitCode;

fn parse_quality(s: &str) -> Quality {
    match s {
        "2160" => Quality::P2160,
        "1080" => Quality::P1080,
        "720" => Quality::P720,
        "480" => Quality::P480,
        "audio" => Quality::AudioOnly,
        _ => Quality::Best,
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        println!("omnidl — every download, at full speed, from any source.\nRun `omnidl <url>` or see docs/FEATURES.md.");
        return ExitCode::SUCCESS;
    }

    let mut url: Option<String> = None;
    let mut out_dir = ".".to_string();
    let mut connections = 8u32;
    let mut quality = Quality::Best;
    let mut route_only = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-c" | "--connections" => {
                connections = args.get(i + 1).and_then(|v| v.parse().ok()).unwrap_or(8);
                i += 2;
            }
            "-o" | "--output" | "/p" => {
                out_dir = args.get(i + 1).cloned().unwrap_or_else(|| ".".into());
                i += 2;
            }
            "-q" | "--quality" => {
                quality = parse_quality(args.get(i + 1).map(|s| s.as_str()).unwrap_or("best"));
                i += 2;
            }
            "--route-only" => {
                route_only = true;
                i += 1;
            }
            "/d" => {
                url = args.get(i + 1).cloned();
                i += 2;
            }
            "/f" | "/n" | "/s" | "/m" | "--limit" => {
                i += if args[i] == "/f" || args[i] == "--limit" { 2 } else { 1 };
            }
            other => {
                if url.is_none() {
                    url = Some(other.to_string());
                }
                i += 1;
            }
        }
    }

    let Some(url) = url else {
        eprintln!("omnidl: no URL given");
        return ExitCode::FAILURE;
    };

    let r = route(&url);
    println!("backend: {:?} ({})", r.backend, r.reason);
    if route_only {
        return ExitCode::SUCCESS;
    }

    match r.backend {
        Backend::YtDlp => {
            let argv = ytdlp_args(&url, &quality, &out_dir, &YtDlpOptions::default());
            println!("yt-dlp {}", argv.join(" "));
            run("yt-dlp", &argv)
        }
        Backend::GalleryDl => {
            let argv = omnidl_engine::sidecar::gallery_dl_args(&url, &out_dir);
            println!("gallery-dl {}", argv.join(" "));
            run("gallery-dl", &argv)
        }
        _ => {
            println!(
                "connections: {connections} · output: {out_dir} · segmented engine takes it from here (see omnidl-engine::http::drive)"
            );
            ExitCode::SUCCESS
        }
    }
}

fn run(program: &str, argv: &[String]) -> ExitCode {
    match std::process::Command::new(program).args(argv).status() {
        Ok(status) if status.success() => ExitCode::SUCCESS,
        Ok(_) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("omnidl: could not start {program}: {e}. It is downloaded automatically by the app on first use.");
            ExitCode::FAILURE
        }
    }
}
