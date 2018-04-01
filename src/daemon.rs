use std::io::prelude::*;
use std::io::Cursor;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::result::Result::Err;
use std::str::from_utf8;
use std::thread;
use byteorder::{ReadBytesExt, BE};

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
        let mut header = [0u8; 16];
        socket.read(&mut header).unwrap();
        let mut cur = Cursor::new(header);
        let mut package = Package {
            length: cur.read_u32::<BE>().unwrap() as usize,
            version: cur.read_u32::<BE>().unwrap(),
            action: cur.read_u32::<BE>().unwrap(),
            param: cur.read_u32::<BE>().unwrap(),
            body: "".to_owned(),
        };
        match package.action {
            3 => {
                let mut buffer = vec![0u8; package.length - 16];
                socket.read(buffer.as_mut_slice()).unwrap();
                let mut cur = Cursor::new(buffer.as_slice());
                println!(
                    "当前房间人数为 {} 人",
                    cur.read_u32::<BE>().unwrap()
                );
            }
            8 => {
                println!("进入房间成功！");
            }
            _ => {
                let mut buffer = vec![0u8; package.length - 16];
                socket.read(buffer.as_mut_slice()).unwrap();
                package.body = from_utf8(buffer.as_slice())
                    .unwrap_or("utf8 decode error!")
                    .to_owned();
                println!("未知的数据包:{:?}", package);
            }
        }
    }
}
