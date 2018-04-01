extern crate notify_rust;
extern crate byteorder;

mod daemon;
mod config;
mod package;
mod notify;

fn main() {
    let config = config::Config {
        host: "livecmt-2.bilibili.com".to_owned(),
        port: 2243,
        userid: 0,
        roomid: 0,
    };
    daemon::main_loop(config);
}
