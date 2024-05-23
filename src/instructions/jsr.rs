use crate::Register;
use super::sign_extend;

pub fn jsr(reg: &mut [u16], instr: u16) {
    let long_flag = (instr >> 11) & 1;
    reg[Register::R7 as usize] = reg[Register::Pc as usize];
    if long_flag == 1 {
        let long_pc_offset = sign_extend(instr & 0x7FF, 11);
        reg[Register::Pc as usize] += long_pc_offset;  /* JSR */
    } else {
        let r1 = (instr >> 6) & 0x7;
        reg[Register::Pc as usize] = reg[r1 as usize]; /* JSRR */
    }
}