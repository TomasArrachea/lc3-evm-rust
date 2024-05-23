pub mod add;
pub mod and;
pub mod not;
pub mod branch;
pub mod jump;
pub mod ldi;
pub mod opcode;

fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
    if (x >> (bit_count - 1)) & 1 == 1 {
        x |= 0xFFFF << bit_count;
    }
    x
}