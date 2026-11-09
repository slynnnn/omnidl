//! Automatic categorisation by file extension — the IDM habit users
//! rely on: finished files land in Video / Music / Programs / Compressed
//! / Documents folders without being asked every time.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Video,
    Music,
    Programs,
    Compressed,
    Documents,
    Images,
    Torrents,
    Other,
}

impl Category {
    pub fn folder_name(&self) -> &'static str {
        match self {
            Category::Video => "Video",
            Category::Music => "Music",
            Category::Programs => "Programs",
            Category::Compressed => "Compressed",
            Category::Documents => "Documents",
            Category::Images => "Images",
            Category::Torrents => "Torrents",
            Category::Other => "Other",
        }
    }
}

fn ext_of(name: &str) -> String {
    name.rsplit('.')
        .next()
        .filter(|e| e.len() <= 5 && !e.contains('/'))
        .unwrap_or("")
        .to_ascii_lowercase()
}

pub fn categorise(filename: &str) -> Category {
    match ext_of(filename).as_str() {
        "mp4" | "mkv" | "avi" | "mov" | "webm" | "flv" | "wmv" | "m4v" | "ts" => Category::Video,
        "mp3" | "flac" | "wav" | "aac" | "ogg" | "m4a" | "opus" => Category::Music,
        "exe" | "msi" | "dmg" | "pkg" | "deb" | "rpm" | "apk" | "ipa" | "appimage" => {
            Category::Programs
        }
        "zip" | "7z" | "rar" | "tar" | "gz" | "xz" | "bz2" | "zst" | "iso" | "img" => {
            Category::Compressed
        }
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "md" | "epub" => {
            Category::Documents
        }
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg" | "avif" | "bmp" => Category::Images,
        "torrent" => Category::Torrents,
        _ => Category::Other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn categorises_common_types() {
        assert_eq!(categorise("movie.MKV"), Category::Video);
        assert_eq!(categorise("setup.exe"), Category::Programs);
        assert_eq!(categorise("ubuntu.iso"), Category::Compressed);
        assert_eq!(categorise("paper.pdf"), Category::Documents);
        assert_eq!(categorise("noext"), Category::Other);
    }
}
