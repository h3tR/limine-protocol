//![MP (Multiprocessor) Feature](https://codeberg.org/Limine/limine-protocol/src/branch/trunk/PROTOCOL.md#mp-multiprocessor-feature)

use crate::requests::LimineRequest;
use core::mem::MaybeUninit;
use core::slice::from_raw_parts;
use crate::{impl_limine_req, LimineReqId};
use crate::requests::multiprocessor::arch::{CpuInfo, MultiprocessorResponse};
use crate::util::PointerSlice;

#[repr(C, align(8))]
pub struct MultiprocessorRequest{
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>,
    #[cfg(target_arch = "x86_64")]
    flags: u64
}

const MP_REQ_ID: LimineReqId = LimineReqId::new([0x95a67b819a1b857e, 0xa0b61b723b6a73e0]);
#[cfg(not(target_arch = "x86_64"))]
group! {
    impl MultiprocessorRequest {
        pub const fn new(revision: u64) -> Self {
            Self {
                id: MP_REQ_ID,
                revision,
                resp: MaybeUninit::uninit()
            }
        }
    }
}

impl_limine_req!(MultiprocessorRequest, MultiprocessorResponse);



#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
pub mod arch;

#[cfg(target_arch = "aarch64")]
#[path = "aarch64.rs"]
pub mod arch;

#[cfg(target_arch = "riscv64")]
#[path = "riscv64.rs"]
pub mod arch;

impl MultiprocessorResponse {
    pub fn get_cpus(&self) -> PointerSlice<CpuInfo> {
        PointerSlice::from(unsafe {
            from_raw_parts(self.cpus, self.cpu_count as usize)
        })
    }
}