use super::keymap::*;
use crate::io::{inb, outb};

pub fn ps2_write_wait() {
    unsafe { while (inb(0x64) & 0x02 != 0) {} }
}

pub fn ps2_read_wait() {
    unsafe { while (!(inb(0x64) & 0x01) != 0) {} }
}

struct Locks {
    capslock: bool,
    scrollock: bool,
    numlock: bool,
}

static mut KBD_STATE: Locks = Locks {
    capslock: false,
    scrollock: false,
    numlock: false,
};

static mut SHIFT: bool = false;

static mut KEYMAP: &'static [u8; 128] = &CHARS;

pub fn keyboard_char(scancode: u8) -> u8 {
    if scancode == 0x2A || scancode == 0x36 {
        unsafe {
            SHIFT = true;
        }
        return b'\0';
    }

    if scancode == 0x2A + 0x80 || scancode == 0x36 + 0x80 {
        unsafe {
            SHIFT = false;
        }
        return b'\0';
    }

    if (scancode == 0x3A) {
        unsafe {
            KBD_STATE.capslock = !KBD_STATE.capslock;
            ps2_write_wait();
            outb(0x60, 0xED);
            inb(0x60);
            ps2_write_wait();
            outb(0x60, if KBD_STATE.capslock { 0x00 } else { 0x04 });
        }
        return b'\0';
    }

    if scancode & 0x80 != 0 {
        return b'\0';
    }

    unsafe {
        if (KBD_STATE.capslock != SHIFT) {
            KEYMAP = &CHARSALT;
        } else {
            KEYMAP = &CHARS;
        }

        return KEYMAP[scancode as usize];
    }
}
