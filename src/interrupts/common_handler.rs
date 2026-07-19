use super::handlers_table::*;
pub struct Registers {
    edi: u32,
    esi: u32,
    ebp: u32,
    esp: u32,
    ebx: u32,
    edx: u32,
    ecx: u32,
    eax: u32,
    int_no: u32,
    err_code: u32,
    eip: u32,
}

#[unsafe(no_mangle)]
pub extern "C" fn common_handler(r: &Registers) -> () {
    unsafe {
        if (HANDLERS[r.int_no].is_some()) {
            HANDLERS[r.int_no]();
        }
    }
}
