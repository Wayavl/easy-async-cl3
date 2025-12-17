pub trait Formatter: Sized {
    /// Converts a byte buffer into Self
    /// Returns None if the buffer length is invalid
    fn from_buffer(buffer: &[u8]) -> Option<Self>;
}

macro_rules! impl_from_le_bytes {
    ($t:ty, $size:expr) => {
        impl Formatter for $t {
            #[inline]
            fn from_buffer(buffer: &[u8]) -> Option<Self> {
                if buffer.len() != $size {
                    return None;
                }
                let mut bytes = [0u8; $size];
                bytes.copy_from_slice(buffer);
                Some(<$t>::from_le_bytes(bytes))
            }
        }
    };
}


impl_from_le_bytes!(f32, 4);
impl_from_le_bytes!(f64, 8);
impl_from_le_bytes!(i32, 4);
impl_from_le_bytes!(u32, 4);
impl_from_le_bytes!(i64, 8);
impl_from_le_bytes!(u64, 8);


impl Formatter for bool {
    fn from_buffer(buffer: &[u8]) -> Option<Self> {
        u32::from_buffer(buffer).map(|x| x != 0)
    }
}

impl Formatter for usize {
    fn from_buffer(buffer: &[u8]) -> Option<Self> {
        if buffer.len() != std::mem::size_of::<usize>() {
            return None;
        }
        let mut bytes = vec![0u8; std::mem::size_of::<usize>()];
        bytes.copy_from_slice(buffer);
        let arr: [u8; std::mem::size_of::<usize>()] = bytes.try_into().ok()?;
        Some(usize::from_le_bytes(arr))
    }
}


impl Formatter for String {
    fn from_buffer(buffer: &[u8]) -> Option<Self> {
        String::from_utf8(buffer.to_vec()).ok()
    }
}
impl Formatter for Vec<usize> {
    fn from_buffer(buffer: &[u8]) -> Option<Self> {
        if buffer.len() % std::mem::size_of::<usize>() != 0 {
            return None;
        }
        let mut result = Vec::new();
        for chunk in buffer.chunks_exact(std::mem::size_of::<usize>()) {
            let mut bytes = vec![0u8; std::mem::size_of::<usize>()];
            bytes.copy_from_slice(chunk);
            let arr: [u8; std::mem::size_of::<usize>()] = bytes.try_into().ok()?;
            result.push(usize::from_le_bytes(arr));
        }
        Some(result)
    }
}
