#[repr(u8)]
enum Flag {
    C = 0,
    Z = 1,
    ID = 2,
    DM = 3,
    BRK = 4,
    O = 6,
    N = 7,
}

struct Status {
    flags: u8,
}

impl Status {
    fn new() -> Status {
        Status { flags: 0x24 }
    }

    fn set_bit(&self, flag: Flag, value: bool) {
        let position = flag;
        todo!()
    }

    fn get_bit(&self, flag: Flag) -> bool {
        todo!()
    }
}

pub struct Registers {
    acc: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    status: Status,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            acc: 0,
            x: 0,
            y: 0,
            pc: 0xc000,
            sp: 0xfd,
            status: Status::new(),
        }
    }
}

pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
        }
    }
    pub fn step(&self) {
        self.registers.status.set_bit(Flag::N, false);
        todo!("Not implemented")
    }
}

//  implement opcodes
// implement instructions
