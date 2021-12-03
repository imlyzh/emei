use super::registers::Reg;
use super::*;


pub fn flw(rd: Reg, rs1: Reg, imm: u16) -> Inst {
    i(0b0000111, rd, 0b010, rs1, imm)
}

pub fn fsw(imm0_4: u8, rs1: Reg, rs2: Reg, imm5_11: u8) -> Inst {
    s(0b0100111, imm0_4, 0b010, rs1, rs2, imm5_11)
}

pub fn fmadd_s(rd: Reg, rm: u8, rs1: Reg, rs2: Reg, rs3: Reg) -> Inst {
    r4(0b1000011, rd, rm, rs1, rs2, rs3, 0b00)
}

pub fn fmsub_s(rd: Reg, rm: u8, rs1: Reg, rs2: Reg, rs3: Reg) -> Inst {
    r4(0b1000111, rd, rm, rs1, rs2, rs3, 0b00)
}

pub fn fnmsub_s(rd: Reg, rm: u8, rs1: Reg, rs2: Reg, rs3: Reg) -> Inst {
    r4(0b1001011, rd, rm, rs1, rs2, rs3, 0b00)
}

pub fn fnmadd_s(rd: Reg, rm: u8, rs1: Reg, rs2: Reg, rs3: Reg) -> Inst {
    r4(0b1001111, rd, rm, rs1, rs2, rs3, 0b00)
}

fn fmathi(rm: u8, rd: Reg, rs1: Reg, rs2: Reg, imm: u8) -> Inst {
    r(0b1010011, rd, rm, rs1, rs2, imm)
}

macro_rules! fmathi_imm_impl {
    ($name:ident, $imm:expr) => {
        pub fn $name(rm: u8, rd: Reg, rs1: Reg, rs2: Reg) -> Inst {
            fmathi(rm, rd, rs1, rs2, $imm)
        }
    };
}

macro_rules! fmathi_rm_imm_impl {
    ($name:ident, $rm:expr, $imm:expr) => {
        pub fn $name(rd: Reg, rs1: Reg, rs2: Reg) -> Inst {
            fmathi($rm, rd, rs1, rs2, $imm)
        }
    };
}

fmathi_imm_impl!(fadd_s, 0b0000000);
fmathi_imm_impl!(fsub_s, 0b0000100);
fmathi_imm_impl!(fmul_s, 0b0001000);
fmathi_imm_impl!(fdiv_s, 0b0001100);

pub fn fsqrt_s(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00000), 0b0001100)
}

fmathi_rm_imm_impl!(fsgnj_s , 0b000, 0b0010000);
fmathi_rm_imm_impl!(fsgnjn_s, 0b001, 0b0010000);
fmathi_rm_imm_impl!(fsgnjx_s, 0b010, 0b0010000);
fmathi_rm_imm_impl!(fmin_s  , 0b000, 0b0010100);
fmathi_rm_imm_impl!(fmax_s  , 0b001, 0b0010100);

pub fn fcvt_w_s(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00000), 0b1100000)
}

pub fn fcvt_wu_s(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00001), 0b1100000)
}

pub fn fmv_x_s(rd: Reg, rs1: Reg) -> Inst {
    fmathi(0b000, rd, rs1, Reg::new(0b00000), 0b1110000)
}

pub fn feq_s(rd: Reg, rs1: Reg, rs2: Reg) -> Inst {
    fmathi(0b010, rd, rs1, rs2, 0b1010000)
}

pub fn flt_s(rd: Reg, rs1: Reg, rs2: Reg) -> Inst {
    fmathi(0b001, rd, rs1, rs2, 0b1010000)
}

pub fn fle_s(rd: Reg, rs1: Reg, rs2: Reg) -> Inst {
    fmathi(0b000, rd, rs1, rs2, 0b1010000)
}

pub fn fclass_s(rd: Reg, rs1: Reg) -> Inst {
    fmathi(0b001, rd, rs1, Reg::new(0b00000), 0b1110000)
}

pub fn fcvt_s_w(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00000), 0b1101000)
}

pub fn fcvt_s_wu(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00001), 0b1101000)
}

pub fn fmv_w_x(rd: Reg, rs1: Reg) -> Inst {
    fmathi(0b000, rd, rs1, Reg::new(0b00000), 0b1111000)
}


///////////////////////////////////////////////////////////////////
/// rv32d

pub fn fld(rd: Reg, rs1: Reg, imm: u16) -> Inst {
    i(0b0000111, rd, 0b011, rs1, imm)
}

pub fn fsd(imm0_4: u8, rs1: Reg, rs2: Reg, imm5_11: u8) -> Inst {
    s(0b0100111, imm0_4, 0b011, rs1, rs2, imm5_11)
}

pub fn fmadd_d(rd: Reg, rm: u8, rs1: Reg, rs2: Reg, rs3: Reg) -> Inst {
    r4(0b1000011, rd, rm, rs1, rs2, rs3, 0b01)
}

pub fn fmsub_d(rd: Reg, rm: u8, rs1: Reg, rs2: Reg, rs3: Reg) -> Inst {
    r4(0b1000111, rd, rm, rs1, rs2, rs3, 0b01)
}

pub fn fnmsub_d(rd: Reg, rm: u8, rs1: Reg, rs2: Reg, rs3: Reg) -> Inst {
    r4(0b1001011, rd, rm, rs1, rs2, rs3, 0b01)
}

pub fn fnmadd_d(rd: Reg, rm: u8, rs1: Reg, rs2: Reg, rs3: Reg) -> Inst {
    r4(0b1001111, rd, rm, rs1, rs2, rs3, 0b01)
}


fmathi_imm_impl!(fadd_d, 0b0000001);
fmathi_imm_impl!(fsub_d, 0b0000101);
fmathi_imm_impl!(fmul_d, 0b0001001);
fmathi_imm_impl!(fdiv_d, 0b0001101);

pub fn fsqrt_d(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00000), 0b0001101)
}

fmathi_rm_imm_impl!(fsgnj_d , 0b000, 0b0010001);
fmathi_rm_imm_impl!(fsgnjn_d, 0b001, 0b0010001);
fmathi_rm_imm_impl!(fsgnjx_d, 0b010, 0b0010001);
fmathi_rm_imm_impl!(fmin_d  , 0b000, 0b0010101);
fmathi_rm_imm_impl!(fmax_d  , 0b001, 0b0010101);

pub fn fcvt_s_d(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00001), 0b0100000)
}

pub fn fcvt_d_s(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00000), 0b0100001)
}

pub fn feq_d(rd: Reg, rs1: Reg, rs2: Reg) -> Inst {
    fmathi(0b010, rd, rs1, rs2, 0b1010001)
}

pub fn flt_d(rd: Reg, rs1: Reg, rs2: Reg) -> Inst {
    fmathi(0b001, rd, rs1, rs2, 0b1010001)
}

pub fn fle_d(rd: Reg, rs1: Reg, rs2: Reg) -> Inst {
    fmathi(0b000, rd, rs1, rs2, 0b1010001)
}

pub fn fclass_d(rd: Reg, rs1: Reg) -> Inst {
    fmathi(0b001, rd, rs1, Reg::new(0b00000), 0b1110001)
}

pub fn fcvt_w_d(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00000), 0b1100001)
}

pub fn fcvt_wu_d(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00001), 0b1100001)
}

pub fn fmv_d_w(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00000), 0b1101001)
}

pub fn fmv_d_wu(rm: u8, rd: Reg, rs1: Reg) -> Inst {
    fmathi(rm, rd, rs1, Reg::new(0b00001), 0b1101001)
}