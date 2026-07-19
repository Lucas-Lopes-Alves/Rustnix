// use core::ptr::{addr_of, addr_of_mut, write};

#[repr(C)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl GdtEntry {
    pub const fn new() -> Self {
        Self {
            limit_low: 0,
            base_low: 0,
            base_middle: 0,
            access: 0,
            granularity: 0,
            base_high: 0,
        }
    }
}

#[repr(C, packed)]
struct GdtPtr {
    limit: u16,
    base: u32,
}

impl GdtPtr {
    pub const fn new() -> Self {
        return Self { limit: 0, base: 0 };
    }
}

static mut GDTR: GdtPtr = GdtPtr::new();

static mut GDT: [GdtEntry; 3] = [GdtEntry::new(), GdtEntry::new(), GdtEntry::new()];

unsafe extern "C" {
    fn gdt_load(gdt_ptr: *const GdtPtr);
}

fn gdt_set_entry(limit: u32, base: u32, access: u8, flags: u8) -> GdtEntry {
    GdtEntry {
        base_low: (base & 0xffff) as u16,
        base_middle: ((base >> 16) & 0xff) as u8,
        base_high: ((base >> 24) & 0xff) as u8,
        limit_low: (limit & 0xffff) as u16,
        granularity: (((limit >> 16) & 0x0f) as u8) | ((flags << 4) as u8),
        access: access,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gdt_init() -> () {
    unsafe {
        GDTR.limit = (core::mem::size_of::<[GdtEntry; 3]>() - 1) as u16;
        GDTR.base = core::ptr::addr_of!(GDT) as u32;

        // let gdt = addr_of_mut!(GDT).cast::<GdtEntry>();
        // write(gdt.add(0), gdt_set_entry(0, 0, 0, 0));
        // write(gdt.add(1), gdt_set_entry(0x000fffff, 0x0, 0x9a, 0x0c));
        // write(gdt.add(2), gdt_set_entry(0x000fffff, 0x0, 0x92, 0x0c));

        GDT = [
            gdt_set_entry(0, 0, 0, 0),
            gdt_set_entry(0x000fffff, 0x0, 0x9a, 0x0c),
            gdt_set_entry(0x000fffff, 0x0, 0x92, 0x0c),
        ];

        gdt_load(&raw mut GDTR);
    }
}
