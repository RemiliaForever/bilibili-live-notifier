use std::io::prelude::*;
use std::io::Cursor;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::result::Result::Err;
use std::str::from_utf8;
use std::thread;
use byteorder::{ReadBytesExt, BE};
use serde_json;
use serde_json::Value;

use config::Config;
use package::Package;

pub fn main_loop(config: Config) {
    loop {
        println!("正在查找服务器...");

        let socket = match format!("{}:{}", config.host, config.port).to_socket_addrs() {
            Ok(addrs) => {
                let mut socket = None;
                for addr in addrs {
                    println!("正在尝试连接服务器...");;
                    match TcpStream::connect_timeout(&addr, Duration::from_secs(20)) {
                        Ok(s) => {
                            socket = Some(s);
                            break;
                        }
                        Err(e) => println!("{}", e),
                    }
                }
                socket
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        };

        let mut stream = match socket {
            None => {
                println!("连接服务器失败！");
                continue;
            }
            Some(socket) => socket,
        };

        println!("正在进入房间...");

        let _ = stream.write_all(
            Package::join_channel(config.userid, config.roomid)
                .as_bytes()
                .as_slice(),
        );
        let mut heart_beat_stream = stream.try_clone().unwrap();
        thread::spawn(move || loop {
            heart_beat_stream
                .write_all(Package::heart_beat().as_bytes().as_slice())
                .unwrap();
            thread::sleep(Duration::from_secs(30));
        });

        recieve(stream);
    }
}

fn recieve(mut socket: TcpStream) {
    loop {
        // 解析数据包头
        let mut header = [0u8; 16];
        socket.read_exact(&mut header).unwrap();
        let mut cur = Cursor::new(header);
        let mut package = Package {
            length: cur.read_u32::<BE>().unwrap() as usize,
            version: cur.read_u32::<BE>().unwrap(),
            action: cur.read_u32::<BE>().unwrap(),
            param: cur.read_u32::<BE>().unwrap(),
            body: None,
        };
        // 读取包主体
        let mut buffer = vec![0u8; package.length - 16];
        if package.length > 16 {
            socket.read_exact(buffer.as_mut_slice()).unwrap();
        }
        match package.action {
            3 => {
                let mut cur = Cursor::new(buffer.as_slice());
                println!(
                    "当前房间人数为 {} 人",
                    cur.read_u32::<BE>().unwrap()
                );
            }
            8 => {
                println!("进入房间成功！");
            }
            5 => {
                let data = from_utf8(buffer.as_slice())
                    .unwrap_or("utf8 decode error!")
                    .to_owned();
                // TODO: 使用Result重写大量match
                match serde_json::from_str::<Value>(&data) {
                    Ok(json) => match json.get("cmd") {
                        Some(cmd) => match cmd.as_str() {
                            Some("WELCOME") => match json.get("data") {
                                Some(value) => match value.get("uname") {
                                    Some(name) => println!("欢迎 {} 来到直播间", name),
                                    None => println!("用户名解析错误"),
                                },
                                None => println!("数据包解析错误"),
                            },
                            Some("SEND_GIFT") => match json.get("data") {
                                Some(value) => {
                                    let gift_name = match value.get("giftName") {
                                        Some(gift_name) => gift_name.as_str().unwrap_or("未知"),
                                        None => "未知",
                                    };
                                    let num = match value.get("num") {
                                        Some(num) => num.as_u64().unwrap_or(999),
                                        None => 999,
                                    };
                                    let name = match value.get("uname") {
                                        Some(name) => name.as_str().unwrap_or("未知"),
                                        None => "未知",
                                    };
                                    println!(
                                        "感谢 {} 送出的 {} 个 {}！",
                                        name, num, gift_name
                                    );
                                }
                                None => println!("数据包解析错误"),
                            },
                            Some("DANMU_MSG") => match json.get("info") {
                                Some(info) => match info.as_array() {
                                    Some(array) => {
                                        let msg = &array[1];
                                        if let Some(user) = array[2].as_array() {
                                            println!("{} : {}", user[1], msg);
                                        }
                                    }
                                    None => {}
                                },
                                None => println!("数据包解析错误:{}", data),
                            },
                            Some("LIVE") => println!("直播开始"),
                            Some("PREPARING") => println!("直播准备中"),
                            _ => println!("无效的命令:{}", data),
                        },
                        None => println!("无效的数据包:{}", data),
                    },
                    Err(e) => println!("{}:{}", e, data),
                }
            }
            _ => {
                package.body = Some(
                    from_utf8(buffer.as_slice())
                        .unwrap_or("utf8 decode error!")
                        .to_owned(),
                );
                println!("未知的数据包:{:?}", package);
            }
        }
    }
}
