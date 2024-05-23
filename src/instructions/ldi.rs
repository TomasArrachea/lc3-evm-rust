use crate::memory::Memory;
use crate::register::update_flags;
use super::sign_extend;
use crate::Register;

pub fn ldi(reg: &mut [u16], instr: u16, memory: &mut Memory) {
    /* destination register (DR) */
    let r0: u16 = (instr >> 9) & 0x7;
    /* PCoffset 9*/
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);
    /* add pc_offset to the current PC, look at that memory location to get the final address */
    let addr = memory.read(reg[Register::Pc as usize] + pc_offset);
    reg[r0 as usize] = memory.read(addr);
    update_flags(reg, r0);
}