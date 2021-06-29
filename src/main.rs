#![allow(dead_code, unused_variables)]

extern crate crossterm;
extern crate sdl2;

mod assembler;
mod optron;
mod qcpu;

use assembler::Assembler;
use optron::Optron;
use qcpu::QCPU;
use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

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
        let rc_optron: Rc<RefCell<Optron>> = Rc::new(RefCell::new(Optron::new()));
        let rc_cpu: Rc<RefCell<QCPU>> = Rc::new(RefCell::new(QCPU::new()));

        rc_optron.as_ref().borrow_mut().init();
        rc_cpu.as_ref().borrow_mut().load(&load_program(args));

        rc_cpu
            .as_ref()
            .borrow_mut()
            .bind_closure(0x06, Optron::closure_test(&rc_optron));
        rc_cpu
            .as_ref()
            .borrow_mut()
            .bind_closure(0x07, Optron::closure_test(&rc_optron));
        rc_cpu
            .as_ref()
            .borrow_mut()
            .bind_closure(0x15, Optron::closure_test(&rc_optron));
        rc_cpu
            .as_ref()
            .borrow_mut()
            .bind_closure(0x0B, Optron::closure_test(&rc_optron));
        rc_cpu
            .as_ref()
            .borrow_mut()
            .bind_closure(0x0C, Optron::closure_test(&rc_optron));

        loop {
            if rc_cpu.as_ref().borrow_mut().flags.halt == -1 {
                rc_cpu.as_ref().borrow_mut().step();
            }
            if !rc_optron.as_ref().borrow_mut().pump() {
                break;
            }
            rc_optron.as_ref().borrow_mut().display();
        }
    }
}
