#![no_main]
#![no_std]

mod descriptors;
pub mod functions;
mod interrupts;
pub mod io;
pub mod terminal;

use core::arch::asm;
use descriptors::gdt::gdt_init;
use terminal::vga::*;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() {
    gdt_init();
    let mut terminal = VgaWriter::new(VgaColor::White, VgaColor::Black);
    terminal.initialize();
    terminal.write_string("ola");
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
