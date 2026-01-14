use cl3::ext::{CL_MEM_OBJECT_IMAGE1D, CL_MEM_OBJECT_IMAGE1D_ARRAY, CL_MEM_OBJECT_IMAGE1D_BUFFER, CL_MEM_OBJECT_IMAGE2D, CL_MEM_OBJECT_IMAGE2D_ARRAY, CL_MEM_OBJECT_IMAGE3D};

#[derive(Debug, Copy, Clone)]
pub enum ClImageType {
    Image1D,
    Image1DArray,
    Image1DBuffer,
    Image2D,
    Image2DArray,
    Image3D,
}

impl Into<u32> for ClImageType {
    fn into(self) -> u32 {
        match self {
            Self::Image1D => CL_MEM_OBJECT_IMAGE1D,
            Self::Image1DArray => CL_MEM_OBJECT_IMAGE1D_ARRAY,
            Self::Image1DBuffer => CL_MEM_OBJECT_IMAGE1D_BUFFER,
            Self::Image2D => CL_MEM_OBJECT_IMAGE2D,
            Self::Image2DArray => CL_MEM_OBJECT_IMAGE2D_ARRAY,
            Self::Image3D => CL_MEM_OBJECT_IMAGE3D
        }
    }
}
