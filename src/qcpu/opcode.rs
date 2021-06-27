use std::fmt;

#[derive(Clone, Copy)]
pub enum AddressingMode {
    IMMEDIATE = 0b00,
    ABSOLUTE = 0b01,
    INDIRECT = 0b10,
    REGISTER = 0b11,
}

#[derive(PartialEq)]
pub enum OpCode {

    // system operations
    NOP = 0x0,  // does nothing
    EXT = 0x1,  // stop exection, returns value a
    SYS = 0x2,  // executes system call a (usually accepting argument in register x)

    // data operations
    MOV = 0x3,  // sets the value in a to the value in b

    // jumps and conditionals
    JMP = 0x4,  // jump to address a
    JEQ = 0x5,  // jump to address a if b == c
    JNE = 0x6,  // jump to address a if b != c
    JGT = 0x7,  // jump to address a if b > c
    JGE = 0x8,  // jump to address a if b >= c
    JLT = 0x9,  // jump to address a if b < c
    JLE = 0xA,  // jump to address a if b <= c

    // subroutines
    JSR = 0xB,  // push the current address to the call stack and jump to address a
    RET = 0xC,  // pop an address from the call stack and jump to that address

    // arithmetic operations	
    ADD = 0xD,  // add b to the contents of a
    SUB = 0xE,  // subtract b from the contents of a
    MUL = 0xF,  // multiply the contents of a by b
    MDL = 0x10, // set the contents of a to a % b

    // bitwise operations	
    AND = 0x11, // set the contents of a to the bitwise and of a with b
    ORR = 0x12, // set the contents of a to the bitwise or of a with b
    NOT = 0x13, // perform a bitwise not on the contents of a
    XOR = 0x14, // set the contents of a to the bitwise xor of a with b
    LSL = 0x15, // perform a logical left shift by b bits on the contents of a
    LSR = 0x16, // perform a logical right shift by b bits on the contents of a

    // stack operations	
    PSH = 0x17, // push value of a onto stack
    POP = 0x18, // 	pop top value from stack into a
}

impl AddressingMode {
    pub fn from_int(num: u16) -> AddressingMode {
        match num {
            0b00 => AddressingMode::IMMEDIATE,
            0b01 => AddressingMode::ABSOLUTE,
            0b10 => AddressingMode::INDIRECT,
            0b11 => AddressingMode::REGISTER,
            _ => panic!("unknown addressing mode {}", num),
        }
    }

    pub fn map_from_int(
        num: u16,
    ) -> (
        AddressingMode,
        AddressingMode,
        AddressingMode,
        AddressingMode,
    ) {
        (
            AddressingMode::from_int((num & 0b11000000) >> 6),
            AddressingMode::from_int((num & 0b00110000) >> 4),
            AddressingMode::from_int((num & 0b00001100) >> 2),
            AddressingMode::from_int(num & 0b00000011),
        )
    }
}

impl fmt::Display for AddressingMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            AddressingMode::IMMEDIATE => "I",
            AddressingMode::INDIRECT => "N",
            AddressingMode::REGISTER => "R",
            AddressingMode::ABSOLUTE => "A",
        };
        write!(f, "{}", s)
    }
}

impl OpCode {
    pub fn from_int(num: u16) -> OpCode {
        match num {
            0x0 => OpCode::NOP,
            0x1 => OpCode::EXT,
            0x2 => OpCode::SYS,
            0x3 => OpCode::MOV,
            0x4 => OpCode::JMP,
            0x5 => OpCode::JEQ,
            0x6 => OpCode::JNE,
            0x7 => OpCode::JGT,
            0x8 => OpCode::JGE,
            0x9 => OpCode::JLT,
            0xA => OpCode::JLE,
            0xB => OpCode::JSR,
            0xC => OpCode::RET,
            0xD => OpCode::ADD,
            0xE => OpCode::SUB,
            0xF => OpCode::MUL,
            0x10 => OpCode::MDL,
            0x11 => OpCode::AND,
            0x12 => OpCode::ORR,
            0x13 => OpCode::NOT,
            0x14 => OpCode::XOR,
            0x15 => OpCode::LSL,
            0x16 => OpCode::LSR,
            0x17 => OpCode::PSH,
            0x18 => OpCode::POP,
            _ => panic!("unrecognised opcode {}", num),
        }
    }

    pub fn arity(&self) -> u16 {
        match *self {
            OpCode::NOP | OpCode::RET => 0,
            OpCode::EXT
            | OpCode::SYS
            | OpCode::JMP
            | OpCode::JSR
            | OpCode::NOT
            | OpCode::PSH
            | OpCode::POP => 1,
            OpCode::MOV
            | OpCode::ADD
            | OpCode::SUB
            | OpCode::MUL
            | OpCode::MDL
            | OpCode::AND
            | OpCode::ORR
            | OpCode::XOR
            | OpCode::LSL
            | OpCode::LSR => 2,
            OpCode::JEQ | OpCode::JNE | OpCode::JGT | OpCode::JGE | OpCode::JLT | OpCode::JLE => 3,
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            OpCode::NOP => "NOP",
            OpCode::EXT => "EXT",
            OpCode::SYS => "SYS",
            OpCode::MOV => "MOV",
            OpCode::JMP => "JMP",
            OpCode::JEQ => "JEQ",
            OpCode::JNE => "JNE",
            OpCode::JGT => "JGT",
            OpCode::JGE => "JGE",
            OpCode::JLT => "JLT",
            OpCode::JLE => "JLE",
            OpCode::JSR => "JSR",
            OpCode::RET => "RET",
            OpCode::ADD => "ADD",
            OpCode::SUB => "SUB",
            OpCode::MUL => "MUL",
            OpCode::MDL => "MDL",
            OpCode::AND => "AND",
            OpCode::ORR => "ORR",
            OpCode::NOT => "NOT",
            OpCode::XOR => "XOR",
            OpCode::LSL => "LSL",
            OpCode::LSR => "LSR",
            OpCode::PSH => "PSH",
            OpCode::POP => "POP",
        };
        write!(f, "{}", s)
    }
}
