use crate::requests::multiprocessor::MultiprocessorRequest;
use crate::requests::LimineRequest;
use crate::{impl_limine_req, LimineReqId};
use core::mem::MaybeUninit;

pub struct MultiprocessorResponse {
    revision: u64,
    flags: u64,
    pub bsp_hartid: u64,
    pub(super) cpu_count: u64,
    pub(super) cpus: *const *const CpuInfo
}

pub struct CpuInfo {
    pub processor_id: u32,
    pub hartid: u64,
    _reserved: u64,
    pub goto: usize,
    pub extra_args: u64
}

