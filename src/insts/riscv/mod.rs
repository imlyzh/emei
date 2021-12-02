pub mod inst_dump_buf;
pub mod registers;
pub mod rv32i;
pub mod rv32m;
pub mod rv32fd;
pub mod rv32a;
pub mod rv32c;
pub mod rv32v;

use registers::Reg;


pub type CInst = u16;
pub type Inst = u32;

pub fn r(
    opcode: u8,
    rd: Reg,
    funct3: u8,
    rs1: Reg,
    rs2: Reg,
    funct7: u8
) -> Inst {
    let mut r = (opcode & 0b111111) as u32;
    r |= ((rd & 0b11111) as u32) << 6;
    r |= ((funct3 & 0b111) as u32) << 11;
    r |= ((rs1 & 0b11111) as u32) << 14;
    r |= ((rs2 & 0b11111) as u32) << 19;
    r |= ((funct7 & 0b1111111) as u32) << 24;
    r
}

pub fn r4(
    opcode: u8,
    rd: Reg,
    rm: u8,
    rs1: Reg,
    rs2: Reg,
    rs3: Reg,
    mut funct7: u8
) -> Inst {
    funct7 |= rs3 << 2;
    r(opcode, rd, rm, rs1, rs2, funct7)
}

pub fn i(
    opcode: u8,
    rd: Reg,
    funct3: u8,
    rs1: Reg,
    imm: u16,
) -> Inst {
    let mut r = opcode as u32;
    r |= ((rd & 0b11111) as u32) << 6;
    r |= ((funct3 & 0b111) as u32) << 11;
    r |= ((rs1 & 0b11111) as u32) << 14;
    r |= ((imm & 0b11111111111) as u32) << 19;
    r
}

pub fn s(
    opcode: u8,
    imm0_4: u8,
    funct3: u8,
    rs1: Reg,
    rs2: Reg,
    imm5_11: u8,
) -> Inst {
    let mut r = opcode as u32;
    r |= ((imm0_4 & 0b11111) as u32) << 6;
    r |= ((funct3 & 0b111) as u32) << 11;
    r |= ((rs1 & 0b11111) as u32) << 14;
    r |= ((rs2 & 0b11111) as u32) << 19;
    r |= ((imm5_11 & 0b1111111) as u32) << 24;
    r
}

pub fn u(
    opcode: u8,
    rd: Reg,
    imm: u32,
) -> Inst {
    let mut r = opcode as u32;
    r |= ((rd & 0b11111) as u32) << 6;
    r |= (imm << 20) >> 9;
    r
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
    let imm0_4 = imm11 | (imm1_4 << 1);
    let imm5_11 = imm5_10 | (imm12 << 6);
    s(opcode, imm0_4, funct3, rs1, rs2, imm5_11)
}

pub fn j(
    opcode: u8,
    rd: Reg,
    imm12_19: u8,
    imm11: u8,
    imm1_10: u8,
    imm20: u8,
) -> Inst {
    // Warning: data is not filter, Pass parameters in compliance with standards
    let mut imm = imm12_19 as u32;
    imm |= (imm11 as u32) << 8;
    imm |= (imm1_10 as u32) << 9;
    imm |= (imm20 as u32) << 19;
    u(opcode, rd, imm)
}
