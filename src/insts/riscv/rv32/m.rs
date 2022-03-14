use crate::insts::riscv::registers::Reg;
use crate::insts::riscv::*;


fn mmath(funct: u8, rd: Reg, rs1: Reg, imm: i16) -> Inst {
    i(0b0110011, rd, funct, rs1, imm)
}

macro_rules! mmath_impl {
    ($name:ident, $funct:expr) => {
        pub fn $name(rd: Reg, rs1: Reg, imm: i16) -> Inst {
            mmath($funct, rd, rs1, imm)
        }
    };
}

mmath_impl!(mul    , 0b000);
mmath_impl!(mulh   , 0b001);
mmath_impl!(mulhsu , 0b010);
mmath_impl!(mulhu  , 0b011);
mmath_impl!(div    , 0b100);
mmath_impl!(divu   , 0b101);
mmath_impl!(rem    , 0b110);
mmath_impl!(remu   , 0b111);