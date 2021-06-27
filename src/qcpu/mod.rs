mod opcode;

use crossterm::style::{paint, Color, StyledObject};

use qcpu::opcode::AddressingMode;
use qcpu::opcode::OpCode;

use std::collections::HashMap;

pub struct OpArgs {
    value: u16,
    mode: AddressingMode,
}

const MEMORY_SIZE: usize = 0xFFFF; // 65535

pub struct QCPU {
    pub memory: [u16; MEMORY_SIZE],
    pub pc: usize,
    pub registers: Registers,
    pub flags: Flags,
    pub call_stack: Vec<usize>,
    pub stack: Vec<u16>,
    pub syscalls: HashMap<u16, fn(&mut QCPU, &OpArgs)>,
    pub current_fg_color: u16,
    pub current_bg_color: u16,
}

pub struct Flags {
    pub halt: i16,
}

pub struct Registers {
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub d: u16,
    pub x: u16,
    pub y: u16,
}

impl QCPU {
    pub fn new() -> QCPU {
        QCPU {
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
            flags: Flags { halt: -1 },
            call_stack: Vec::<usize>::new(),
            stack: Vec::<u16>::new(),
            syscalls: HashMap::new(),

            current_fg_color: 0,
            current_bg_color: 7,
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        if data.len() % 2 != 0 {
            panic!("data must be multiple of 2")
        }
        for i in 0..(data.len() / 2) {
            let byte1: u8 = data[i * 2];
            let byte2: u8 = data[i * 2 + 1];

            self.memory[i] = ((byte2 as u16) << 8) + (byte1 as u16)
        }
    }

    pub fn bind(&mut self, key: u16, func: fn(&mut QCPU, &OpArgs)) {
        self.syscalls.insert(key, func);
    }

    pub fn print(&mut self) {
        while self.pc < self.memory.len() {
            let mut s = String::new();
            s.push_str(&self.pc.to_string());
            s.push_str(" ");

            let current = self.memory[self.pc];
            let op_part = current & 0x00FF;
            let addr_part = (current & 0xFF00) >> 8;
            let opcode = OpCode::from_int(op_part);

            let arity = (&opcode).arity();
            s.push_str(&(&opcode).to_string());
            s.push_str(" ");

            let addressing_modes = AddressingMode::map_from_int(addr_part);
            s.push_str(
                &(match arity {
                    0 => format!(""),
                    1 => format!("{}", addressing_modes.0.to_string()),
                    2 => format!(
                        "{}{}",
                        addressing_modes.0.to_string(),
                        addressing_modes.1.to_string()
                    ),
                    3 => format!(
                        "{}{}{}",
                        addressing_modes.0.to_string(),
                        addressing_modes.1.to_string(),
                        addressing_modes.2.to_string()
                    ),
                    4 => format!(
                        "{}{}{}{}",
                        addressing_modes.0.to_string(),
                        addressing_modes.1.to_string(),
                        addressing_modes.2.to_string(),
                        addressing_modes.3.to_string()
                    ),
                    _ => panic!("arity > 4"),
                }),
            );
            self.pc += 1;

            let arguments = &self.memory[self.pc..self.pc + arity as usize];

            for arg in arguments.iter() {}

            self.pc += arity as usize;

            println!("{}", s);
        }
    }

    pub fn step(&mut self) {
        let current = self.memory[self.pc];
        let op_part = current & 0x00FF;
        let addr_part = (current & 0xFF00) >> 8;
        let opcode = OpCode::from_int(op_part);

        let arity = (&opcode).arity();
        let addressing_modes = AddressingMode::map_from_int(addr_part);

        self.pc += 1;

        let zip = self.zip_args(
            &self.memory[self.pc..self.pc + arity as usize + 1],
            addressing_modes,
        );

        self.pc += arity as usize;
        self.execute_op(opcode, zip);
    }

    pub fn write(&mut self, to: &OpArgs, val: u16) {
        match to.mode {
            AddressingMode::IMMEDIATE => panic!("cannot write to immediate value {}", to.value),
            AddressingMode::ABSOLUTE => self.memory[to.value as usize] = val,
            AddressingMode::INDIRECT => {
                self.memory[self.read(&OpArgs {
                    value: to.value,
                    mode: AddressingMode::REGISTER,
                }) as usize] = val
            }
            AddressingMode::REGISTER => self.write_reg(to.value, val),
        }
    }

    pub fn write_reg(&mut self, to: u16, val: u16) {
        match to {
            0 => self.registers.a = val,
            1 => self.registers.b = val,
            2 => self.registers.c = val,
            3 => self.registers.d = val,
            4 => self.registers.x = val,
            5 => self.registers.y = val,
            _ => panic!("unknown register {}", to),
        }
    }

    pub fn read(&self, from: &OpArgs) -> u16 {
        match from.mode {
            AddressingMode::IMMEDIATE => from.value,
            AddressingMode::ABSOLUTE => self.memory[from.value as usize],
            AddressingMode::INDIRECT => {
                self.memory[self.read(&OpArgs {
                    value: from.value,
                    mode: AddressingMode::REGISTER,
                }) as usize]
            }
            AddressingMode::REGISTER => self.read_reg(from.value),
        }
    }

    pub fn read_reg(&self, from: u16) -> u16 {
        match from {
            0 => self.registers.a,
            1 => self.registers.b,
            2 => self.registers.c,
            3 => self.registers.d,
            4 => self.registers.x,
            5 => self.registers.y,
            _ => panic!("unknown register {}", from),
        }
    }

    pub fn colorise_string(&self, s: String) -> StyledObject<String> {
        paint(s)
            .with(match self.current_fg_color {
                0 => Color::White,
                1 => Color::Green,
                2 => Color::Red,
                3 => Color::Yellow,
                4 => Color::Blue,
                5 => Color::Magenta,
                6 => Color::Cyan,
                7 => Color::Black,
                8 => Color::Grey,
                _ => panic!("unknown color {}", self.current_fg_color),
            })
            .on(match self.current_bg_color {
                0 => Color::White,
                1 => Color::Green,
                2 => Color::Red,
                3 => Color::Yellow,
                4 => Color::Blue,
                5 => Color::Magenta,
                6 => Color::Cyan,
                7 => Color::Black,
                8 => Color::Grey,
                _ => panic!("unknown color {}", self.current_bg_color),
            })
    }

    fn zip_args(
        &self,
        arguments: &[u16],
        modes: (
            AddressingMode,
            AddressingMode,
            AddressingMode,
            AddressingMode,
        ),
    ) -> Vec<OpArgs> {
        let mut v = Vec::<OpArgs>::new();

        for i in 0..arguments.len() {
            match i {
                0 => {}
                1 => v.push(OpArgs {
                    value: arguments[0],
                    mode: modes.0,
                }),
                2 => v.push(OpArgs {
                    value: arguments[1],
                    mode: modes.1,
                }),
                3 => v.push(OpArgs {
                    value: arguments[2],
                    mode: modes.2,
                }),
                4 => v.push(OpArgs {
                    value: arguments[3],
                    mode: modes.3,
                }),
                _ => panic!("unallowed number of arguments {}/{}", i, arguments.len()),
            }
        }

        v
    }

    pub fn execute_op(&mut self, opcode: OpCode, arguments: Vec<OpArgs>) {
        //println!("{}: {} -> {}", self.pc, opcode, arguments.len());
        //println!("{:?}", &self.memory[self.pc .. self.pc + 8]);
        match opcode {
            OpCode::NOP => self.nop(),
            OpCode::EXT => self.ext(&arguments[0]),
            OpCode::SYS => self.sys(&arguments[0]),
            OpCode::MOV => self.mov(&arguments[0], &arguments[1]),
            OpCode::JMP => self.jmp(&arguments[0]),
            OpCode::JEQ => self.jeq(&arguments[0], &arguments[1], &arguments[2]),
            OpCode::JNE => self.jne(&arguments[0], &arguments[1], &arguments[2]),
            OpCode::JGT => self.jgt(&arguments[0], &arguments[1], &arguments[2]),
            OpCode::JGE => self.jge(&arguments[0], &arguments[1], &arguments[2]),
            OpCode::JLT => self.jlt(&arguments[0], &arguments[1], &arguments[2]),
            OpCode::JLE => self.jle(&arguments[0], &arguments[1], &arguments[2]),
            OpCode::JSR => self.jsr(&arguments[0]),
            OpCode::RET => self.ret(),
            OpCode::ADD => self.add(&arguments[0], &arguments[1]),
            OpCode::SUB => self.sub(&arguments[0], &arguments[1]),
            OpCode::MUL => self.mul(&arguments[0], &arguments[1]),
            OpCode::MDL => self.mdl(&arguments[0], &arguments[1]),
            OpCode::AND => self.and(&arguments[0], &arguments[1]),
            OpCode::ORR => self.orr(&arguments[0], &arguments[1]),
            OpCode::NOT => self.not(&arguments[0]),
            OpCode::XOR => self.xor(&arguments[0], &arguments[1]),
            OpCode::LSL => self.lsl(&arguments[0], &arguments[1]),
            OpCode::LSR => self.lsr(&arguments[0], &arguments[1]),
            OpCode::PSH => self.psh(&arguments[0]),
            OpCode::POP => self.pop(&arguments[0]),
        }
    }

    // 0
    pub fn nop(&mut self) {
        // do nothing
    }

    // 1
    pub fn ext(&mut self, code: &OpArgs) {
        self.flags.halt = self.read(code) as i16;
    }

    // 2
    pub fn sys(&mut self, val: &OpArgs) {
        match self.syscalls.get(&self.read(val)) {
            Some(sys) => {
                sys(self, val);
            }
            None => {
                panic!("unknown syscode {}", self.registers.x)
            }
        }
    }

    // 3
    pub fn mov(&mut self, to: &OpArgs, from: &OpArgs) {
        let read = self.read(from);
        self.write(to, read);
    }

    // 4
    pub fn jmp(&mut self, addr: &OpArgs) {
        self.pc = (self.read(addr)) as usize;
    }

    // 5
    pub fn jeq(&mut self, addr: &OpArgs, b: &OpArgs, c: &OpArgs) {
        if self.read(b) == self.read(c) {
            self.jmp(addr)
        };
    }

    // 6
    pub fn jne(&mut self, addr: &OpArgs, b: &OpArgs, c: &OpArgs) {
        if self.read(b) != self.read(c) {
            self.jmp(addr)
        };
    }

    // 7
    pub fn jgt(&mut self, addr: &OpArgs, b: &OpArgs, c: &OpArgs) {
        if self.read(b) > self.read(c) {
            self.jmp(addr)
        };
    }

    // 8
    pub fn jge(&mut self, addr: &OpArgs, b: &OpArgs, c: &OpArgs) {
        if self.read(b) >= self.read(c) {
            self.jmp(addr)
        };
    }

    // 9
    pub fn jlt(&mut self, addr: &OpArgs, b: &OpArgs, c: &OpArgs) {
        if self.read(b) < self.read(c) {
            self.jmp(addr)
        };
    }

    // 10
    pub fn jle(&mut self, addr: &OpArgs, b: &OpArgs, c: &OpArgs) {
        if self.read(b) <= self.read(c) {
            self.jmp(addr)
        };
    }

    // 11
    pub fn jsr(&mut self, addr: &OpArgs) {
        self.call_stack.push(self.pc);
        self.jmp(addr);
    }

    // 12
    pub fn ret(&mut self) {
        self.pc = match self.call_stack.pop() {
            Some(addr) => addr,
            None => panic!("tried to return with empty call stack!"),
        }
    }

    // 13
    pub fn add(&mut self, a: &OpArgs, b: &OpArgs) {
        let read_a = self.read(a);
        let read_b = self.read(b);
        self.write(a, read_a.wrapping_add(read_b));
    }

    // 14
    pub fn sub(&mut self, a: &OpArgs, b: &OpArgs) {
        let read_a = self.read(a);
        let read_b = self.read(b);
        self.write(a, read_a.wrapping_sub(read_b));
    }

    // 15
    pub fn mul(&mut self, a: &OpArgs, b: &OpArgs) {
        let read_a = self.read(a);
        let read_b = self.read(b);
        self.write(a, read_a.wrapping_mul(read_b));
    }

    // 16
    pub fn mdl(&mut self, a: &OpArgs, b: &OpArgs) {
        let read_a = self.read(a);
        let read_b = self.read(b);
        self.write(a, read_a.wrapping_rem(read_b));
    }

    // 17
    pub fn and(&mut self, a: &OpArgs, b: &OpArgs) {
        let read_a = self.read(a);
        let read_b = self.read(b);
        self.write(a, read_a & read_b);
    }

    // 18
    pub fn orr(&mut self, a: &OpArgs, b: &OpArgs) {
        let read_a = self.read(a);
        let read_b = self.read(b);
        self.write(a, read_a | read_b);
    }

    // 19
    pub fn not(&mut self, a: &OpArgs) {
        let read_a = self.read(a);
        self.write(a, !read_a);
    }

    // 20
    pub fn xor(&mut self, a: &OpArgs, b: &OpArgs) {
        let read_a = self.read(a);
        let read_b = self.read(b);
        self.write(a, read_a ^ read_b);
    }

    // 21
    pub fn lsl(&mut self, a: &OpArgs, b: &OpArgs) {
        let read_a = self.read(a);
        let read_b = self.read(b);

        let result = read_a.checked_shl(read_b as u32);
        self.write(
            a,
            match result {
                Some(n) => n,
                None => 0,
            },
        );
    }

    // 22
    pub fn lsr(&mut self, a: &OpArgs, b: &OpArgs) {
        let read_a = self.read(a);
        let read_b = self.read(b);

        let result = read_a.checked_shr(read_b as u32);
        self.write(
            a,
            match result {
                Some(n) => n,
                None => 0,
            },
        );
    }

    // 23
    pub fn psh(&mut self, a: &OpArgs) {
        let read_a = self.read(a);
        self.stack.push(read_a);
    }

    // 23
    pub fn pop(&mut self, a: &OpArgs) {
        let popped = self.stack.pop();
        self.write(
            a,
            match popped {
                Some(n) => n,
                None => panic!("attempted to pop empty stack"),
            },
        );
    }
}
