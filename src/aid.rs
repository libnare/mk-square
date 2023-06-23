use std::time::{Duration, SystemTime};
use chrono::{DateTime, Utc};
use rand::Rng;

#[napi(js_name = "aidRegExp")]
pub const AID_REG_EXP: &str = "^[0-9a-z]{10}$";

const TIME2000: i64 = 946_684_800_000;

trait ToBase36 {
    fn to_base36(&self) -> Option<String>;
}

impl ToBase36 for String {
    fn to_base36(&self) -> Option<String> {
        let decimal = self.parse::<u64>().ok()?;
        let chars: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyz".chars().collect();

        let mut result = String::new();
        let base = chars.len() as u64;
        let mut n = decimal;

        while n > 0 {
            let rem = n % base;
            n /= base;
            result.push(chars[rem as usize]);
        }

        Some(result.chars().rev().collect())
    }
}

trait PadStart {
    fn pad_start(&self, width: usize, fill: char) -> String;
}

impl PadStart for String {
    fn pad_start(&self, width: usize, fill: char) -> String {
        let len = self.len();
        if len >= width {
            return self.clone();
        }
        let padding = fill.to_string().repeat(width - len);
        padding + self
    }
}

fn get_time(time: i64) -> String {
    let mut time = time - TIME2000;
    if time < 0 {
        time = 0;
    }
    time.to_string()
        .to_base36()
        .unwrap()
        .pad_start(8, '0')
}

fn get_noise() -> String {
    let mut rng = rand::thread_rng();
    let noise: u16 = rng.gen_range(0..=u16::MAX);
    let noise_base36 = noise.to_string().to_base36().unwrap();
    let truncated_noise = noise_base36.chars().take(2).collect::<String>();
    truncated_noise.pad_start(2, '0')
}

#[napi]
fn gen_aid(date: DateTime<Utc>) -> String {
    let t = date.timestamp_millis();
    let time = get_time(t);
    let noise = get_noise();
    format!("{}{}", time, noise)
}

#[napi(object)]
struct ParseAid {
    pub date: DateTime<Utc>,
}

#[napi]
fn parse_aid(id: String) -> ParseAid {
    let time = i64::from_str_radix(&id[..8], 36).unwrap() + TIME2000;
    let system_time = SystemTime::UNIX_EPOCH
        .checked_add(Duration::from_millis(time as u64))
        .ok_or("Failed to parse AID: Invalid Date").unwrap();

    ParseAid { date: DateTime::<Utc>::from(system_time) }
}
