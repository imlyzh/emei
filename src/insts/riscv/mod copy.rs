pub mod inst_dump_buf;
pub mod isa_transform;
pub mod registers;
pub mod untils;
pub mod rv32;
pub mod rv64;
// pub mod rv128;

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
    let mut r = (opcode & 0b1111111) as u32;
    r |= ((rd.0 & 0b11111) as u32) << 7;
    r |= ((funct3 & 0b111) as u32) << 12;
    r |= ((rs1.0 & 0b11111) as u32) << 15;
    r |= ((rs2.0 & 0b11111) as u32) << 20;
    r |= ((funct7 & 0b1111111) as u32) << 25;
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
    funct7 |= rs3.0 << 2;
    r(opcode, rd, rm, rs1, rs2, funct7)
}

pub fn i(
    opcode: u8,
    rd: Reg,
    funct3: u8,
    rs1: Reg,
    imm: i16,
) -> Inst {
    let mut r = (opcode & 0b1111111) as u32;
    r |= ((rd.0 & 0b11111) as u32) << 7;
    r |= ((funct3 & 0b111) as u32) << 12;
    r |= ((rs1.0 & 0b11111) as u32) << 15;
    r |= ((imm & 0b11111111111) as u32) << 20;
    r |= ((imm >> 15) as u32) << 31;
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
    let mut r = (opcode & 0b1111111) as u32;
    r |= ((imm0_4 & 0b11111) as u32) << 7;
    r |= ((funct3 & 0b111) as u32) << 12;
    r |= ((rs1.0 & 0b11111) as u32) << 15;
    r |= ((rs2.0 & 0b11111) as u32) << 20;
    r |= ((imm5_11 & 0b1111111) as u32) << 25;
    r
}

pub fn u(
    opcode: u8,
    rd: Reg,
    imm: u32,
) -> Inst {
    let mut r = (opcode & 0b1111111) as u32;
    r |= ((rd.0 & 0b11111) as u32) << 7;
    r |= imm << 12;
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
    imm1_10: u16,
    imm20: u8,
) -> Inst {
    // Warning: data is not filter, Pass parameters in compliance with standards
    let mut imm = imm12_19 as u32;
    imm |= (imm11 as u32) << 8;
    imm |= (imm1_10 as u32) << 9;
    imm |= (imm20 as u32) << 19;
    u(opcode, rd, imm)
}

/*
pub fn cr(opcode: u8, rs2: Reg, rd: Reg, funct4: u8) -> CInst {
    let mut r = opcode as u16;
    r |= ((rs2.0 & 0b11111) as u16) << 2;
    r |= ((rd.0 & 0b11111) as u16) << 7;
    r |= ((funct4 & 0b1111) as u16) << 12;
    r
}

pub fn ci(opcode: u8, nzimm0_4: u8, rd: Reg, nzimm_5: u8, code: u8) -> CInst {
    let mut r = opcode as u16;
    r |= ((nzimm0_4 & 0b11111) as u16) << 2;
    r |= ((rd.0 & 0b11111) as u16) << 7;
    r |= ((nzimm_5 & 0b1) as u16) << 12;
    r |= ((code & 0b1111) as u16) << 13;
    r
}

pub fn css(opcode: u8, rs2: Reg, imm0_4: u8, funct3: u8) -> CInst {
    let mut r = opcode as u16;
    r |= ((rs2.0 & 0b11111) as u16) << 2;
    r |= ((imm0_4 & 0b11111) as u16) << 7;
    r |= ((funct3 & 0b111) as u16) << 13;
    r
}

pub fn ciw(opcode: u8, rdc: CReg, imm: u8, funct3: u8) -> CInst {
    let mut r = opcode as u16;
    r |= ((rdc.0 & 0b111) as u16) << 2;
    r |= ((imm & 0b11111111) as u16) << 5;
    r |= ((funct3 & 0b111) as u16) << 13;
    r
}

// pub fn cls(opcode: u8, rdc: Reg, imm: )

pub fn cb(opcode: u8, offset0: u8, rs1c: CReg, offset1: u8, funct3: u8) -> CInst {
    let mut r = opcode as u16;
    r |= ((offset0 & 0b11111) as u16) << 2;
    r |= ((rs1c.0 & 0b111) as u16) << 7;
    r |= ((offset1 & 0b11111) as u16) << 10;
    r |= ((funct3 & 0b111) as u16) << 13;
    r
}

pub fn cj(opcode: u8, offset: u16, funct3: u8) -> CInst {
    let mut r = opcode as u16;
    r |= ((offset & 0b11111111111) as u16) << 2;
    r |= ((funct3 & 0b111) as u16) << 13;
    r
}
*/