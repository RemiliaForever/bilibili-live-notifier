use std::{str, thread, u32, u64};
use std::io::{Cursor, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::result::Result::Err;
use std::time::Duration;
use std::sync::mpsc::{self, TryRecvError};

use byteorder::{ReadBytesExt, BE};
use serde_json::{self, Value};

use notify::{danmu, gift, welcome};
use config::Config;
use package::Package;

/// 后台线程
pub fn main_loop(config: Config) {
    loop {
        // 解析服务器地址，并在多个结果中循环尝试至获得第一个成功的socket连接
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
        // 检查连接，获取失败则等待3秒后重试
        let mut stream = match socket {
            None => {
                println!("连接服务器失败！");
                thread::sleep(Duration::from_secs(3));
                continue;
            }
            Some(socket) => socket,
        };
        // 尝试进入房间
        println!("正在进入房间...");
        if let Err(e) = send(
            &mut stream,
            Package::join_channel(config.userid, config.roomid),
        ) {
            // 进入失败则重新连接服务器
            println!("进入房间失败:{}", e);
            continue;
        }
        // 后台开启心跳线程，每30秒发送一次
        let mut heart_beat_stream = stream.try_clone().unwrap();
        let (tx, rx) = mpsc::channel::<()>();
        thread::spawn(move || loop {
            // 检测主线程状态，收到终止信号结束循环
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("心跳线程终止！");
                    break;
                }
                Err(TryRecvError::Empty) => {}
            };
            // 发送成功等待30秒，失败则10秒后重试
            if let Err(e) = send(&mut heart_beat_stream, Package::heart_beat()) {
                println!("心跳包发送失败:{}", e);
                thread::sleep(Duration::from_secs(10));
            } else {
                thread::sleep(Duration::from_secs(30));
            }
        });
        // 开始主循环
        if let Err(e) = recieve(stream) {
            println!("网络连接出错:{}", e);
        }
        // 主循环结束，通知后台线程
        if let Err(e) = tx.send(()) {
            println!("结束子线程出错:{}", e);
        }
    }
}

fn send(stream: &mut TcpStream, package: Package) -> Result<(), &'static str> {
    let bytes = package.as_bytes()?;
    stream
        .write_all(bytes.as_slice())
        .or(Err("数据包发送失败！"))
}

/// 这一层抛出网络连接和原始数据包解析的错误，这个错误需要重新连接服务器
fn recieve(mut socket: TcpStream) -> Result<(), &'static str> {
    loop {
        // 解析数据包头
        let mut header = [0u8; 16];
        socket
            .read_exact(&mut header)
            .or(Err("socket读取出错"))?;
        let mut cur = Cursor::new(header);
        let mut package = Package {
            length: cur.read_u32::<BE>().or(Err("length解析出错"))? as usize,
            version: cur.read_u32::<BE>().or(Err("version解析出错"))?,
            action: cur.read_u32::<BE>().or(Err("action解析出错"))?,
            param: cur.read_u32::<BE>().or(Err("param解析出错"))?,
            body: None,
        };
        // 读取包主体
        let mut buffer = vec![0u8; package.length - 16];
        if package.length > 16 {
            socket
                .read_exact(buffer.as_mut_slice())
                .or(Err("socket读取出错"))?;
            package.body = Some(
                str::from_utf8(buffer.as_slice())
                    .unwrap_or("utf8 decode error!")
                    .to_owned(),
            );
        }
        match package.action {
            3 => println!(
                "当前房间人数为 {} 人",
                Cursor::new(buffer.as_slice())
                    .read_u32::<BE>()
                    .unwrap_or(u32::MAX)
            ),
            8 => println!("进入房间成功！"),
            5 => {
                let data = str::from_utf8(buffer.as_slice())
                    .unwrap_or("utf8 decode error!")
                    .to_owned();
                if let Err(e) = parse_danmu(&data) {
                    println!("解析弹幕数据出错:{}", e);
                    println!("{:?}", package);
                }
            }
            _ => println!("未知的数据包:{:?}", package),
        }
    }
}

/// 这一层抛出数据包主体结构的错误，
/// 这个错误无需重连服务器。
/// 数据包内容解析错误不会抛出。
fn parse_danmu(data: &str) -> Result<(), &'static str> {
    let json: Value = serde_json::from_str::<Value>(data).or(Err("body解析出错"))?;
    let cmd = json.get("cmd").ok_or("未知的命令")?;
    match cmd.as_str() {
        Some("WELCOME") => {
            let value = json.get("data").ok_or("data解析出错")?;
            let name = value
                .get("uname")
                .map_or("未知", |uname| uname.as_str().unwrap_or("未知"));
            Ok(welcome(name))
        }
        Some("WELCOME_GUARD") => {
            let value = json.get("data").ok_or("data解析出错")?;
            let name = value
                .get("username")
                .map_or("未知", |uname| uname.as_str().unwrap_or("未知"));
            Ok(welcome(name))
        }
        Some("SEND_GIFT") => {
            let value = json.get("data").ok_or("data解析出错")?;
            let gift_name = value
                .get("giftName")
                .map_or("未知", |gift_name| gift_name.as_str().unwrap_or("未知"));
            let num = value
                .get("num")
                .map_or(u64::MAX, |num| num.as_u64().unwrap_or(u64::MAX));
            let name = value
                .get("uname")
                .map_or("未知", |uname| uname.as_str().unwrap_or("未知"));
            Ok(gift(name, gift_name, num))
        }
        Some("DANMU_MSG") => {
            let info = json.get("info").ok_or("info解析出错")?;
            let array = info.as_array().ok_or("弹幕信息解析出错")?;
            if array.len() <= 3 {
                Err("弹幕信息解析出错")?;
            }
            let msg = array[1].as_str().ok_or("弹幕内容解析出错")?;
            let user = array[2]
                .as_array()
                .map_or("未知", |user| user[1].as_str().unwrap_or("未知"));
            Ok(danmu(user, msg))
        }
        Some("LIVE") => Ok(println!("直播开始")),
        Some("PREPARING") => Ok(println!("直播准备中")),
        _ => Err("无效的命令"),
    }
}
