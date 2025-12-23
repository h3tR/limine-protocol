//![Paging Mode Feature](https://codeberg.org/Limine/limine-protocol/src/branch/trunk/PROTOCOL.md#paging-mode-feature)
use crate::requests::LimineRequest;
use core::mem::MaybeUninit;
use crate::{group, impl_limine_req, LimineReqId};

#[cfg(target_arch = "x86_64")]
group! {
    pub const X86_64_4LVL: u64 = 0;
    pub const X86_64_5LVL: u64 = 1;

    pub const X86_64_DEFAULT_MODE: u64 = X86_64_4LVL;
    pub const X86_64_MIN_MODE: u64 = X86_64_4LVL;
}

#[cfg(target_arch = "aarch64")]
group! {
    pub const AARCH64_4LVL: u64 = 0;
    pub const AARCH64_5LVL: u64 = 1;

    pub const AARCH64_DEFAULT_MODE: u64 = AARCH64_4LVL;
    pub const AARCH64_MIN_MODE: u64 = AARCH64_4LVL;
}

#[cfg(target_arch = "riscv64")]
group! {
    pub const RISCV_SV39: u64 = 0;
    pub const RISCV_SV48: u64 = 1;
    pub const RISCV_SV57: u64 = 2;

    pub const RISCV64_DEFAULT_MODE: u64 = RISCV_SV48;
    pub const RISCV64_MIN_MODE: u64 = RISCV_SV39;
}

#[cfg(target_arch = "loongarch64")]
group! {
    pub const LOONGARCH_4LVL: u64 = 0;

    pub const LOONGARCH_DEFAULT_MODE: u64 = LOONGARCH_4LVL;
    pub const LOONGARCH_MIN_MODE: u64 = LOONGARCH_4LVL;
}



#[repr(C, align(8))]
pub struct PagingModeRequest{
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>,
    preferred_mode: u64,
    max_mode: u64,
    min_mode: u64
}

impl PagingModeRequest {
    pub const fn new(revision: u64, preferred_mode: u64, max_mode: u64, min_mode: u64) -> Self {
        Self {
            id: LimineReqId::new([0x95c1a0edab0944cb, 0xa4e5cb3842f7488a]),
            revision,
            resp: MaybeUninit::uninit(),
            preferred_mode,
            max_mode,
            min_mode
        }
    }
}

impl_limine_req!(PagingModeRequest, PagingModeResponse);

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct PagingModeResponse {
    revision: u64,
    pub mode: u64,
}