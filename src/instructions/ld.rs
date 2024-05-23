use crate::memory::Memory;
use crate::register::update_flags;
use super::sign_extend;
use crate::Register;

pub fn ld(reg: &mut [u16], instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    reg[r0 as usize] = memory.read(reg[Register::Pc as usize] + pc_offset);
    update_flags(reg, r0);
}