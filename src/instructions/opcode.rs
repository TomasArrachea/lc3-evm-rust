use crate::memory::Memory;

use super::add::add;
use super::and::and;
use super::not::not;
use super::ldi::ldi;
use std::process::exit;

enum Opcode {
    OpBr,   /* branch */
    OpAdd,  /* add  */
    OpLd,   /* load */
    OpSt,   /* store */
    OpJsr,  /* jump register */
    OpAnd,  /* bitwise and */
    OpLdr,  /* load register */
    OpStr,  /* store register */
    OpRti,  /* unused */
    OpNot,  /* bitwise not */
    OpLdi,  /* load indirect */
    OpSti,  /* store indirect */
    OpJmp,  /* jump */
    OpRes,  /* reserved (unused) */
    OpLea,  /* load effective address */
    OpTrap, /* execute trap */
}

pub fn execute(reg: &mut [u16], instr: u16, memory: &mut Memory) {
    let op: u16 = instr >> 12;

    match op {
        x if x == Opcode::OpAdd as u16 => add(reg, instr),
        x if x == Opcode::OpAnd as u16 => and(reg, instr),
        x if x == Opcode::OpNot as u16 => not(reg, instr),
        x if x == Opcode::OpBr as u16 => todo!(),
        x if x == Opcode::OpJmp as u16 => todo!(),
        x if x == Opcode::OpJsr as u16 => todo!(),
        x if x == Opcode::OpLd as u16 => todo!(),
        x if x == Opcode::OpLdi as u16 => ldi(reg, instr, memory),
        x if x == Opcode::OpLdr as u16 => todo!(),
        x if x == Opcode::OpLea as u16 => todo!(),
        x if x == Opcode::OpSt as u16 => todo!(),
        x if x == Opcode::OpSti as u16 => todo!(),
        x if x == Opcode::OpStr as u16 => todo!(),
        x if x == Opcode::OpTrap as u16 => todo!(),
        x if x == Opcode::OpRes as u16 => (),
        x if x == Opcode::OpRti as u16 => (),
        _ => exit(3),
    }
}
