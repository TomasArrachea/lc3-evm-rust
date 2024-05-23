use super::sign_extend;
use crate::memory::Memory;

pub fn str(reg: &mut [u16], instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    memory.write(reg[r1 as usize] + offset, reg[r0 as usize]);
}
