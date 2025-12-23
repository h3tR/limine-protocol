use crate::requests::multiprocessor::{MultiprocessorRequest, MP_REQ_ID};
use core::mem::MaybeUninit;

impl MultiprocessorRequest {
    pub const fn new(revision: u64, enable_x2apic: bool) -> Self {
        Self {
            id: MP_REQ_ID,
            revision,
            resp: MaybeUninit::uninit(),
            flags: enable_x2apic as u64
        }
    }
}


pub struct MultiprocessorResponse {
    revision: u64,
    flags: u32,
    pub bsp_lapic_id: u32,
    pub(super) cpu_count: u64,
    pub(super) cpus: *const *const CpuInfo
}

impl MultiprocessorResponse {
    pub fn x2apic_enabled(&self) -> bool {
        self.flags & 1 == 1
    }
}

pub struct CpuInfo {
    pub processor_id: u32,
    pub lapid_id: u32,
    _reserved: u64,
    pub goto: usize,
    pub extra_args: u64
}

