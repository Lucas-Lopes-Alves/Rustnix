pub unsafe fn kstrlen(string: *const u8) -> usize {
    let mut len = 0;
    unsafe {
        while *string.add(len) != 0 {
            len += 1;
        }
    }
    len
}

pub unsafe fn kstrcmp(string1: &[u8], string2: &[u8]) -> isize {
    let len = if string1.len() < string2.len() {
        string1.len()
    } else {
        string2.len()
    };

    for i in 0..len {
        if string1[i] != string2[i] {
            return string1[i] as isize - string2[i] as isize;
        }
    }
    string1.len() as isize - string2.len() as isize
}
