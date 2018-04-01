use byteorder::WriteBytesExt;
use byteorder::BE;

#[derive(Debug)]
pub struct Package {
    pub length: usize,
    pub version: u32,
    pub action: u32,
    pub param: u32,
    pub body: String,
}

impl Package {
    pub fn new() -> Package {
        Package {
            length: 0x0000_0010,
            version: 0x0010_0001,
            action: 0x0000_0000,
            param: 0x0000_0001,
            body: "".to_owned(),
        }
    }

    pub fn set_body(&mut self, body: String) {
        self.length = body.as_bytes().len() + 16;
        self.body = body;
    }

    pub fn join_channel(user_id: u32, room_id: u32) -> Package {
        let mut package = Package::new();
        package.action = 7;
        let body = format!("{{\"roomid\":{},\"uid\":{}}}", room_id, user_id);
        package.set_body(body);
        package
    }

    pub fn heart_beat() -> Package {
        let mut package = Package::new();
        package.action = 2;
        package
    }

    pub fn as_bytes(self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(self.length);
        buffer.write_u32::<BE>(self.length as u32).unwrap();
        buffer.write_u32::<BE>(self.version).unwrap();
        buffer.write_u32::<BE>(self.action).unwrap();
        buffer.write_u32::<BE>(self.param).unwrap();
        buffer.extend_from_slice(self.body.as_bytes());
        buffer
    }
}
