use std::time::{Duration, SystemTime};
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use nanoid::nanoid;
use napi::{Env, JsObject};
use rand::Rng;
use crate::to_base36;

#[napi(js_name = "aidxRegExp")]
pub const AIDX_REG_EXP: &str = "^[0-9a-z]{16}$";

const TIME2000: i64 = 946_684_800_000;
const ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8',
    '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
    'i', 'j', 'k','l', 'm', 'n', 'o', 'p', 'q',
    'r', 's', 't','u', 'v', 'w', 'x', 'y', 'z',
];
lazy_static!(
    static ref NODE_ID: String = nanoid!(4, &ALPHABET);
);

fn get_time(time: i64) -> String {
    let time = (time - TIME2000).max(0);
    let base36_time = to_base36(time as u64);
    format!("{:0>8}", base36_time)
}

fn get_noise() -> String {
    const NOISE_LENGTH: usize = 4;
    let mut rng = rand::thread_rng();
    let noise: String = (0..NOISE_LENGTH)
        .map(|_| rng.gen_range(0..36))
        .map(|digit| to_base36(digit as u64))
        .collect();
    noise
}

#[napi]
pub fn gen_aidx(t: i64) -> String {
    let time = get_time(t);
    let noise = get_noise();

    let mut aidx = String::with_capacity(16);
    aidx.push_str(&time);
    aidx.push_str(&noise);
    aidx.push_str(&NODE_ID);

    aidx
}

#[napi(ts_return_type="{ date: Date; }")]
fn parse_aidx(env: Env, id: String) -> napi::Result<JsObject> {
    let time = i64::from_str_radix(&id[..8], 36).unwrap() + TIME2000;
    let system_time = SystemTime::UNIX_EPOCH
        .checked_add(Duration::from_millis(time as u64))
        .ok_or("Failed to parse AIDX: Invalid Date").unwrap();
    let mut obj = env.create_object()?;
    obj.set_named_property("date", DateTime::<Utc>::from(system_time))?;

    return Ok(obj);
}

#[napi]
fn is_safe_aidx_t(t: i64) -> bool {
    t > TIME2000
}