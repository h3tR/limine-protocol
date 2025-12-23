//![Device Tree Blob Feature](https://codeberg.org/Limine/limine-protocol/src/branch/trunk/PROTOCOL.md#device-tree-blob-feature)

use crate::requests::LimineRequest;
use core::mem::MaybeUninit;
use crate::{impl_limine_req, LimineReqId};

#[repr(C, align(8))]
pub struct DeviceTreeBlobRequest {
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>
}

impl DeviceTreeBlobRequest {
    pub const fn new(revision: u64) -> Self {
        Self {
            id: LimineReqId::new([0xb40ddb48fb54bac7, 0x545081493f81ffb7]),
            revision,
            resp: MaybeUninit::uninit()
        }
    }
}

impl_limine_req!(DeviceTreeBlobRequest, DeviceTreeBlobResponse);

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct DeviceTreeBlobResponse {
    revision: u64,
    pub address: usize
}