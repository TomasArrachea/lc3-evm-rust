use crate::memory::Memory;
use crate::Register;
use std::io::{stdout, Write};

pub fn puts(reg: &mut [u16], memory: &mut Memory) {
    /* one char per word */
    let mut c: u16 = memory.read(reg[Register::R0 as usize]);
    let mut i = 1;
    while c != 0 {
        print!("{}", c as u8 as char);
        c = memory.read(reg[Register::R0 as usize] + i);
        i += 1;
    }
    let _ = stdout().flush();
}