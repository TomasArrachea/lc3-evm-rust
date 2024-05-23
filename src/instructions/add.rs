use crate::register::update_flags;

fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
    if (x >> (bit_count - 1)) & 1 == 1 {
        x |= 0xFFFF << bit_count;
    }
    x
}

pub fn add(reg: &mut [u16], instr: u16) {
    /* destination register (DR) */
    let r0: u16 = (instr >> 9) & 0x7;
    /* first operand (SR1) */
    let r1: u16 = (instr >> 6) & 0x7;
    /* whether we are in immediate mode */
    let imm_flag: u16 = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5: u16 = sign_extend(instr & 0x1F, 5);
        reg[r0 as usize] = reg[r1 as usize] + imm5;
    } else {
        let r2: u16 = instr & 0x7;
        reg[r0 as usize] = reg[r1 as usize] + reg[r2 as usize];
    }

    update_flags(reg, r0);
}
