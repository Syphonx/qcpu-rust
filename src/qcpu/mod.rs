mod opcode;
use qcpu::opcode::OpCode;

const MEMORY_SIZE: usize = 0xFFFF;

pub struct QCPU {
    pub memory: [u16; MEMORY_SIZE],
    pub pc: u16,
    pub registers: Registers,

}

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    x: u8,
    y: u8,
}

impl QCPU {
    pub fn new() -> QCPU { QCPU {
            memory: [0; MEMORY_SIZE],
            pc: 0,
            registers: Registers {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                x: 0,
                y: 0,
            },
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        if data.len() % 2 != 0 { panic!("data must be multiple of 2") }
        for i in 0..(data.len() / 2) {
            let byte1:u8 = data[i*2];
            let byte2:u8 = data[i*2 + 1];
        }
    }
}