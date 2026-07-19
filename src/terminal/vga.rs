use core::str::Bytes;

pub enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}

pub struct VgaWriter {
    buffer: *mut u16,
    column: usize,
    row: usize,
    color: u8,
}

impl VgaWriter {
    pub fn new(fg: VgaColor, bg: VgaColor) -> Self {
        Self {
            buffer: 0xb8000 as *mut u16,
            column: 0,
            row: 0,
            color: fg as u8 | ((bg as u8) << 4) as u8,
        }
    }

    pub fn initialize(&mut self) {
        for _ in 0..(80 * 25) {
            self.putchar(b' ');
        }

        self.column = 0;
        self.row = 0;
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
