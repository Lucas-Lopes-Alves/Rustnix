#![no_main]
#![no_std]

mod descriptors;
pub mod drivers;
pub mod functions;
mod interrupts;
pub mod io;
mod panic;

use crate::drivers::ps2_keyboard::keyboard::keyboard_char;
use crate::io::{inb, outb};
use core::arch::asm;
use descriptors::gdt::gdt_init;
use drivers::ps2_keyboard::keyboard;
use drivers::terminal::vga::{VgaColor, VgaWriter};

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() {
    gdt_init();
    let mut terminal = VgaWriter::new(VgaColor::White, VgaColor::Black);
    terminal.initialize();
    loop {
        unsafe {
            if (inb(0x64) & 0x01 != 0) {
                let code: u8 = inb(0x60);
                let str: u8 = keyboard_char(code);
                if (str == b'\0') {
                    continue;
                }
                terminal.putchar(str);
            }
        }
    }
}
