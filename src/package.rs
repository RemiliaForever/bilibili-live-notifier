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

    pub const HEART_BEAT: Package = Package {
        length: 0x0000_0010,
        version: 0x0010_0001,
        action: 0x0000_0002,
        param: 0x0000_0001,
        body: None,
    };
}
