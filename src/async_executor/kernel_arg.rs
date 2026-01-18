use std::os::raw::c_void;

use crate::cl_types::{cl_buffer::ClBuffer, cl_image::ClImage, cl_pipe::ClPipe};

#[cfg(feature = "CL_VERSION_1_1")]
pub enum KernelArg<'a> {
    #[cfg(feature = "CL_VERSION_2_0")]
    Svm {
        arg_index: u32,
        arg: *mut c_void,
        len: usize,
    },

    Buffer {
        arg_index: u32,
        arg: &'a ClBuffer
    },

    Scalar {
        arg_index: u32,
        arg: Vec<u8>
    },

    #[cfg(feature = "CL_VERSION_1_2")]
    Image {
        arg_index: u32,
        arg: &'a ClImage
    },

    #[cfg(feature = "CL_VERSION_2_0")]
    Pipe {
        arg_index: u32,
        arg: &'a ClPipe
    }
}