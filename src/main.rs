mod qcpu;
use qcpu::QCPU;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = match File::open("./programs/testbench") {
        Ok(file) => file,
        Err(E) => panic!("{:?}", E)
    };

    let mut data: Vec<u8> = Vec::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_end(&mut data);

    let mut cpu = QCPU::new();
    cpu.load(&data);

    println!("{:?}", &cpu.memory[..8650]);
}
