#[napi(object)]
pub struct Acct {
    pub username: String,
    pub host: Option<String>,
}

#[napi]
pub fn parse(acct: String) -> Acct {
    let acct = acct.trim_start_matches('@').to_owned();
    let mut split = acct.splitn(2, '@');
    let username = split.next().unwrap().to_owned();
    let host = split.next().map(str::to_owned);
    Acct { username, host }
}

#[napi]
pub fn to_string(acct: Acct) -> String {
    match acct.host {
        Some(ref host) => format!("{}@{}", acct.username, host),
        None => acct.username,
    }
}
