mod instructions;
mod register;

use console::Term;
use register::{ConditionFlags, Register};
use std::env;
use std::fs::File;
use std::io::Read;
use std::process::exit;

const MEMORY_MAX: usize = 1 << 16;
const R_COUNT: usize = 10;

fn load_args(memory: &mut [u16]) {
    if env::args().len() < 2 {
        /* show usage string */
        println!("lc3 [image-file1] ...");
        exit(2);
    }

    for arg in env::args().skip(1) {
        if read_image(&arg, memory) == 0 {
            println!("failed to load image: {}", arg);
            exit(1);
        }
    }
}

fn read_image_file(mut file: File, memory: &mut [u16]) {
    /* the origin tells us where in memory to place the image */
    let mut buffer = [0; 2];
    file.read(&mut buffer).unwrap();
    let origin = u16::from_be_bytes(buffer) as usize;

    /* use a heap allocated array as buffer */
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    /* store memory words from bytes */
    for (i, chunk) in buffer.chunks(2).enumerate() {
        memory[origin + i] = u16::from_be_bytes(chunk.try_into().unwrap());
    }
}

fn read_image(image_path: &str, memory: &mut [u16]) -> i32 {
    if let Ok(f) = File::open(image_path) {
        read_image_file(f, memory);
        return 1;
    }
    0
}

// Memory Mapped Registers
enum MemoryRegisters {
    Kbsr = 0xFE00, /* keyboard status */
    Kbdr = 0xFE02, /* keyboard data */
}

// memory access
fn mem_write(memory: &mut [u16], address: u16, val: u16) {
    memory[address as usize] = val;
}

fn mem_read(memory: &mut [u16], address: u16, term: &Term) -> u16 {
    if address == MemoryRegisters::Kbsr as u16 {
        if let Ok(c) = term.read_char() {
            memory[MemoryRegisters::Kbsr as usize] = 1 << 15;
            memory[MemoryRegisters::Kbdr as usize] = c as u16;
        } else {
            memory[MemoryRegisters::Kbsr as usize] = 0;
        }
    }
    memory[address as usize]
}

fn main() {
    let mut memory: [u16; MEMORY_MAX] = [0; MEMORY_MAX]; /* 65536 locations */
    let mut reg: [u16; R_COUNT] = [0; R_COUNT];
    let term = Term::stdout();

    load_args(&mut memory);

    /* since exactly one condition flag should be set at any given time, set the Z flag */
    reg[Register::Cond as usize] = ConditionFlags::Zro as u16;

    /* set the PC to starting position */
    /* 0x3000 is the default */
    let pc_start: u16 = 0x3000;
    reg[Register::Pc as usize] = pc_start;

    loop {
        /* FETCH */
        reg[Register::Pc as usize] += 1;
        let instr = mem_read(&mut memory, reg[Register::Pc as usize], &term);
        instructions::opcode::execute(&mut reg, instr);
    }
}
