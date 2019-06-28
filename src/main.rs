#![no_std]
#![no_main]

use core::panic::PanicInfo;
use r_efi::efi;

// Infinite loop on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[export_name = "efi_main"]
pub extern fn efi_main(handle: efi::Handle, sys_table: *mut efi::SystemTable) -> efi::Status {
    efi::Status::SUCCESS
}
