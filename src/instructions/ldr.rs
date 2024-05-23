use crate::memory::Memory;
use crate::register::update_flags;
use super::sign_extend;

pub fn ldr(reg: &mut [u16], instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    reg[r0 as usize] = memory.read(reg[r1 as usize] + offset);
    update_flags(reg, r0);
}