use core::fmt;

#[derive(Clone, Copy)]
pub struct SvmCapabilities {
    raw: u64,
}

impl From<u64> for SvmCapabilities {
    fn from(raw: u64) -> Self {
        Self { raw }
    }
}

impl SvmCapabilities {
    pub fn coarse_grain_buffer(&self) -> bool {
        self.raw & cl3::device::CL_DEVICE_SVM_COARSE_GRAIN_BUFFER != 0
    }

    pub fn fine_grain_buffer(&self) -> bool {
        self.raw & cl3::device::CL_DEVICE_SVM_FINE_GRAIN_BUFFER != 0
    }

    pub fn fine_grain_system(&self) -> bool {
        self.raw & cl3::device::CL_DEVICE_SVM_FINE_GRAIN_SYSTEM != 0
    }

    pub fn atomics(&self) -> bool {
        self.raw & cl3::device::CL_DEVICE_SVM_ATOMICS != 0
    }
}

impl fmt::Debug for SvmCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CoarseGrainBuffer: {}, FineGrainBuffer: {}, FineGrainSystem: {}, Atomics: {}",
            self.coarse_grain_buffer(),
            self.fine_grain_buffer(),
            self.fine_grain_system(),
            self.atomics(),
        )
    }
}

impl fmt::Display for SvmCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CoarseGrainBuffer: {}, FineGrainBuffer: {}, FineGrainSystem: {}, Atomics: {}",
            self.coarse_grain_buffer(),
            self.fine_grain_buffer(),
            self.fine_grain_system(),
            self.atomics(),
        )
    }
}