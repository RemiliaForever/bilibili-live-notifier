use notify_rust::Notification;
use std::time::Duration;
use std::thread::sleep;

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

fn notify() {
    let handle = Notification::new()
        .summary("First Notification")
        .body("This notification will be changed!")
        .icon("dialog-warning")
        .show()
        .unwrap();
    let id = handle.id();
    sleep(Duration::from_millis(1_500));

    Notification::new()
        .appname("foo") // changing appname to keep plasma from merging the new and the old one
        .icon("dialog-ok")
        .body("<b>This</b> has been changed by sending a new notification with the same id")
        .id(id)
        .show()
        .unwrap();
    for i in 1..5 {
        sleep(Duration::from_millis(5_000));
        Notification::new()
            .icon("dialog-ok")
            .body(&format!("notification{}", i))
            .id(id)
            .show()
            .unwrap();
    }
}
