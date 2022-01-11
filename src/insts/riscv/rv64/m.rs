use crate::insts::riscv::registers::Reg;
use crate::insts::riscv::*;

fn mmath_64(funct: u8, rd: Reg, rs1: Reg, imm: u16) -> Inst {
    i(0b0111011, rd, funct, rs1, imm)
}

macro_rules! mmath_64_impl {
    ($name:ident, $funct:expr) => {
        pub fn $name(rd: Reg, rs1: Reg, imm: u16) -> Inst {
            mmath_64($funct, rd, rs1, imm)
        }
    };
}

mmath_64_impl!(mulw , 0b000);
mmath_64_impl!(divw , 0b100);
mmath_64_impl!(divuw, 0b101);
mmath_64_impl!(remw , 0b110);
mmath_64_impl!(remuw, 0b111);