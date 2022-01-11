use crate::insts::riscv::registers::Reg;
use crate::insts::riscv::*;


pub fn lr_d(rd: Reg, rs1: Reg, rl: u8, aq: u8) -> Inst {
    let mut funct7 = rl;
    funct7 |= aq << 1;
    funct7 |= 0b00010 << 2;
    r(0b0101111, rd, 0b011, rs1, Reg::new(0b00000), funct7)
}


fn atomic_64(rd: Reg, rs1: Reg, rs2: Reg, aq: u8, rl: u8, immcode: u8) -> Inst {
    let mut funct7 = rl;
    funct7 |= aq << 1;
    funct7 |= immcode << 2;
    r(0b0101111, rd, 0b011, rs1, rs2, funct7)
}

macro_rules! atomic_64_impl {
    ($name:ident, $code:expr) => {
        pub fn $name(rd: Reg, rs1: Reg, rs2: Reg, aq: u8, rl: u8) -> Inst {
            atomic_64(rd, rs1, rs2, aq, rl, $code)
        }
    };
}

atomic_64_impl!(sc_d        , 0b00011);
atomic_64_impl!(amoswap_d   , 0b00001);
atomic_64_impl!(amoadd_d    , 0b00000);
atomic_64_impl!(amoxor_d    , 0b00100);
atomic_64_impl!(amoand_d    , 0b01100);
atomic_64_impl!(amoor_d     , 0b01000);
atomic_64_impl!(amomin_d    , 0b10000);
atomic_64_impl!(amomax_d    , 0b10100);
atomic_64_impl!(amominu_d   , 0b11000);
atomic_64_impl!(amomaxu_d   , 0b11100);
