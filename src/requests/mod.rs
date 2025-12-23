pub mod memory_map;
pub mod framebuffer;
pub mod bootloader_info;
pub mod hhdm;
pub mod date_at_boot;
pub mod executable_cmdline;
pub mod firmware_type;
pub mod stack_size;
pub mod paging_mode;
#[cfg(target_arch = "riscv64")]
pub mod bsp_hart_id;
pub mod executable_file;
pub mod rsdp;
pub mod smbios;
pub mod efi_system_table;
pub mod efi_memory_map;
pub mod executable_address;
pub mod device_tree_blob;
pub mod bootloader_performance;

///Implements [LimineRequest] for $req. Requires \$req to have a resp field of the type core::mem::MaybeUninit\<usize\>.
#[macro_export]
macro_rules! impl_limine_req {
    ($req:ty, $resp:ty) => {
        impl LimineRequest for $req {
            type Response = $resp;

            fn get_response(&self) -> Option<&Self::Response> {
                unsafe {
                    if self.resp.assume_init() == 0 {
                       return None
                    }
                    (self.resp.assume_init() as *const Self::Response).as_ref()
                }
            }
        }
    };
}

pub trait LimineRequest {
    type Response;
    fn get_response(&self) -> Option<&Self::Response>;
}