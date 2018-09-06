extern crate byteorder;
extern crate chrono;
extern crate notify_rust;
extern crate serde_json;

pub mod config;
pub mod daemon;
pub mod notify;
pub mod package;

use std::env;

fn main() {
    let mut args = env::args();
    let config = config::Config {
        host: "livecmt-2.bilibili.com".to_owned(),
        port: 2243,
        userid: 6217572,
        roomid: args.nth(1).unwrap().parse().unwrap(),
    };
    daemon::main_loop(config);
}
