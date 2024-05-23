use super::sign_extend;
use crate::Register;

pub fn branch(reg: &mut [u16], instr: u16) {
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let cond_flag = (instr >> 9) & 0x7;
    if cond_flag & reg[Register::Cond as usize] == 1 {
        reg[Register::Pc as usize] += pc_offset;
    }
}