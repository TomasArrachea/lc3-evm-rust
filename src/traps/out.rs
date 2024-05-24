use crate::Register;
use std::io::{stdout, Write};

pub fn out(reg: &mut [u16]) {
    print!("{}", reg[Register::R0 as usize] as u8 as char);
    let _ = stdout().flush();
}