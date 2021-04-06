mod cpu;
mod bus;
mod debug;

use std::fs::File;
use std::env::args;
use crate::cpu::CPU;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("A file is required for loading.");
    }

    let mut cpu = CPU::new();
    let file = File::open(&args[1])?;

    cpu.load_code(file)?;

    while !cpu.halt {
        cpu.run_instr();
    }

    println!("{:?}", cpu);

    Ok(())
}
