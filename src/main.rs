extern crate byteorder;
extern crate notify_rust;
extern crate serde_json;

pub mod daemon;
pub mod config;
pub mod package;
pub mod notify;

fn main() {
    let config = config::Config {
        host: "livecmt-2.bilibili.com".to_owned(),
        port: 2243,
        userid: 8393961,
        roomid: 90713,
    };
    daemon::main_loop(config);
}
