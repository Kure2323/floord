use dirs::{audio_dir, document_dir, picture_dir, video_dir};
use std::fs::{DirEntry, create_dir_all, rename};
use std::path::Path;

const AUDIO_EXT: &[&str] = &[
    "mp3", "wav", "flac", "aac", "ogg", "opus",
    "m4a", "wma", "aiff", "mid", "midi", "amr",
];

const PICTURE_EXT: &[&str] = &[
    "jpg", "jpeg", "jp2", "j2k", "jpf", "jpx",
    "jpm", "mj2", "png", "gif", "bmp", "webp",
    "svg", "tif", "tiff", "ico", "heic", "heif",
    "avif", "raw", "psd", "bmp", "dib"
];

const VIDEO_EXT: &[&str] = &[
    "mp4", "mkv", "avi", "mov", "wmv", "flv",
    "webm", "m4v", "mpeg", "mpg", "3gp", "ts",
    "mts", "vob", "ogv",
];

const DOCUMENT_EXT: &[&str] = &[
    "pdf", "doc", "docx", "odt", "rtf", "txt",
    "md", "tex", "xls", "xlsx", "ods", "csv",
    "ppt", "pptx", "odp", "epub", "mobi", "pages",
];

fn save_unknown(path: &Path, inbox: &Path) {
    let unknown_path = inbox.join("Unknown");
    create_dir_all(&unknown_path).expect("[ERROR] Could not create inbox/Unknown directory");

    let file_name = path.file_name().expect("[ERROR] Getting bad paths example: (/foo.txt/..)");
    rename(path, unknown_path.join(file_name)).expect("[ERROR] Verify inbox does exist");

}

pub fn save(file: DirEntry, inbox: &Path) {

    let path = file.path();

    if path.is_dir() && path.file_name().expect("[ERROR] Could not read file name") == "Unknown" {
        return;
    }

    // TODO: Verify if the new path already exists and control check
    if let Some(extension) = path.extension().and_then(|i| { i.to_str() }) {
        if AUDIO_EXT.contains(&extension) {
            if let Some(audio_dir) = audio_dir() {
                let file_name = path.file_name().expect("[ERROR] Getting bad paths example: (/foo.txt/..)");
                let new_path = audio_dir.join(file_name);

                if let Err(_) = rename(path, new_path) {
                    return;
                }
                return;
            }
            return;

        }
        if PICTURE_EXT.contains(&extension) {
            if let Some(picture_dir) = picture_dir() {
                let file_name = path.file_name().expect("[ERROR] Getting bad paths example: (/foo.txt/..)");
                let new_path = picture_dir.join(file_name);
                if let Err(_) = rename(path, new_path) {
                    return;
                }
                return;
            }
            return
        }
        if VIDEO_EXT.contains(&extension) {
            if let Some(video_dir) = video_dir() {
                let file_name = path.file_name().expect("[ERROR] Getting bad paths example: (/foo.txt/..)");
                let new_path = video_dir.join(file_name);
                if let Err(_) = rename(path, new_path) {
                    return;
                }
                return;
            }
            return
        }
        if DOCUMENT_EXT.contains(&extension) {
            if let Some(document_dir) = document_dir() {
                let file_name = path.file_name().expect("[ERROR] Getting bad paths example: (/foo.txt/..)");
                let new_path = document_dir.join(file_name);
                if let Err(_) = rename(path, new_path) {
                    return;
                }               return;
            }
            return;
        }
    }

    save_unknown(&path, inbox);

}