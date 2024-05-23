use console::Term;

const MEMORY_MAX: usize = 1 << 16;

pub struct Memory {
    memory: [u16; MEMORY_MAX],
    term: Term,
}

enum MemoryRegisters {
    Kbsr = 0xFE00, /* keyboard status */
    Kbdr = 0xFE02, /* keyboard data */
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: [0; MEMORY_MAX],
            term: Term::stdout(),
        }
    }

    pub fn write(&mut self, address: u16, val: u16) {
        self.memory[address as usize] = val;
    }

    pub fn read(&mut self, address: u16) -> u16 {
        if address == MemoryRegisters::Kbsr as u16 {
            if let Ok(c) = self.term.read_char() {
                self.write(MemoryRegisters::Kbsr as u16, 1 << 15);
                self.write(MemoryRegisters::Kbdr as u16, c as u16);
            } else {
                self.write(MemoryRegisters::Kbsr as u16, 0);
            }
        }
        self.memory[address as usize]
    }
}
