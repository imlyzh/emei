use crate::insts::x86_64::ImmByte;

use super::{registers::*, Inst, Operator1, Operator2};

/// ## mov
/// - mov
pub fn mov(is_atomic: bool, is_long_mode: bool, op1: Operator1, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x89],
        op1: Some(op1),
        op2: Some(Operator2::Register(op2)),
    }
    .into_raw()
    .encode()
}

/// - mov_rev
/// mov_rev is the same as mov, but the source and destination operands are reversed.
pub fn mov_rev(is_atomic: bool, is_long_mode: bool, op1: Operator1, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x8b],
        op1: Some(op1),
        op2: Some(Operator2::Register(op2)),
    }
    .into_raw()
    .encode()
}

pub fn mov_imm_into_reg(is_atomic: bool, is_long_mode: bool, op1: TargetReg, op2: u64) -> Vec<u8> {
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
    }
    .into_raw()
    .encode()
}

pub fn imm_sign_extend_into_reg(
    is_atomic: bool,
    is_long_mode: bool,
    op1: TargetReg,
    op2: u64,
) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x7c],
        op1: Some(Operator1::Direct(op1)),
        op2: Some(Operator2::Imm(op2, ImmByte::Bit32)),
    }
    .into_raw()
    .encode()
}

///  movs == movsq
pub fn movs(atomic: bool) -> Vec<u8> {
    Inst::new(atomic, false, &[0xa5]).into_raw().encode()
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
    }
    .into_raw()
    .encode()
}

/// - push_imm
pub fn push_imm(atomic: bool, imm: u32) -> Vec<u8> {
    Inst {
        atomic,
        long_mode: false,
        opcode: vec![0x60],
        op1: None,
        op2: Some(Operator2::Imm(imm as u64, ImmByte::Bit32)),
    }
    .into_raw()
    .encode()
}

/// - push_all
pub fn push_all(atomic: bool) -> Vec<u8> {
    Inst::new(atomic, false, &[0x60]).into_raw().encode()
}

/// ## add
/// - add_to_eax(rax)

pub fn add_first_reg(atomic: bool, long_mode: bool, imm: u32) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x05],
        op1: None,
        op2: Some(Operator2::Imm(imm as u64, ImmByte::Bit32)),
    }
    .into_raw()
    .encode()
}

pub fn add_imm32(atomic: bool, long_mode: bool, op1: Operator1, imm: u32) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x81],
        op1: Some(op1),
        op2: Some(Operator2::Imm(imm as u64, ImmByte::Bit32)),
    }
    .into_raw()
    .encode()
}

pub fn add_imm8(atomic: bool, long_mode: bool, op1: Operator1, imm: u8) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x83],
        op1: Some(op1),
        op2: Some(Operator2::Imm(imm as u64, ImmByte::Bit8)),
    }
    .into_raw()
    .encode()
}

pub fn add(atomic: bool, long_mode: bool, op1: Operator1, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x01],
        op1: Some(op1),
        op2: Some(Operator2::Register(op2)),
    }
    .into_raw()
    .encode()
}

/// - add_rev: add_rev is the same as add, but the source and destination operands are reversed.
pub fn add_rev(atomic: bool, long_mode: bool, op1: Operator1, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x03],
        op1: Some(op1),
        op2: Some(Operator2::Register(op2)),
    }
    .into_raw()
    .encode()
}

pub fn inc(atomic: bool, long_mode: bool, op1: Operator1) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0xfe],
        op1: Some(op1),
        op2: None,
    }
    .into_raw()
    .encode()
}

pub fn inc_reg32(atomic: bool, op1: Register32) -> Vec<u8> {
    Inst {
        atomic,
        long_mode: false,
        opcode: vec![0x40],
        op1: None,
        op2: Some(Operator2::Register(TargetReg::from(op1 as u8))),
    }
    .into_raw()
    .encode()
}

/// ## sub

pub fn sub_first_reg(atomic: bool, imm: u32) -> Vec<u8> {
    Inst {
        atomic,
        long_mode: false,
        opcode: vec![0x2d],
        op1: None,
        op2: Some(Operator2::Imm(imm as u64, ImmByte::Bit32)),
    }
    .into_raw()
    .encode()
}

pub fn sub_imm(
    atomic: bool,
    long_mode: bool,
    op1: Operator1,
    imm: u32) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x81, 5],
        op1: Some(op1),
        op2: Some(Operator2::Imm(imm as u64, ImmByte::Bit32)),
    }
    .into_raw()
    .encode()
}

pub fn sub_signed_imm8(
    atomic: bool,
    long_mode: bool,
    op1: Operator1,
    imm: u32) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x83, 5],
        op1: Some(op1),
        op2: Some(Operator2::Imm(imm as u64, ImmByte::Bit32)),
    }
    .into_raw()
    .encode()
}

/// - sub: Subtract r32 from r/m32
pub fn sub(
    atomic: bool,
    long_mode: bool,
    op1: Operator1,
    op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x29],
        op1: Some(op1),
        op2: Some(Operator2::Register(op2)),
    }
    .into_raw()
    .encode()
}

/// - sub_rev: Subtract r/m32 from r32
pub fn sub_rev(
    atomic: bool,
    long_mode: bool,
    op1: Operator1,
    op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x2b],
        op1: Some(op1),
        op2: Some(Operator2::Register(op2)),
    }
    .into_raw()
    .encode()
}

// todo: sbb

pub fn dec(
    atomic: bool,
    long_mode: bool,
    op1: Operator1
) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0xff, 1],
        op1: Some(op1),
        op2: None,
    }
    .into_raw()
    .encode()
}

pub fn dec_reg32(
    atomic: bool,
    reg: Register32
) -> Vec<u8> {
    Inst {
        atomic,
        long_mode: false,
        opcode: vec![0x48],
        op1: None,
        op2: Some(Operator2::Register(TargetReg::from(reg as u8))),
    }
    .into_raw()
    .encode()
}

// ## nop

pub fn nop1() -> Vec<u8> {
    let r = Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0x90],
        op1: None,
        op2: None,
    }
    .into_raw()
    .encode();
    debug_assert_eq!(dbg!(r.len()), 1);
    r
}

pub fn nop3() -> Vec<u8> {
    let r = nop_multi_reg(Operator1::Direct(TargetReg::from(0)));
    debug_assert_eq!(r.len(), 3);
    r
}

pub fn nop4() -> Vec<u8> {
    let r = nop_multi_reg(Operator1::DeRef(TargetReg::from(0), u8::MAX as usize));
    debug_assert_eq!(r.len(), 4);
    r
}

pub fn nop7() -> Vec<u8> {
    let r = nop_multi_reg(Operator1::DeRef(TargetReg::from(0), u32::MAX as usize));
    debug_assert_eq!(r.len(), 7);
    r
}

pub fn nop_multi_reg(op1: Operator1) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0x0f, 0x1f],
        op1: Some(op1),
        op2: None,
    }
    .into_raw()
    .encode()
}

/// ## ret

pub fn near_ret() -> Vec<u8> {
    Inst::new(false, false, &[0xc3]).into_raw().encode()
}

pub fn far_ret() -> Vec<u8> {
    Inst::new(false, false, &[0xcb]).into_raw().encode()
}

pub fn near_ret_imm16(imm: u16) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0xc2],
        op1: None,
        op2: Some(Operator2::Imm(imm as u64, ImmByte::Bit16)),
    }
    .into_raw()
    .encode()
}

pub fn far_ret_imm16(imm: u16) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0xca],
        op1: None,
        op2: Some(Operator2::Imm(imm as u64, ImmByte::Bit16)),
    }
    .into_raw()
    .encode()
}
