//![EFI Memory Map Feature](https://codeberg.org/Limine/limine-protocol/src/branch/trunk/PROTOCOL.md#efi-memory-map-feature)

use crate::requests::LimineRequest;
use core::mem::MaybeUninit;
use crate::{impl_limine_req, LimineReqId};

#[repr(C, align(8))]
pub struct EfiMemoryMapRequest {
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>
}

impl EfiMemoryMapRequest {
    pub const fn new(revision: u64) -> Self {
        Self {
            id: LimineReqId::new([0x7df62a431d6872d5, 0xa4fcdfb3e57306c8]),
            revision,
            resp: MaybeUninit::uninit()
        }
    }
}

impl_limine_req!(EfiMemoryMapRequest, EfiMemoryMapResponse);

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct EfiMemoryMapResponse {
    revision: u64,
    pub address: usize,
    pub size: u64,
    pub descriptor_size: u64,
    pub descriptor_version: u64,
}