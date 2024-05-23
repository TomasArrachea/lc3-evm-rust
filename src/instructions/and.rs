use crate::register::update_flags;
use super::sign_extend;

pub fn and(reg: &mut [u16], instr: u16) {
    let r0: u16 = (instr >> 9) & 0x7;
    let r1: u16 = (instr >> 6) & 0x7;
    let imm_flag: u16 = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        reg[r0 as usize] = reg[r1 as usize] & imm5;
    } else {
        let r2: u16 = instr & 0x7;
        reg[r0 as usize] = reg[r1 as usize] & reg[r2 as usize];
    }
    update_flags(reg, r0);
}
