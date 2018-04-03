use notify_rust::Notification;
use notify_rust::NotificationHint;
use chrono::Local;

use std::collections::LinkedList;

enum Color {
    Red,
}

struct Message {
    id: String,
    color: Color,
    content: String,
}

struct Buffer {
    queue: LinkedList<Message>,
    config: BufferConfig,
}

struct BufferConfig {
    max_size: u32,
}

pub fn danmu(name: &str, content: &str) {
    println!("{} -- {} : {}", Local::now(), name, content);
    Notification::new()
        .appname("bilbili-live-notifier")
        .id(3)
        .timeout(0)
        .body(&format!("{} : {}", name, content))
        .hint(NotificationHint::X(512))
        .show();
}

pub fn gift(name: &str, gift: &str, num: u64) {
    println!(
        "{} -- 感谢 {} 送出的 {} 个 {}！",
        Local::now(),
        name,
        num,
        gift
    );
    Notification::new()
        .appname("bilbili-live-notifier")
        .id(2)
        .timeout(0)
        .body(&format!(
            "感谢 <span color=\"red\"><b>{}</b></span> 送出的 {} 个 {}！",
            name, num, gift
        ))
        .show();
}

pub fn welcome(name: &str) {
    println!("{} -- 欢迎 {} 来到直播间！", Local::now(), name);
    Notification::new()
        .appname("bilbili-live-notifier")
        .id(1)
        .timeout(0)
        .body(&format!("欢迎 {} 来到直播间！", name))
        .show();
}
