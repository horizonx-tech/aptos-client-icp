#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use getrandom::register_custom_getrandom;

extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate url;

pub mod apis;
pub mod client;
pub mod http;
pub mod models;

fn custom_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    panic!("getrandom not supported")
}
register_custom_getrandom!(custom_getrandom);
