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

const VGA_WIDTH: usize = 80;
// const VGA_HEIGHT: usize = 25;

pub struct VgaWriter {
    buffer: *mut u16,
    column: usize,
    row: usize,
    color: u8,
}

impl VgaWriter {
    pub fn new(text: VgaColor, background: VgaColor) -> Self {
        Self {
            buffer: 0xb8000 as *mut u16,
            column: 0,
            row: 0,
            color: (text as u8) | ((background as u8) << 4),
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
        let index = self.row * VGA_WIDTH + self.column;
        let character: u16 = ((self.color as u16) << 8) | c as u16;
        unsafe {
            self.buffer.add(index).write_volatile(character);
        }

        self.column += 1;
        if self.column >= VGA_WIDTH {
            self.row += 1;
            self.column = 0;
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for c in string.bytes() {
            self.putchar(c);
        }
    }
}
