use crate::register::{update_flags, Register};
use super::sign_extend;

pub fn lea(reg: &mut [u16], instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    reg[r0 as usize] = reg[Register::Pc as usize] + pc_offset;
    update_flags(reg, r0);
}