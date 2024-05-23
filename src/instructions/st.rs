use super::sign_extend;
use crate::{memory::Memory, register::Register};

pub fn st(reg: &mut [u16], instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    memory.write(reg[Register::Pc as usize] + pc_offset, reg[r0 as usize]);
}
