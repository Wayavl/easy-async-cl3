use std::os::raw::c_void;

use crate::cl_types::{cl_context::ClContext, cl_device::ClDevice, releaseable::Releaseable};

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

impl Formatter for Vec<*mut std::os::raw::c_void> {
    fn from_buffer(buffer: &[u8]) -> Option<Self> {
        if buffer.len() % std::mem::size_of::<*mut std::os::raw::c_void>() != 0 {
            return None;
        }
        let mut result = Vec::new();
        for chunk in buffer.chunks_exact(std::mem::size_of::<*mut std::os::raw::c_void>()) {
            let mut bytes = vec![0u8; std::mem::size_of::<*mut std::os::raw::c_void>()];
            bytes.copy_from_slice(chunk);
            let arr: [u8; std::mem::size_of::<*mut std::os::raw::c_void>()] = bytes.try_into().ok()?;
            let ptr = usize::from_le_bytes(arr) as *mut std::os::raw::c_void;
            result.push(ptr);
        }
        Some(result)
    }
}



impl Formatter for Vec<isize> {
    fn from_buffer(buffer: &[u8]) -> Option<Self> {
        if buffer.len() % std::mem::size_of::<isize>() != 0 {
            return None;
        }
        let mut result = Vec::new();
        for chunk in buffer.chunks_exact(std::mem::size_of::<isize>()) {
            let mut bytes = vec![0u8; std::mem::size_of::<isize>()];
            bytes.copy_from_slice(chunk);
            let arr: [u8; std::mem::size_of::<isize>()] = bytes.try_into().ok()?;
            result.push(isize::from_le_bytes(arr));
        }
        Some(result)
    }
}



impl Formatter for Vec<u64> {
    fn from_buffer(buffer: &[u8]) -> Option<Self> {
        if buffer.len() % std::mem::size_of::<u64>() != 0 {
            return None;
        }
        let mut result = Vec::new();
        for chunk in buffer.chunks_exact(std::mem::size_of::<u64>()) {
            let mut bytes = vec![0u8; std::mem::size_of::<u64>()];
            bytes.copy_from_slice(chunk);
            let arr: [u8; std::mem::size_of::<u64>()] = bytes.try_into().ok()?;
            result.push(u64::from_le_bytes(arr));
        }
        Some(result)
    }
}






// Releaseables
impl Formatter for ClContext {
    fn from_buffer(buffer: &[u8]) -> Option<Self> {
        if buffer.len() != std::mem::size_of::<*mut c_void>() {
            return None;
        }
        let ptr = unsafe { std::ptr::read_unaligned(buffer.as_ptr() as *const *mut c_void) };
        let context = ClContext::from_ptr(ptr);
        unsafe { context.increase_reference_count() };
        Some(context)
    }
}

impl Formatter for ClDevice {
    fn from_buffer(buffer: &[u8]) -> Option<Self> {
        if buffer.len() != std::mem::size_of::<*mut c_void>() {
            return None;
        }
        let ptr = unsafe { std::ptr::read_unaligned(buffer.as_ptr() as *const *mut c_void) };
        let device = ClDevice::new(ptr);
        unsafe { device.increase_reference_count() };
        Some(device)
    }
}

impl Formatter for Vec<ClDevice> {
    fn from_buffer(buffer: &[u8]) -> Option<Self> {
        let ptr_size = std::mem::size_of::<*mut c_void>();
        if buffer.len() % ptr_size != 0 {
            return None;
        }
        let mut result = Vec::with_capacity(buffer.len() / ptr_size);
        for chunk in buffer.chunks_exact(ptr_size) {
            let ptr = unsafe { std::ptr::read_unaligned(chunk.as_ptr() as *const *mut c_void) };
            let device = ClDevice::new(ptr);
            unsafe { device.increase_reference_count() };
            result.push(device);
        }
        Some(result)
    }
}
