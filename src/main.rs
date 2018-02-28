#![allow(dead_code,unused_variables)]

extern crate crossterm;

mod qcpu;
use qcpu::QCPU;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = match File::open("./programs/bitcount") {
        Ok(file) => file,
        Err(e) => panic!("{:?}", e)
    };

    let mut data: Vec<u8> = Vec::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_end(&mut data).unwrap();

    let mut cpu = QCPU::new();
    cpu.load(&data);

    cpu.run();
}
