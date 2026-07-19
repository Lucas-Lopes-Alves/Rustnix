pub enum VgaColor {
    VgaColorBlack = 0,
    VgaColorBlue = 1,
    VgaColorGreen = 2,
    VgaColorCyan = 3,
    VgaColorRed = 4,
    VgaColorMagenta = 5,
    VgaColorBrown = 6,
    VgaColorLightGrey = 7,
    VgaColorDarkGrey = 8,
    VgaColorLightBlue = 9,
    VgaColorLightGreen = 10,
    VgaColorLightCyan = 11,
    VgaColorLightRed = 12,
    VgaColorLightMagenta = 13,
    VgaColorYellow = 14,
    VgaColorWhite = 15,
}

struct VgaWriter {
    buffer: *mut u16,
    column: usize,
    row: usize,
    color: u8,
}

impl VgaWriter {
    pub fn new(color: VgaColor) -> Self {
        Self {
            buffer: 0xb8000 as *mut u16,
            column: 0,
            row: 0,
            color: color as u8,
        }
    }

    pub fn initialize(&self) {
        for _i in 0..(80 * 25) {
            self.putchar(b' ');
        }
    }

    pub fn putchar(&mut self, c: u8) {
        let index = self.row * 80 + self.row;
        let character: u16 = ((self.color as u16) << 8) | c as u16;
        unsafe {
            self.buffer.add(index).write_volatile(character);
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for c in string.bytes() {
            self.putchar(c);
        }
    }
}
