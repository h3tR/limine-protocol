//![Stack Size Feature](https://codeberg.org/Limine/limine-protocol/src/branch/trunk/PROTOCOL.md#stack-size-feature)

use crate::requests::LimineRequest;
use core::mem::MaybeUninit;
use crate::{impl_limine_req, LimineReqId};

#[repr(C, align(8))]
pub struct StackSizeRequest{
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>,
    stack_size: u64
}

impl StackSizeRequest {
    pub const fn new(revision: u64, stack_size: u64) -> Self {
        Self {
            id: LimineReqId::new([0x224ef0460a8e8926, 0xe1cb0fc25f46ea3d]),
            revision,
            resp: MaybeUninit::uninit(),
            stack_size
        }
    }
}

impl_limine_req!(StackSizeRequest, StackSizeResponse);

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct StackSizeResponse {
    revision: u64
}