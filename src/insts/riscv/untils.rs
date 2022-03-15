use super::{registers::Reg, *};


pub fn branch(funct: u8, rs1: Reg, rs2: Reg, imm: i16) -> Inst {
    let imm = imm as u16;
    let imm11 = ((imm >> 11) & 0b1) as u8;
    let imm1_4 = ((imm >> 1) & 0b1111) as u8;
    let imm5_10 = ((imm >> 5) & 0b111111) as u8;
    let imm12 = ((imm >> 15) & 0b1) as u8;
    b(0b1100011, imm11, imm1_4, funct, rs1, rs2, imm5_10, imm12)
}


pub fn load_data(funct: u8, rd: Reg, rs1: Reg, imm: i16) -> Inst {
    i(0b0000011, rd, funct, rs1, imm)
}

#[macro_export]
macro_rules! ld_impl {
    ($name:ident, $funct:expr) => {
        pub fn $name(rd: Reg, rs1: Reg, imm: i16) -> Inst {
            load_data($funct, rd, rs1, imm)
        }
    };
}

pub fn store_data(funct: u8, rs1: Reg, rs2: Reg, imm: i16) -> Inst {
    let imm0_4 = (imm & 0b1111) as u8;
    let imm5_11 = ((imm >> 5) & 0b1111111) as u8;
    s(0b0100011, imm0_4, funct, rs1, rs2, imm5_11)
}

#[macro_export]
macro_rules! st_impl {
    ($name:ident, $funct:expr) => {
        pub fn $name(rs1: Reg, rs2: Reg, imm: i16) -> Inst {
            store_data($funct, rs1, rs2, imm)
        }
    };
}

pub fn fmathi(rm: u8, rd: Reg, rs1: Reg, rs2: Reg, imm: u8) -> Inst {
    r(0b1010011, rd, rm, rs1, rs2, imm)
}