#![allow(dead_code, unused_variables)]

extern crate crossterm;
extern crate sdl2;

mod assembler;
mod optron;
mod qcpu;

use assembler::Assembler;
use optron::Optron;
use qcpu::QCPU;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn is_assemble(args: Vec<String>) -> bool {
    if args.len() >= 2 {
        return args[1].eq("assemble");
    } else {
        return false;
    }
}

fn load_program(args: Vec<String>) -> Vec<u8> {
    let mut query: &str = "./programs/testbench";
    //println!("{:?}", args);

    if args.len() >= 2 {
        query = &args[1];
    }
    let mut file = match File::open(query) {
        Ok(file) => file,
        Err(e) => panic!("{:?}", e),
    };

    let mut data: Vec<u8> = Vec::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_end(&mut data).unwrap();

    return data;
}

pub struct Host<'a> {
    pub cpu: QCPU<'a>,
    pub optron: Optron,
}

impl Host<'_> {
    pub fn new<'a>() -> Host<'a> {
        Host {
            cpu: QCPU::new(),
            optron: Optron::new(),
        }
    }

    pub fn init(&mut self, args: Vec<String>) {
        self.optron.init();
        self.cpu.load(&load_program(args));
        //self.cpu.bind_closure(0x15, self.optron.closure_test());
        // self.bind();
    }

    pub fn run(&mut self) {
        loop {
            if self.cpu.flags.halt == -1 {
                self.cpu.step();
            }
            if !self.optron.pump() {
                break;
            }
            self.optron.display();
        }
    }

    fn bind(&mut self) {
        // self.cpu.bind(0x06, Host::sys_print);
        // self.cpu.bind(0x07, Host::sys_read);
        // self.cpu.bind(0x15, Host::sys_dummy);
        // self.cpu.bind(0x0B, Host::sys_fgc);
        // self.cpu.bind(0x0C, Host::sys_bgc);
    }

    fn sys_dummy(cpu: &mut QCPU, args: &qcpu::OpArgs) {}

    fn sys_print(cpu: &mut QCPU, args: &qcpu::OpArgs) {
        print!(
            "{}",
            cpu.colorise_string(
                std::char::from_u32(cpu.registers.x as u32)
                    .unwrap()
                    .to_string()
            )
        );
    }

    fn sys_read(cpu: &mut QCPU, args: &qcpu::OpArgs) {
        cpu.registers.x = std::io::stdin().bytes().next().unwrap().unwrap() as u16;
    }

    fn sys_fgc(cpu: &mut QCPU, args: &qcpu::OpArgs) {
        cpu.current_fg_color = cpu.registers.x;
    }

    fn sys_bgc(cpu: &mut QCPU, args: &qcpu::OpArgs) {
        cpu.current_bg_color = cpu.registers.x;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if is_assemble(args.clone()) {
        let mut avengers: Assembler = Assembler::new();
        if args.len() < 4 {
            println!(
                "Error: invalid assemble args: e.g. qcpu assemble ./inputfile.qasm ./outputfile.qasm"
            )
        } else {
            avengers.assemble(&args[2], &args[3]);
        }
    } else {
        let mut optron: Optron = Optron::new();
        let mut cpu: QCPU = QCPU::new();
        optron.init();
        cpu.load(&load_program(args));
        cpu.bind_closure(0x06, optron.closure_test());
        cpu.bind_closure(0x07, optron.closure_test());
        cpu.bind_closure(0x15, optron.closure_test());
        cpu.bind_closure(0x0B, optron.closure_test());
        cpu.bind_closure(0x0C, optron.closure_test());
        // self.cpu.bind(0x06, Host::sys_print);
        // self.cpu.bind(0x07, Host::sys_read);
        // self.cpu.bind(0x15, Host::sys_dummy);
        // self.cpu.bind(0x0B, Host::sys_fgc);
        // self.cpu.bind(0x0C, Host::sys_bgc);
        // let mut host: Host = Host::new();
        // host.init(args);
        // host.run();
    }
}
