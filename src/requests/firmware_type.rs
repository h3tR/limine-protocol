//![Firmware Type Feature](https://codeberg.org/Limine/limine-protocol/src/branch/trunk/PROTOCOL.md#firmware-type-feature)
use crate::requests::LimineRequest;
use crate::{impl_limine_req, LimineReqId};
use core::mem::MaybeUninit;

#[repr(C, align(8))]
pub struct FirmwareTypeRequest{
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>
}

impl FirmwareTypeRequest {
    pub const fn new(revision: u64) -> Self {
        Self {
            id: LimineReqId::new([0x8c2f75d90bef28a8, 0x7045a4688eac00c3]),
            revision,
            resp: MaybeUninit::uninit()
        }
    }
}

impl_limine_req!(FirmwareTypeRequest, FirmwareTypeResponse);

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct FirmwareTypeResponse {
    revision: u64,
    firmware_type: u64
}

impl FirmwareTypeResponse {
    pub fn get_type(&self) -> FirmwareType {
        FirmwareType::from(self.firmware_type)
    }
}

pub enum FirmwareType {
    X86Bios,
    EFI32,
    EFI64,
    SBI
}

impl From<u64> for FirmwareType {
    fn from(value: u64) -> Self {
        match value {
            0 => FirmwareType::X86Bios,
            1 => FirmwareType::EFI32,
            2 => FirmwareType::EFI64,
            3 => FirmwareType::SBI,
            _ => panic!("Invalid Firmware Type")
        }
    }
}