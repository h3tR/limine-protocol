//![SMBIOS Feature](https://codeberg.org/Limine/limine-protocol/src/branch/trunk/PROTOCOL.md#smbios-feature)

use crate::requests::LimineRequest;
use core::mem::MaybeUninit;
use crate::{impl_limine_req, LimineReqId};

#[repr(C, align(8))]
pub struct SmbiosRequest {
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>
}

impl SmbiosRequest {
    pub const fn new(revision: u64) -> Self {
        Self {
            id: LimineReqId::new([0x9e9046f11e095391, 0xaa4a520fefbde5ee]),
            revision,
            resp: MaybeUninit::uninit()
        }
    }
}

impl_limine_req!(SmbiosRequest, SmbiosResponse);

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct SmbiosResponse {
    revision: u64,
    entry_32: usize,
    entry_64: usize
}

impl SmbiosResponse {
    pub fn get_entry_32(&self) -> Option<usize> {
        if self.entry_32 == 0 {
            return None;
        }
        Some(self.entry_32)
    }

    pub fn get_entry_64(&self) -> Option<usize> {
        if self.entry_64 == 0 {
            return None;
        }
        Some(self.entry_64)
    }
}