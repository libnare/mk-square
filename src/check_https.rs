use url::Url;

#[napi]
pub fn check_https(url: String, node_env: Option<String>) -> bool {
    if let Ok(parsed_url) = Url::parse(&*url) {
        match parsed_url.scheme() {
            "https" => true,
            "http" if node_env.as_deref() != Some("production") => true,
            _ => false,
        }
    } else {
        false
    }
}
