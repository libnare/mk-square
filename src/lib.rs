#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod aid;
pub mod corrent_filename;
pub mod secure_rndstr;
pub mod acct;
pub mod nsfw;
pub mod aidx;

pub fn to_base36(mut decimal: u64) -> String {
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