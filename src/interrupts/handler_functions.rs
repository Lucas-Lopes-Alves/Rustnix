//ISR 0
pub fn divide_by_zero(r: &mut Registers) {
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
