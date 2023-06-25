#[napi]
pub fn correct_filename(filename: String, ext: Option<String>) -> String {
    let dot_ext = match ext {
        Some(ref e) if e.starts_with('.') => e.clone(),
        Some(ref e) => format!(".{}", e),
        None => String::from(".unknown"),
    };

    if filename.ends_with(&dot_ext) {
        return filename;
    }

    match ext.as_deref() {
        Some("jpg") if filename.ends_with(".jpeg") => filename,
        Some("tif") if filename.ends_with(".tiff") => filename,
        _ => format!("{}{}", filename, dot_ext),
    }
}
