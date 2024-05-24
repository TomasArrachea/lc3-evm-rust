use super::getc::getc;
use super::puts::puts;
use super::out::out;
use super::t_in::t_in;
use super::putsp::putsp;
use super::halt::halt;
use crate::memory::Memory;
use crate::register::Register;

pub enum Trap {
    Getc = 0x20,  /* get character from keyboard, not echoed onto the terminal */
    Out = 0x21,   /* output a character */
    Puts = 0x22,  /* output a word string */
    In = 0x23,    /* get character from keyboard, echoed onto the terminal */
    Putsp = 0x24, /* output a byte string */
    Halt = 0x25   /* halt the program */
}

pub fn execute(reg: &mut [u16], instr: u16, memory: &mut Memory, running: &mut bool) {
    reg[Register::R7 as usize] = reg[Register::Pc as usize];

    // print!("<trap {:?}>", instr & 0xFF);

    match instr & 0xFF {
        x if x == Trap::Getc as u16 => getc(reg, memory),
        x if x == Trap::Out as u16 => out(reg),
        x if x == Trap::Puts as u16 => puts(reg, memory),
        x if x == Trap::In as u16 => t_in(reg, memory),
        x if x == Trap::Putsp as u16 => putsp(reg, memory),
        x if x == Trap::Halt as u16 => halt(running),
        _ => ()
    }
}