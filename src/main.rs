mod instructions;
mod memory;
mod register;
mod traps;

use memory::Memory;
use register::{ConditionFlags, Register};
use std::env;
use std::fs::File;
use std::io::Read;
use std::process::exit;

const R_COUNT: usize = 10;

fn load_args(memory: &mut Memory) {
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

fn read_image(image_path: &str, memory: &mut Memory) -> i32 {
    if let Ok(f) = File::open(image_path) {
        read_image_file(f, memory);
        return 1;
    }
    0
}

fn read_image_file(mut file: File, memory: &mut Memory) {
    /* the origin tells us where in memory to place the image */
    let mut buffer = [0; 2];
    file.read(&mut buffer).unwrap();
    let origin = u16::from_be_bytes(buffer);

    /* use a heap allocated array as buffer */
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    /* store memory words from bytes */
    for (i, chunk) in buffer.chunks(2).enumerate() {
        memory.write(
            origin + i as u16,
            u16::from_be_bytes(chunk.try_into().unwrap()),
        );
    }
}

fn main() {
    let mut memory = Memory::new(); /* 65536 locations */
    let mut reg: [u16; R_COUNT] = [0; R_COUNT];
    load_args(&mut memory);

    /* since exactly one condition flag should be set at any given time, set the Z flag */
    reg[Register::Cond as usize] = ConditionFlags::Zro as u16;

    /* set the PC to starting position */
    /* 0x3000 is the default */
    let pc_start: u16 = 0x3000;
    reg[Register::Pc as usize] = pc_start;

    let mut running = true;
    while running {
        /* FETCH */
        let instr = memory.read(reg[Register::Pc as usize]);
        reg[Register::Pc as usize] += 1;
        instructions::opcode::execute(&mut reg, instr, &mut memory, &mut running);
    }
}
