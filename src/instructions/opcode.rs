use crate::memory::Memory;

use super::add::add;
use super::and::and;
use super::not::not;
use super::branch::branch;
use super::jmp::jmp;
use super::jsr::jsr;
use super::ld::ld;
use super::ldr::ldr;
use super::lea::lea;
use super::ldi::ldi;
use super::st::st;
use super::sti::sti;
use super::str::str;
use std::process::exit;

enum Opcode {
    OpBr = 0,   /* branch */
    OpAdd = 1,  /* add  */
    OpLd = 2,   /* load */
    OpSt = 3,   /* store */
    OpJsr = 4,  /* jump register */
    OpAnd = 5,  /* bitwise and */
    OpLdr = 6,  /* load register */
    OpStr = 7,  /* store register */
    OpRti = 8,  /* unused */
    OpNot = 9,  /* bitwise not */
    OpLdi = 10,  /* load indirect */
    OpSti = 11,  /* store indirect */
    OpJmp = 12,  /* jump */
    OpRes = 13,  /* reserved (unused) */
    OpLea = 14,  /* load effective address */
    OpTrap = 15, /* execute trap */
}

pub fn execute(reg: &mut [u16], instr: u16, memory: &mut Memory) {
    let op: u16 = instr >> 12;

    match op {
        x if x == Opcode::OpAdd as u16 => add(reg, instr),
        x if x == Opcode::OpAnd as u16 => and(reg, instr),
        x if x == Opcode::OpNot as u16 => not(reg, instr),
        x if x == Opcode::OpBr as u16 => branch(reg, instr),
        x if x == Opcode::OpJmp as u16 => jmp(reg, instr),
        x if x == Opcode::OpJsr as u16 => jsr(reg, instr),
        x if x == Opcode::OpLd as u16 => ld(reg, instr, memory),
        x if x == Opcode::OpLdi as u16 => ldi(reg, instr, memory),
        x if x == Opcode::OpLdr as u16 => ldr(reg, instr, memory),
        x if x == Opcode::OpLea as u16 => lea(reg, instr),
        x if x == Opcode::OpSt as u16 => st(reg, instr, memory),
        x if x == Opcode::OpSti as u16 => sti(reg, instr, memory),
        x if x == Opcode::OpStr as u16 => str(reg, instr, memory),
        x if x == Opcode::OpTrap as u16 => todo!(),
        x if x == Opcode::OpRes as u16 => (),
        x if x == Opcode::OpRti as u16 => (),
        _ => exit(3),
    }
}
