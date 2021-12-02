use super::registers::Reg;
use super::*;

fn mmathi(funct: u8, rd: Reg, rs1: Reg, imm: u16) -> Inst {
    i(0b0110011, rd, funct, rs1, imm)
}

macro_rules! mmathi_impl {
    ($name:ident, $funct:expr) => {
        pub fn $name(rd: Reg, rs1: Reg, imm: u16) -> Inst {
            mmathi($funct, rd, rs1, imm)
        }
    };
}

mmathi_impl!(mul, 0b000);
mmathi_impl!(mulh, 0b001);
mmathi_impl!(mulhsu, 0b010);
mmathi_impl!(mulhu, 0b011);
mmathi_impl!(div, 0b100);
mmathi_impl!(divu, 0b101);
mmathi_impl!(rem, 0b110);
mmathi_impl!(remu, 0b111);