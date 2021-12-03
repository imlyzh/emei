use super::registers::Reg;
use super::*;


pub fn lr_w(rd: Reg, rs1: Reg, rl: u8, aq: u8) -> Inst {
    let mut funct7 = rl;
    funct7 |= aq << 1;
    funct7 |= 0b00010 << 2;
    r(0b0101111, rd, 0b010, rs1, 0b00000, funct7)
}

fn atomic(rd: Reg, rs1: Reg, rs2: Reg, aq: u8, rl: u8, immcode: u8) -> Inst {
    let mut funct7 = rl;
    funct7 |= aq << 1;
    funct7 |= immcode << 2;
    r(0b0101111, rd, 0b010, rs1, rs2, funct7)
}

macro_rules! atomic_impl {
    ($name:ident, $code:expr) => {
        pub fn $name(rd: Reg, rs1: Reg, rs2: Reg, aq: u8, rl: u8) -> Inst {
            atomic(rd, rs1, rs2, aq, rl, $code)
        }
    };
}

atomic_impl!(sc_w       , 0b00011);
atomic_impl!(amoswap_w  , 0b00001);
atomic_impl!(amoadd_w   , 0b00000);
atomic_impl!(amoxor_w   , 0b00100);
atomic_impl!(amoand_w   , 0b01100);
atomic_impl!(amoor_w    , 0b01000);
atomic_impl!(amomin_w   , 0b10000);
atomic_impl!(amomax_w   , 0b10100);
atomic_impl!(amominu_w  , 0b11000);
atomic_impl!(amomaxu_w  , 0b11100);