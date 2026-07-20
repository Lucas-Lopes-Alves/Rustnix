#[repr(C, packed)]
struct IdtPtr {
    limit: u16,
    base: u32,
}

#[repr(C, packed)]
struct IdtEntry {
    isr_low: u16,
    selector: u16,
    reserved: u8,
    attributes: u8,
    isr_high: u16,
}

impl IdtPtr {
    const fn new() -> Self {
        Self { limit: 0, base: 0 }
    }
}

impl IdtEntry {
    const fn new() -> Self {
        Self {
            isr_low: 0,
            selector: 0,
            reserved: 0,
            attributes: 0,
            isr_high: 0,
        }
    }
}
