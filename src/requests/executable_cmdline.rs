//![Executable Command Line Feature](https://codeberg.org/Limine/limine-protocol/src/branch/trunk/PROTOCOL.md#executable-command-line-feature)
use crate::requests::LimineRequest;
use core::mem::MaybeUninit;
use crate::{impl_limine_req, LimineReqId};
use core::ffi::{c_char, CStr};

#[repr(C, align(8))]
pub struct ExecutableCmdlineRequest{
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>
}

impl ExecutableCmdlineRequest {
    pub const fn new(revision: u64) -> Self {
        Self {
            id: LimineReqId::new([0x4b161536e598651e, 0xb390ad4a2f1f303a]),
            revision,
            resp: MaybeUninit::uninit()
        }
    }
}

impl_limine_req!(ExecutableCmdlineRequest, ExecutableCmdlineResponse);

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct ExecutableCmdlineResponse {
    revision: u64,
    cmdline: *const u8,
}
impl ExecutableCmdlineResponse {
    pub fn get_cmdline(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.cmdline as *const c_char).to_str().unwrap()
        }
    }
}