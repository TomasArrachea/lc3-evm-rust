use crate::memory::Memory;
use crate::Register;
use std::io::{stdout, Write};

pub fn putsp(reg: &mut [u16], memory: &mut Memory) {
    /* one char per byte (two bytes per word)
    here we need to swap back to
    big endian format */
    let mut c = memory.read(reg[Register::R0 as usize]);
    let mut i = 0;
    while c != 0 {
        let char1 = c & 0xFF;
        print!("{char1}");
        let char2 = c >> 8;
        if char2 != 0 {
            print!("{char2}");
        }
        c = memory.read(reg[Register::R0 as usize] + i);
        i += 1;
    }
    let _ = stdout().flush();
}