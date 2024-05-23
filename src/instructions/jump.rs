use crate::Register;

pub fn jump(reg: &mut [u16], instr: u16) {
    let r1 = (instr >> 6) & 0x7;
    reg[Register::Pc as usize] = reg[r1 as usize];
}