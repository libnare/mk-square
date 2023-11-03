use std::collections::HashSet;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref TARGET_EXTS_TO_SKIP: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert(".gz");
        set.insert(".tar");
        set.insert(".tgz");
        set.insert(".bz2");
        set.insert(".xz");
        set.insert(".zip");
        set.insert(".7z");
        set
    };
}

lazy_static! {
    static ref EXT_REGEX: Regex = Regex::new(r"\.[0-9a-zA-Z]+$").unwrap();
}

#[napi]
pub fn correct_filename(filename: String, ext: Option<&str>) -> String {
    let dot_ext = match ext {
        Some(e) if e.starts_with('.') => e.to_string(),
        Some(e) => format!(".{}", e),
        None => ".unknown".to_string(),
    };

    if let Some(captures) = EXT_REGEX.captures(&*filename) {
        let filename_ext = captures[0].to_lowercase();

        if ext.is_none() || filename_ext == dot_ext
            || (dot_ext == ".jpg" && filename_ext == ".jpeg")
            || (dot_ext == ".tif" && filename_ext == ".tiff")
            || (dot_ext == ".exe" && filename_ext == ".dll")
            || TARGET_EXTS_TO_SKIP.contains(&*dot_ext) {
            return filename.to_string();
        }
    }

    format!("{}{}", filename, dot_ext)
}