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
    let mut new_path = unknown_path.join(file_name);
    match write_to_path(path, &new_path) {
        Ok(_) => {}
        Err(WritingError::Exists) => {
            rename_destination(&mut new_path);
            let _ = write_to_path(path, &new_path);
        }
        _ => {}
    }

}
// TODO: Check if file at destination path already exists
pub fn save(file: DirEntry, inbox: &Path) {

    let path = file.path();

    if path.is_dir() && path.file_name().expect("[ERROR] Could not read file name") == "Unknown" {
        return;
    }

    if let Some(extension) = path.extension().and_then(|i| { i.to_str() }) {
        if AUDIO_EXT.contains(&extension) {
            if let Some(audio_dir) = audio_dir() {
                let file_name = path.file_name().expect("[ERROR] Getting bad paths example: (/foo.txt/..)");
                let mut new_path = audio_dir.join(file_name);
                write(&path, &mut new_path)
            }
            return;

        }
        if PICTURE_EXT.contains(&extension) {
            if let Some(picture_dir) = picture_dir() {
                let file_name = path.file_name().expect("[ERROR] Getting bad paths example: (/foo.txt/..)");
                let mut new_path = picture_dir.join(file_name);
                write(&path, &mut new_path)
            }
            return
        }
        if VIDEO_EXT.contains(&extension) {
            if let Some(video_dir) = video_dir() {
                let file_name = path.file_name().expect("[ERROR] Getting bad paths example: (/foo.txt/..)");
                let mut new_path = video_dir.join(file_name);
                write(&path, &mut new_path)
            }
            return
        }
        if DOCUMENT_EXT.contains(&extension) {
            if let Some(document_dir) = document_dir() {
                let file_name = path.file_name().expect("[ERROR] Getting bad paths example: (/foo.txt/..)");
                let mut new_path = document_dir.join(file_name);
                write(&path, &mut new_path)
            }
            return;
        }
    }

    save_unknown(&path, inbox);

}

fn write(path: &Path, new_path: &mut PathBuf) {
    match write_to_path(path, new_path) {
        Ok(()) => {}
        Err(WritingError::Exists) => {
            rename_destination(new_path);

            let _ = write_to_path(path, new_path);
        }
        _ => {}
    }
}

#[derive(Debug, PartialEq)]
enum WritingError {
    Exists,
    IsSymlink,
    Renaming,
}

fn write_to_path(file_path: &Path, new_path: &Path) -> Result<(), WritingError> {
    if new_path.exists() {
        return Err(WritingError::Exists);
    }
    if file_path.is_symlink() {
        return Err(WritingError::IsSymlink)
    }

    if rename(file_path, new_path).is_err() {
        return Err(WritingError::Renaming)
    }

    Ok(())
}


use std::path::PathBuf;

fn rename_destination(new_path: &mut PathBuf) {
    if let (Some(file_name), Some(folder)) =
        (new_path.file_stem(), new_path.parent())
    {
        let file_name = file_name.to_string_lossy().to_string();
        let folder = folder.to_path_buf();

        let extension = new_path.extension().map(|e| e.to_os_string());

        let (base_name, mut count) = match file_name.rsplit_once('(') {
            Some((name, number)) => {
                match number.strip_suffix(')').and_then(|n| n.parse::<u32>().ok()) {
                    Some(n) if !name.is_empty() => (name.to_string(), n + 1),
                    _ => (file_name.clone(), 1),
                }
            }
            None => (file_name.clone(), 1),
        };

        while new_path.exists() {
            let new_name = match &extension {
                Some(ext) => format!(
                    "{} ({}).{}",
                    base_name,
                    count,
                    ext.to_string_lossy()
                ),
                None => format!("{}({})", base_name, count),
            };

            *new_path = folder.join(new_name);
            count += 1;
        }
    }
}