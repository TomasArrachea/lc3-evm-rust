use crate::memory::Memory;
use crate::register::update_flags;
use crate::Register;

pub fn getc(reg: &mut [u16], memory: &mut Memory) {
    /* read a single ASCII char */
    if let Ok(c) = memory.term.read_char() {
        reg[Register::R0 as usize] = c as u16;
    }
    update_flags(reg, Register::R0 as u16);
}