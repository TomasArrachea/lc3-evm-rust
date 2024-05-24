use std::io::{stdout, Write};

pub fn halt(running: &mut bool) {
    print!("HALT");
    let _ = stdout().flush();
    *running = false;
}