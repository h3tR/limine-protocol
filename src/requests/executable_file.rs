//! [Executable File Feature](https://codeberg.org/Limine/limine-protocol/src/branch/trunk/PROTOCOL.md#executable-file-feature)

use crate::requests::LimineRequest;
use core::mem::MaybeUninit;
use crate::{impl_limine_req, LimineReqId};
use crate::file::LimineFile;

#[repr(C, align(8))]
pub struct ExecutableFileRequest{
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>
}

impl ExecutableFileRequest {
    pub const fn new(revision: u64) -> Self {
        Self {
            id: LimineReqId::new([0xad97e90e83f1ed67, 0x31eb5d1c5ff23b69]),
            revision,
            resp: MaybeUninit::uninit()
        }
    }
}

impl_limine_req!(ExecutableFileRequest, ExecutableFileResponse);

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct ExecutableFileResponse {
    revision: u64,
    executable_file: *const LimineFile
}

impl ExecutableFileResponse {
    pub fn get_executable_file(&self) -> &LimineFile {
        unsafe { self.executable_file.as_ref().unwrap() }
    }
}