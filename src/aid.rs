use std::time::{Duration, SystemTime};

use chrono::{DateTime, Utc};
use napi::{Env, JsObject};
use rand::Rng;

#[napi(js_name = "aidRegExp")]
pub const AID_REG_EXP: &str = "^[0-9a-z]{10}$";

const TIME2000: i64 = 946_684_800_000;

fn to_base36(mut decimal: u64) -> String {
    const CHARS: &[char] = &[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let mut result = String::new();
    let base = CHARS.len() as u64;

    while decimal > 0 {
        let rem = decimal % base;
        decimal /= base;
        result.insert(0, CHARS[rem as usize]);
    }

    result
}

fn get_time(time: i64) -> String {
    let time = (time - TIME2000).max(0);
    let base36_time = to_base36(time as u64);
    format!("{:0>8}", base36_time)
}

fn get_noise() -> String {
    const NOISE_LENGTH: usize = 2;
    let mut rng = rand::thread_rng();
    let noise: String = (0..NOISE_LENGTH)
        .map(|_| rng.gen_range(0..36))
        .map(|digit| to_base36(digit as u64))
        .collect();
    noise
}

#[napi]
fn gen_aid(t: i64) -> String {
    let time = get_time(t);
    let noise = get_noise();

    let mut aid = String::with_capacity(10);
    aid.push_str(&time);
    aid.push_str(&noise);

    aid
}

#[napi]
fn parse_aid(env: Env, id: String) -> napi::Result<JsObject> {
    let time = i64::from_str_radix(&id[..8], 36).unwrap() + TIME2000;
    let system_time = SystemTime::UNIX_EPOCH
        .checked_add(Duration::from_millis(time as u64))
        .ok_or("Failed to parse AID: Invalid Date").unwrap();
    let mut obj = env.create_object()?;
    obj.set_named_property("date", DateTime::<Utc>::from(system_time))?;

    return Ok(obj);
}
