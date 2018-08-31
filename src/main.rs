extern crate byteorder;
extern crate chrono;
extern crate notify_rust;
extern crate serde_json;

pub mod config;
pub mod daemon;
pub mod notify;
pub mod package;

fn main() {
    let config = config::Config {
        host: "livecmt-2.bilibili.com".to_owned(),
        port: 2243,
        userid: 6217572,
        roomid: 73700,
    };
    daemon::main_loop(config);
}
