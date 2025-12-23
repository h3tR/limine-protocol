//![EFI System Table Feature](https://codeberg.org/Limine/limine-protocol/src/branch/trunk/PROTOCOL.md#efi-system-table-feature)

use crate::requests::LimineRequest;
use core::mem::MaybeUninit;
use crate::{impl_limine_req, LimineReqId};

#[repr(C, align(8))]
pub struct EfiSystemTableRequest {
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>
}

impl EfiSystemTableRequest {
    pub const fn new(revision: u64) -> Self {
        Self {
            id: LimineReqId::new([0xc5e77b6b397e7b43, 0x27637845accdcf3c]),
            revision,
            resp: MaybeUninit::uninit()
        }
    }
}

impl_limine_req!(EfiSystemTableRequest, EfiSystemTableResponse);

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct EfiSystemTableResponse {
    revision: u64,
    pub address: usize
}