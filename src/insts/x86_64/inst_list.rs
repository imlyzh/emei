use crate::insts::x86_64::{ImmByte};

use super::{Inst, Operator1, Operator2, registers::*};



/// ## mov
/// - mov
pub fn mov(
    is_atomic: bool,
    is_long_mode: bool,
    op1: Operator1,
    op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x89],
        op1: Some(op1),
        op2: Some(Operator2::Register(op2)),
    }.into_raw().encode()
}

/// - mov_rev
/// mov_rev is the same as mov, but the source and destination operands are reversed.
pub fn mov_rev(
    is_atomic: bool,
    is_long_mode: bool,
    op1: Operator1,
    op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x8b],
        op1: Some(op1),
        op2: Some(Operator2::Register(op2)),
    }.into_raw().encode()
}

pub fn mov_imm_into_reg(
    is_atomic: bool,
    is_long_mode: bool,
    op1: TargetReg,
    op2: u64) -> Vec<u8> {
    let imm_byte = if is_long_mode {
        ImmByte::Bit64
    } else {
        ImmByte::Bit32
    };
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x8b],
        op1: Some(Operator1::Direct(op1)),
        op2: Some(Operator2::Imm(op2, imm_byte)),
    }.into_raw().encode()
}

pub fn imm_sign_extend_into_reg(
    is_atomic: bool,
    is_long_mode: bool,
    op1: TargetReg,
    op2: u64) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x7c],
        op1: Some(Operator1::Direct(op1)),
        op2: Some(Operator2::Imm(op2, ImmByte::Bit32)),
    }.into_raw().encode()
}

///  movs == movsq
pub fn movs(atomic: bool) -> Vec<u8> {
    Inst::new(atomic, false,&[0xa5]).into_raw().encode()
}

/// ## push
///
/// - push reg
pub fn push_reg(atomic: bool, reg: TargetReg) -> Vec<u8> {
    Inst {
        atomic,
        long_mode: false,
        opcode: vec![0x60],
        op1: None,
        op2: Some(Operator2::Imm(reg as u64, ImmByte::Bit8)),
    }.into_raw().encode()
}

/// - push_imm
pub fn push_imm(atomic: bool, imm: u32) -> Vec<u8> {
    Inst {
        atomic,
        long_mode: false,
        opcode: vec![0x60],
        op1: None,
        op2: Some(Operator2::Imm(imm as u64, ImmByte::Bit32)),
    }.into_raw().encode()
}

/// - push_all
pub fn push_all(atomic: bool) -> Vec<u8> {
    Inst::new(atomic, false,&[0x60]).into_raw().encode()
}

/// ## add
/// - add_to_eax(rax)

pub fn add_imm32_to_first_reg(atomic: bool, long_mode: bool, imm: u32) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x05],
        op1: None,
        op2: Some(Operator2::Imm(imm as u64, ImmByte::Bit32)),
    }.into_raw().encode()
}

pub fn add_imm32(atomic: bool, long_mode: bool, op1: Operator1, imm: u32) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x81],
        op1: Some(op1),
        op2: Some(Operator2::Imm(imm as u64, ImmByte::Bit32)),
    }.into_raw().encode()
}

pub fn add_imm8(atomic: bool, long_mode: bool, op1: Operator1, imm: u8) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x83],
        op1: Some(op1),
        op2: Some(Operator2::Imm(imm as u64, ImmByte::Bit8)),
    }.into_raw().encode()
}

pub fn add(atomic: bool, long_mode: bool, op1: Operator1, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x01],
        op1: Some(op1),
        op2: Some(Operator2::Register(op2)),
    }.into_raw().encode()
}

/// - add_rev: add_rev is the same as add, but the source and destination operands are reversed.
pub fn add_rev(atomic: bool, long_mode: bool, op1: Operator1, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x03],
        op1: Some(op1),
        op2: Some(Operator2::Register(op2)),
    }.into_raw().encode()
}

/// ## ret

pub fn near_ret() -> Vec<u8> {
    Inst::new(false, false,&[0xc3]).into_raw().encode()
}

pub fn far_ret() -> Vec<u8> {
    Inst::new(false, false,&[0xcb]).into_raw().encode()
}

pub fn near_ret_imm16(imm: u16) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0xc2],
        op1: None,
        op2: Some(Operator2::Imm(imm as u64, ImmByte::Bit16)),
    }.into_raw().encode()
}

pub fn far_ret_imm16(imm: u16) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0xca],
        op1: None,
        op2: Some(Operator2::Imm(imm as u64, ImmByte::Bit16)),
    }.into_raw().encode()
}