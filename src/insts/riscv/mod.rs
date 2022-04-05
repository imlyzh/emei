pub mod inst_dump_buf;
pub mod isa_transform;
pub mod registers;
pub mod untils;
pub mod rv32;
pub mod rv64;
// pub mod rv128;

use registers::Reg;
use lyuu_commons::isa::riscv::inst_binary::*;

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
    let mut r = RType::new();
    r.set_opcode_checked(opcode).unwrap();
    r.set_rd_checked(rd.0).unwrap();
    r.set_funct3_checked(funct3).unwrap();
    r.set_rs1_checked(rs1.0).unwrap();
    r.set_rs2_checked(rs2.0).unwrap();
    r.set_funct7_checked(funct7).unwrap();
    u32::from_le_bytes(r.into_bytes())
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
    let mut r = IType::new();
    r.set_opcode_checked(opcode).unwrap();
    r.set_rd_checked(rd.0).unwrap();
    r.set_funct3_checked(funct3).unwrap();
    r.set_rs1_checked(rs1.0).unwrap();
    assert!((imm >> 11) != 0b11111 || (imm >> 11) != 0b0, "imm out of range");
    let mut rimm = (imm & 0b11111111111) as u16;
    rimm |= ((imm >> 15) << 15) as u16;
    r.set_imm_checked(rimm).unwrap();
    u32::from_le_bytes(r.into_bytes())
}

pub fn s(
    opcode: u8,
    imm0_4: u8,
    funct3: u8,
    rs1: Reg,
    rs2: Reg,
    imm5_11: u8,
) -> Inst {
    let mut r = SType::new();
    r.set_opcode_checked(opcode).unwrap();
    r.set_imm4_0_checked(imm0_4).unwrap();
    r.set_funct3_checked(funct3).unwrap();
    r.set_rs1_checked(rs1.0).unwrap();
    r.set_rs2_checked(rs2.0).unwrap();
    r.set_imm11_5_checked(imm5_11).unwrap();
    u32::from_le_bytes(r.into_bytes())
}

pub fn u(
    opcode: u8,
    rd: Reg,
    imm: u32,
) -> Inst {
    let mut r = UType::new();
    r.set_opcode_checked(opcode).unwrap();
    r.set_rd_checked(rd.0).unwrap();
    r.set_imm_checked(imm).unwrap();
    u32::from_le_bytes(r.into_bytes())
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
    let mut r = BType::new();
    r.set_opcode_checked(opcode).unwrap();
    r.set_imm11_checked(imm11).unwrap();
    r.set_imm4_1_checked(imm1_4).unwrap();
    r.set_funct3_checked(funct3).unwrap();
    r.set_rs1_checked(rs1.0).unwrap();
    r.set_rs2_checked(rs2.0).unwrap();
    r.set_imm10_5_checked(imm5_10).unwrap();
    r.set_imm12_checked(imm12).unwrap();
    u32::from_le_bytes(r.into_bytes())
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
    let mut r = JType::new();
    r.set_opcode_checked(opcode).unwrap();
    r.set_rd_checked(rd.0).unwrap();
    r.set_imm19_12_checked(imm12_19).unwrap();
    r.set_imm11_checked(imm11).unwrap();
    r.set_imm10_1_checked(imm1_10).unwrap();
    r.set_imm20_checked(imm20).unwrap();
    u32::from_le_bytes(r.into_bytes())
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