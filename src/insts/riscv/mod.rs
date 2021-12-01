pub mod registers;
pub mod rv32i;

use registers::Reg;


// pub type CInst = u16;
pub type Inst = u32;

pub fn r(
    opcode: u8,
    rd: Reg,
    funct3: u8,
    rs1: Reg,
    rs2: Reg,
    funct7: u8
) -> Inst {
    todo!()
}

pub fn i(
    opcode: u8,
    rd: Reg,
    funct3: u8,
    rs1: Reg,
    imm: u16,
) -> Inst {
    todo!()
}

pub fn s(
    opcode: u8,
    imm0_4: u8,
    funct3: u8,
    rs1: Reg,
    rs2: Reg,
    imm5_11: u8,
) -> Inst {
    todo!()
}

pub fn u(
    opcode: u8,
    rd: Reg,
    imm: u32,
) -> Inst {
    todo!()
}

pub fn b(
    opcode: u8,
    imm11: u8,
    imm1_4: u8,
    funct3: u8,
    rs1: Reg,
    rs2: Reg,
    imm5_10: u8,
    imm12: u8,
) -> Inst {
    todo!()
}

pub fn j(
    opcode: u8,
    rd: Reg,
    imm12_19: u8,
    imm11: u8,
    imm1_10: u8,
    imm20: u8,
) -> Inst {
    todo!()
}
