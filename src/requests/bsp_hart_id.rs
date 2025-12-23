//![RISC-V BSP Hart ID Feature](https://codeberg.org/Limine/limine-protocol/src/branch/trunk/PROTOCOL.md#risc-v-bsp-hart-id-feature)

use crate::requests::LimineRequest;
use core::mem::MaybeUninit;
use crate::{impl_limine_req, LimineReqId};

#[repr(C, align(8))]
pub struct BspHartIdRequest {
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>
}

impl BspHartIdRequest {
    pub const fn new(revision: u64) -> Self {
        Self {
            id: LimineReqId::new([0xf55038d8e2a1202f, 0x279426fcf5f59740]),
            revision,
            resp: MaybeUninit::uninit()
        }
    }
}

impl_limine_req!(BspHartIdRequest, BspHartIdResponse);

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct BspHartIdResponse {
    revision: u64,
    pub bsp_hart_id: u64
}