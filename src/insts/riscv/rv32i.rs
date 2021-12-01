use super::registers::Reg;
use super::*;

pub fn lui(rd: Reg, imm: u32) -> Inst {
    u(0b0110111, rd, imm)
}

pub fn auipc(rd: Reg, imm: u32) -> Inst {
    u(0b0010111, rd, imm)
}

pub fn jal(rd: Reg, imm: i32) -> Inst {
    let imm12_19 = ((imm >> 12) & 0b11111111) as u8;
    let imm11 = ((imm >> 11) & 0b1) as u8;
    let imm1_10 = ((imm >> 1) & 0b1111111111) as u8;
    let imm20 = ((imm >> 31) & 0b1) as u8;
    j(0b1101111, rd, imm12_19, imm11, imm1_10, imm20)
}

pub fn jalr(rd: Reg, rs1: Reg, imm: u16) -> Inst {
    i(0b1100111, rd, 000, rs1, imm)
}

pub fn branch(funct: u8, rs1: Reg, rs2: Reg, imm: i16) -> Inst {
    let imm11 = ((imm >> 11) & 0b1) as u8;
    let imm1_4 = ((imm >> 1) & 0b1111) as u8;
    let imm5_10 = ((imm >> 5) & 0b111111) as u8;
    let imm12 = ((imm >> 15) & 0b1) as u8;
    b(0b1100011, imm11, imm1_4, funct, rs1, rs2, imm5_10, imm12)
}

macro_rules! br_impl {
    ($name:ident, $funct:expr) => {
        pub fn $name(rs1: Reg, rs2: Reg, imm: i16) -> Inst {
            branch($funct, rs1, rs2, imm)
        }
    };
}

br_impl!(beq, 0b000);
br_impl!(bne, 0b001);
br_impl!(blt, 0b100);
br_impl!(bge, 0b101);
br_impl!(bltu, 0b110);
br_impl!(bgeu, 0b111);


pub fn load_data(funct: u8, rd: Reg, rs1: Reg, imm: u16) -> Inst {
    i(0b0000011, rd, funct, rs1, imm)
}

macro_rules! ld_impl {
    ($name:ident, $funct:expr) => {
        pub fn $name(rd: Reg, rs1: Reg, imm: u16) -> Inst {
            load_data($funct, rd, rs1, imm)
        }
    };
}

ld_impl!(lb, 0b000);
ld_impl!(lh, 0b001);
ld_impl!(lw, 0b010);
ld_impl!(lbu, 0b100);
ld_impl!(lhu, 0b101);

