use super::common_handler::Registers;
use crate::drivers::terminal::vga::VgaColor;
use crate::drivers::terminal::vga::VgaWriter;
use core::arch::asm;

pub static mut HANDLERS: [Option<fn(&Registers)>; 256] = {
    let mut a: [Option<fn(&Registers)>; 256] = [None; 256];
    a[0] = Some(divide_by_zero);
    return a;
};

fn divide_by_zero(r: &Registers) {
    let characters: &str = "ERROR,Division by zero!";
    let mut count: usize = 0;
    let terminal = VgaWriter::new(VgaColor::LightGrey, VgaColor::Black);
    while (characters[count]) {
        terminal.putchar(characters[count]);
        count += 1;
    }
    loop {
        unsafe {
            asm!("cli");
            asm!("hlt");
        }
    }
}
