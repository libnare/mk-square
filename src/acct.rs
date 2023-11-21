use napi::{Env, JsObject};

#[napi(object)]
pub struct Acct {
    pub username: String,
    #[napi(ts_type = "string | null")]
    pub host: Option<String>,
}

#[napi(ts_return_type="{ username: string; host: string | null; }")]
pub fn parse(env: Env, acct: String) -> napi::Result<JsObject> {
    let acct = acct.trim_start_matches('@').to_owned();
    let mut split = acct.splitn(2, '@');
    let username = split.next().unwrap().to_owned();
    let host = split.next().map(str::to_owned);
    let mut obj = env.create_object()?;
    match host {
        Some(host) => {
            obj.set_named_property("username", username)?;
            obj.set_named_property("host", host)?;
        },
        None => {
            obj.set_named_property("username", username)?;
            obj.set_named_property("host", &env.get_null()?)?;
        }
    }
    Ok(obj)
}

#[napi]
pub fn to_string(acct: Acct) -> String {
    match acct.host {
        Some(ref host) => format!("{}@{}", acct.username, host),
        None => acct.username,
    }
}
