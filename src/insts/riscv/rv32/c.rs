use crate::insts::riscv::registers::{Reg, X0};
use crate::insts::riscv::*;

pub fn c_addi(rd: Reg, nzimm: i8) -> CInst {
    let imm0_4 = nzimm as u8 & 0b11111;
    let imm5 = (nzimm as u8) >> 7;
    ci(0b01, imm0_4, rd, imm5, 0b000)
}

pub fn c_nop() -> CInst {
    c_addi(Reg::new(X0), 0)
}

/*
pub fn c_jal(offset: i16) -> CInst {
    // let imm5 = offset as u16 >> 5;
    // let imm11 = (offset as u16) >> 11;
    cj(0b01, todo!(), 0b001)
}
*/