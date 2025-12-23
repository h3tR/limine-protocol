//![Bootloader Performance Feature](https://codeberg.org/Limine/limine-protocol/src/branch/trunk/PROTOCOL.md#bootloader-performance-feature)

use crate::requests::LimineRequest;
use core::mem::MaybeUninit;
use crate::{impl_limine_req, LimineReqId};

#[repr(C, align(8))]
pub struct BootloaderPerformanceRequest {
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>
}

impl BootloaderPerformanceRequest {
    pub const fn new(revision: u64) -> Self {
        Self {
            id: LimineReqId::new([0x6b50ad9bf36d13ad, 0xdc4c7e88fc759e17]),
            revision,
            resp: MaybeUninit::uninit()
        }
    }
}

impl_limine_req!(BootloaderPerformanceRequest, BootloaderPerformanceResponse);

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct BootloaderPerformanceResponse {
    revision: u64,
    pub reset_usec: u64,
    pub init_usec: u64,
    pub exec_usec: u64
}