pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    Pc, /* program counter */
    Cond,
    Count
}

pub enum ConditionFlags {
    Pos = 1 << 0, /* P */
    Zro = 1 << 1, /* Z */
    Neg = 1 << 2, /* N */
}

pub fn update_flags(reg: &mut [u16], r: u16) {
    let r_cond_idx = Register::Cond as usize;
    if reg[r as usize] == 0 {
        reg[r_cond_idx] = ConditionFlags::Zro as u16;
    } else if reg[r as usize] >> 15 == 1 {
        /* a 1 in the left-most bit indicates negative */
        reg[r_cond_idx] = ConditionFlags::Neg as u16;
    } else {
        reg[r_cond_idx] = ConditionFlags::Pos as u16;
    }
}