mod cpu;

use std::{env, process::exit};

use cpu::Cpu;

fn main() {
    if env::args().len() != 2 {
        println!("wrong number of arguments: arcm <file>");
        exit(1);
    }

    let mut cpu: Cpu = Cpu::new(env::args().nth(1).unwrap().as_str());

    cpu.execute();
}