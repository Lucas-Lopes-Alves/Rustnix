#![no_main]
#![no_std]

mod descriptors;
mod functions;
mod io;
mod terminal;

use core::arch::asm;

use descriptors::gdt::gdt_init;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() {
    gdt_init();
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
