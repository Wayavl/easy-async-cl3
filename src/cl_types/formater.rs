pub trait Formater {
    fn from_buffer(buffer: Vec<u8>) -> Self;
}

impl Formater for f32 {
    fn from_buffer(buffer: Vec<u8>) -> Self {
        let bytes = [buffer[0], buffer[1], buffer[2], buffer[3]];
        f32::from_le_bytes(bytes)
    }
}

impl Formater for f64 {
    fn from_buffer(buffer: Vec<u8>) -> Self {
        let bytes = [buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7]];
        f64::from_le_bytes(bytes)
    }
}

impl Formater for i32 {
    fn from_buffer(buffer: Vec<u8>) -> Self {
        let bytes = [buffer[0], buffer[1], buffer[2], buffer[3]];
        i32::from_le_bytes(bytes)
    }
}

impl Formater for u32 {
    fn from_buffer(buffer: Vec<u8>) -> Self {
        let bytes = [buffer[0], buffer[1], buffer[2], buffer[3]];
        u32::from_le_bytes(bytes)
    }
}

impl Formater for i64 {
    fn from_buffer(buffer: Vec<u8>) -> Self {
        let bytes = [buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7]];
        i64::from_le_bytes(bytes)
    }
}

impl Formater for u64 {
    fn from_buffer(buffer: Vec<u8>) -> Self {
        let bytes = [buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7]];
        u64::from_le_bytes(bytes)
    }
}

impl Formater for String {
    fn from_buffer(buffer: Vec<u8>) -> Self {
        String::from_utf8_lossy(&buffer).into_owned()
    }
}