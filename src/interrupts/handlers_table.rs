use super::common_handler::Registers;
use crate::drivers::terminal::vga::VgaColor;
use crate::drivers::terminal::vga::VgaWriter;
use core::arch::asm;

pub static mut HANDLERS: [Option<fn(&mut Registers)>; 256] = {
    let mut a: [Option<fn(&mut Registers)>; 256] = [None; 256];
    a[0] = Some(divide_by_zero);
    a
};

fn divide_by_zero(r: &mut Registers) {
    let characters: &str = "ERROR,Division by zero!";
    let mut terminal = VgaWriter::new(VgaColor::LightGrey, VgaColor::Black);
    for i in characters.bytes() {
        terminal.putchar(i);
    }
    loop {
        unsafe {
            asm!("cli");
            asm!("hlt");
        }
    }
}
