use super::sign_extend;
use crate::{memory::Memory, register::Register};

pub fn sti(reg: &mut [u16], instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let addr = memory.read(reg[Register::Pc as usize] + pc_offset);
    memory.write(addr, reg[r0 as usize]);
}
