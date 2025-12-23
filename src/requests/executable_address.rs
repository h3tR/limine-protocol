//![Executable Address Feature](https://codeberg.org/Limine/limine-protocol/src/branch/trunk/PROTOCOL.md#executable-address-feature)

use crate::requests::LimineRequest;
use core::mem::MaybeUninit;
use crate::{impl_limine_req, LimineReqId};

#[repr(C, align(8))]
pub struct ExecutableAddressRequest {
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>
}

impl ExecutableAddressRequest {
    pub const fn new(revision: u64) -> Self {
        Self {
            id: LimineReqId::new([0x71ba76863cc55f63, 0xb2644a48c516a487]),
            revision,
            resp: MaybeUninit::uninit()
        }
    }
}

impl_limine_req!(ExecutableAddressRequest, ExecutableAddressResponse);

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct ExecutableAddressResponse {
    revision: u64,
    pub physical_base: usize,
    pub virtual_base: usize
}