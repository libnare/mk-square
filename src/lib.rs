#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod aid;
pub mod check_https;
pub mod corrent_filename;
mod secure_rndstr;
