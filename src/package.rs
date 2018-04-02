use byteorder::WriteBytesExt;
use byteorder::BE;

/// 数据包封装
#[derive(Debug)]
pub struct Package {
    /// 数据包长度，最小16
    pub length: usize,
    /// 未知，可能是版本和设备类型
    pub version: u32,
    /// * 2 - 心跳/请求在线人数
    /// * 3 - 在线人数
    /// * 5 - 弹幕数据
    /// * 7 - 请求进入房间
    /// * 8 - 回复进入房间
    pub action: u32,
    /// 未知，可能是设备类型
    pub param: u32,
    pub body: Option<String>,
}

impl Package {
    /// 新建一个body为空的数据包
    pub fn new() -> Package {
        Package {
            length: 0x0000_0010,
            version: 0x0010_0001,
            action: 0x0000_0000,
            param: 0x0000_0001,
            body: None,
        }
    }

    /// 修改数据包body并更新length
    pub fn set_body(&mut self, body: Option<String>) {
        match body {
            Some(body) => {
                self.length = body.as_bytes().len() + 16;
                self.body = Some(body);
            }
            None => {
                self.length = 0x0000_0010;
                self.body = None;
            }
        }
    }

    /// 进入房间
    pub fn join_channel(user_id: u32, room_id: u32) -> Package {
        let mut package = Package::new();
        package.action = 7;
        let body = format!("{{\"roomid\":{},\"uid\":{}}}", room_id, user_id);
        package.set_body(Some(body));
        package
    }

    /// 心跳包/请求房间人数
    pub fn heart_beat() -> Package {
        let mut package: Package = Package::new();
        package.action = 2;
        package
    }

    /// 包装为二进制
    pub fn as_bytes(self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(self.length);
        buffer.write_u32::<BE>(self.length as u32).unwrap();
        buffer.write_u32::<BE>(self.version).unwrap();
        buffer.write_u32::<BE>(self.action).unwrap();
        buffer.write_u32::<BE>(self.param).unwrap();
        match self.body {
            Some(body) => buffer.extend_from_slice(body.as_bytes()),
            None => {}
        };
        buffer
    }
}
