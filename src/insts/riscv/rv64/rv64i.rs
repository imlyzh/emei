use crate::insts::riscv::registers::Reg;
use crate::insts::riscv::*;
use crate::insts::riscv::untils::*;

use crate::{ld_impl, st_impl};


ld_impl!(ld, 0b011);
ld_impl!(lwu, 0b110);

st_impl!(sd, 0b011);

pub fn mathi_64(funct: u8, rd: Reg, rs1: Reg, imm: i16) -> Inst {
    i(0b0011011, rd, funct, rs1, imm)
}

macro_rules! mathi_64_impl {
    ($name:ident, $funct:expr) => {
        pub fn $name(rd: Reg, rs1: Reg, imm: i16) -> Inst {
            mathi_64($funct, rd, rs1, imm)
        }
    };
}

mathi_64_impl!(addiw, 0b000);
mathi_64_impl!(slliw, 0b001);
mathi_64_impl!(srliw, 0b101);
mathi_64_impl!(sraiw, 0b101);


pub fn math_64(funct3: u8, funct7: u8, rd: Reg, rs1: Reg, rs2: Reg) -> Inst {
    r(0b0111011, rd, funct3, rs1, rs2, funct7)
}

macro_rules! math_64_impl {
    ($name:ident, $funct3:expr, $funct7:expr) => {
        pub fn $name(rd: Reg, rs1: Reg, rs2: Reg) -> Inst {
            math_64($funct3, $funct7, rd, rs1, rs2)
        }
    };
}

math_64_impl!(addw, 0b000, 0b0000000);
math_64_impl!(subw, 0b000, 0b0100000);
math_64_impl!(sllw, 0b001, 0b0000000);
math_64_impl!(srlw, 0b101, 0b0000000);
math_64_impl!(sraw, 0b101, 0b0100000);
