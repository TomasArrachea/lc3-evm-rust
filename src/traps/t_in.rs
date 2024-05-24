use crate::{memory::Memory, Register};
use std::io::{stdout, Write};
use crate::register::update_flags;

pub fn t_in(reg: &mut [u16], memory: &mut Memory) {
    print!("Enter a character: ");
    if let Ok(c) = memory.term.read_char() {
        print!("{}", c);
        let _ = stdout().flush();
        reg[Register::R0 as usize] = c as u16;
        update_flags(reg, Register::R0 as u16);
    }
}