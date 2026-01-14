pub enum MemoryFlags {
    ReadWrite,
    WriteOnly,
    ReadOnly,
    UseHostPtr,
    AllocHostPtr,
    CopyHostPtr,
    HostWriteOnly,
    HostReadOnly,
    HostNoAccess,
    KernelReadAndWrite,
}

impl MemoryFlags {
    pub fn to_u64(flags: &Vec<MemoryFlags>) -> u64 {
        let mut carry = 0;
        for f in flags {
            carry |= MemoryFlags::get_u64(f);
        }
        carry
    }

    pub fn get_u64(flag: &MemoryFlags) -> u64 {
        match flag {
            Self::ReadWrite => cl3::memory::CL_MEM_READ_WRITE,
            Self::WriteOnly => cl3::memory::CL_MEM_WRITE_ONLY,
            Self::ReadOnly => cl3::memory::CL_MEM_READ_ONLY,
            Self::UseHostPtr => cl3::memory::CL_MEM_USE_HOST_PTR,
            Self::AllocHostPtr => cl3::memory::CL_MEM_ALLOC_HOST_PTR,
            Self::CopyHostPtr => cl3::memory::CL_MEM_COPY_HOST_PTR,
            Self::HostWriteOnly => cl3::memory::CL_MEM_HOST_WRITE_ONLY,
            Self::HostReadOnly => cl3::memory::CL_MEM_HOST_READ_ONLY,
            Self::HostNoAccess => cl3::memory::CL_MEM_HOST_NO_ACCESS,
            Self::KernelReadAndWrite => cl3::memory::CL_MEM_KERNEL_READ_AND_WRITE,
        }
    }
}
