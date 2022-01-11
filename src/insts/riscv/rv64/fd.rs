use crate::insts::riscv::registers::Reg;
use crate::insts::riscv::*;
use crate::insts::riscv::untils::*;


pub fn fcvt_l_s(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00010), 0b1100000)
}

pub fn fcvt_lu_s(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00011), 0b1100000)
}

pub fn fcvt_s_l(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00010), 0b1101000)
}

pub fn fcvt_s_lu(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00011), 0b1101000)
}


pub fn fcvt_l_d(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00010), 0b1100001)
}

pub fn fcvt_lu_d(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00011), 0b1100001)
}

pub fn fcvt_x_d(rd: Reg, rs1: Reg) -> Inst {
    fmathi(0b000, rd, rs1, Reg::new(0b0000), 0b1110001)
}

pub fn fcvt_d_l(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00010), 0b1101001)
}

pub fn fcvt_d_lu(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00011), 0b1101001)
}

pub fn fcvt_d_x(rd: Reg, rs1: Reg) -> Inst {
    fmathi(0b000, rd, rs1, Reg::new(0b0000), 0b1111001)
}
