use core::ffi::{c_char, CStr};

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct LimineUUID(u16, u16, u16, [u8; 8]);

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct LimineFile {
    revision: u64,
    pub address: usize,
    pub size: u64,
    path: *const c_char,
    string: *const c_char,
    media_type: u32,
    _unused: u32,
    pub tftp_ip: u32,
    pub tfpt_port: u32,
    pub partition_index: u32,
    pub mbr_disk_id: u32,
    pub gpt_disk_uuid: LimineUUID,
    pub gpt_part_uuid: LimineUUID,
    pub part_uuid: LimineUUID
}

impl LimineFile {
    pub fn get_path(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.path).to_str().unwrap()
        }
    }

    pub fn get_string(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.string).to_str().unwrap()
        }
    }

    pub fn get_media_type(&self) -> MediaType {
        MediaType::from(self.media_type)
    }
}

pub enum MediaType {
    Generic,
    Optical,
    TFTP
}

impl From<u32> for MediaType {
    fn from(value: u32) -> Self {
        match value {
            0 => MediaType::Generic,
            1 => MediaType::Optical,
            2 => MediaType::TFTP,
            _ => panic!("Invalid media type")
        }
    }
}