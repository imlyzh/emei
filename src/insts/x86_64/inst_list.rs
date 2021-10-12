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
pub fn movs(
    atomic: bool,
    long_mode: bool) -> Vec<u8> {
    Inst::new(atomic, long_mode,&[0xa5]).into_raw().encode()
}

/// ## push
pub fn push_all(atomic: bool, long_mode: bool) -> Vec<u8> {
    Inst::new(atomic, long_mode,&[0x60]).into_raw().encode()
}
