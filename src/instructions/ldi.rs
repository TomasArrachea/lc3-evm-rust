use console::Term;
use crate::register::update_flags;
use super::sign_extend;
use crate::Register;
use crate::mem_read;

pub fn ldi(reg: &mut [u16], instr: u16, memory: &mut [u16], term: &Term) {
    /* destination register (DR) */
    let r0: u16 = (instr >> 9) & 0x7;
    /* PCoffset 9*/
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);
    /* add pc_offset to the current PC, look at that memory location to get the final address */
    let addr = mem_read(memory, reg[Register::Pc as usize] + pc_offset, term);
    reg[r0 as usize] = mem_read(memory, addr, term);
    update_flags(reg, r0);
}