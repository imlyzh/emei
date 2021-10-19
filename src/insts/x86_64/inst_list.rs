use crate::insts::{ImmByte, inst_dump_buf::JumpInst};

use super::{registers::*, Inst, Op1, Op2};

/// ## mov
/// - mov
pub fn mov(is_atomic: bool, is_long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x89],
        op1: Some(op1),
        op2: Some(Op2::Reg(op2)),
    }
    .into_raw()
    .encode()
}

pub fn mov_zero_extend_bit8(
    is_atomic: bool,
    is_long_mode: bool,
    op1: Op1,
    op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x0f, 0xb6],
        op1: Some(op1),
        op2: Some(Op2::Reg(op2)),
    }
    .into_raw()
    .encode()
}

pub fn mov_zero_extend_bit16(
    is_atomic: bool,
    is_long_mode: bool,
    op1: Op1,
    op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x0f, 0xb7],
        op1: Some(op1),
        op2: Some(Op2::Reg(op2)),
    }
    .into_raw()
    .encode()
}

pub fn mov_sign_extend_bit8(
    is_atomic: bool,
    is_long_mode: bool,
    op1: Op1,
    op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x0f, 0xbe],
        op1: Some(op1),
        op2: Some(Op2::Reg(op2)),
    }
    .into_raw()
    .encode()
}

pub fn mov_sign_extend_bit16(
    is_atomic: bool,
    is_long_mode: bool,
    op1: Op1,
    op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x0f, 0xbf],
        op1: Some(op1),
        op2: Some(Op2::Reg(op2)),
    }
    .into_raw()
    .encode()
}

pub fn mov_sign_extend_bit32(
    is_atomic: bool,
    is_long_mode: bool,
    op1: Op1,
    op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x63],
        op1: Some(op1),
        op2: Some(Op2::Reg(op2)),
    }
    .into_raw()
    .encode()
}

/// - mov_rev
/// mov_rev is the same as mov, but the source and destination operands are reversed.
pub fn mov_rev(is_atomic: bool, is_long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x8b],
        op1: Some(op1),
        op2: Some(Op2::Reg(op2)),
    }
    .into_raw()
    .encode()
}

/// - mov_imm[16/32/64(long_mode)]_into_reg
pub fn mov_imm_into_reg(is_atomic: bool, is_long_mode: bool, op1: TargetReg, op2: u64) -> Vec<u8> {
    let imm_byte = if is_long_mode {
        ImmByte::Bit64
    } else {
        ImmByte::Bit32
    };
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0xb8],
        op1: Some(Op1::Direct(op1)),
        op2: Some(Op2::Imm(op2, imm_byte)),
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
        op1: Some(Op1::Direct(op1)),
        op2: Some(Op2::Imm(op2, ImmByte::Bit32)),
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
        op2: Some(Op2::Imm(reg as u64, ImmByte::Bit8)),
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
        op2: Some(Op2::Imm(imm as u64, ImmByte::Bit32)),
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
        op2: Some(Op2::Imm(imm as u64, ImmByte::Bit32)),
    }
    .into_raw()
    .encode()
}

pub fn add_imm32(atomic: bool, long_mode: bool, op1: Op1, imm: u32) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x81],
        op1: Some(op1),
        op2: Some(Op2::Imm(imm as u64, ImmByte::Bit32)),
    }
    .into_raw()
    .encode()
}

pub fn add_imm8(atomic: bool, long_mode: bool, op1: Op1, imm: u8) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x83],
        op1: Some(op1),
        op2: Some(Op2::Imm(imm as u64, ImmByte::Bit8)),
    }
    .into_raw()
    .encode()
}

pub fn add(atomic: bool, long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x01],
        op1: Some(op1),
        op2: Some(Op2::Reg(op2)),
    }
    .into_raw()
    .encode()
}

/// - add_rev: add_rev is the same as add, but the source and destination operands are reversed.
pub fn add_rev(atomic: bool, long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x03],
        op1: Some(op1),
        op2: Some(Op2::Reg(op2)),
    }
    .into_raw()
    .encode()
}

pub fn inc(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
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
        op2: Some(Op2::Reg(TargetReg::from(op1 as u8))),
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
        op2: Some(Op2::Imm(imm as u64, ImmByte::Bit32)),
    }
    .into_raw()
    .encode()
}

pub fn sub_imm(
    atomic: bool,
    long_mode: bool,
    op1: Op1,
    imm: u32) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x81, 5],
        op1: Some(op1),
        op2: Some(Op2::Imm(imm as u64, ImmByte::Bit32)),
    }
    .into_raw()
    .encode()
}

pub fn sub_signed_imm8(
    atomic: bool,
    long_mode: bool,
    op1: Op1,
    imm: u32) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x83, 5],
        op1: Some(op1),
        op2: Some(Op2::Imm(imm as u64, ImmByte::Bit32)),
    }
    .into_raw()
    .encode()
}

/// - sub: Subtract r32 from r/m32
pub fn sub(
    atomic: bool,
    long_mode: bool,
    op1: Op1,
    op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x29],
        op1: Some(op1),
        op2: Some(Op2::Reg(op2)),
    }
    .into_raw()
    .encode()
}

/// - sub_rev: Subtract r/m32 from r32
pub fn sub_rev(
    atomic: bool,
    long_mode: bool,
    op1: Op1,
    op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x2b],
        op1: Some(op1),
        op2: Some(Op2::Reg(op2)),
    }
    .into_raw()
    .encode()
}

// todo: sbb

pub fn dec(
    atomic: bool,
    long_mode: bool,
    op1: Op1
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
        op2: Some(Op2::Reg(TargetReg::from(reg as u8))),
    }
    .into_raw()
    .encode()
}

/// ## cmp

pub fn cmp_first_reg_and_imm(
    long_mode: bool,
    imm: u32
) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x3d],
        op1: None,
        op2: Some(Op2::Imm(imm as u64, ImmByte::Bit32)),
    }
    .into_raw()
    .encode()
}

/// - cmp: Compare imm32 [with r/m32 | sign-extended to 64-bits with r/m64]
pub fn cmp_imm(
    long_mode: bool,
    op1: Op1,
    imm: u32
) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x81, 7],
        op1: Some(op1),
        op2: Some(Op2::Imm(imm as u64, ImmByte::Bit32)),
    }
    .into_raw()
    .encode()
}

/// - cmp_imm8: Compare imm8 with r/m8
pub fn cmp_imm8(
    long_mode: bool,
    op1: Op1,
    imm: u8
) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x83, 7],
        op1: Some(op1),
        op2: Some(Op2::Imm(imm as u64, ImmByte::Bit8)),
    }
    .into_raw()
    .encode()
}

/// - cmp: Compare r32 with r/m32(64)
pub fn cmp(
    long_mode: bool,
    op1: Op1,
    op2: TargetReg
) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x39, 7],
        op1: Some(op1),
        op2: Some(Op2::Reg(op2)),
    }
    .into_raw()
    .encode()
}

/// - cmp_rev: Compare r/m32(64) with r32
pub fn cmp_rev(
    long_mode: bool,
    op1: Op1,
    op2: TargetReg
) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x3b, 7],
        op1: Some(op1),
        op2: Some(Op2::Reg(op2)),
    }
    .into_raw()
    .encode()
}

/// - cmps: Compares quadword at address (R|E)SI with quadword at address (R|E)DI and sets the status flags accordingly.
pub fn cmps(long_mode: bool) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0xa7],
        op1: None,
        op2: None,
    }
    .into_raw()
    .encode()
}

/// ## jmp

pub fn jmp(
    label: String
) -> JumpInst {
    let opcodes = Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0xe9],
        op1: None,
        op2: Some(Op2::Imm(0, ImmByte::Bit32)),
    }
    .into_raw()
    .encode();
    JumpInst::from(opcodes, ImmByte::Bit32, label)
}

pub fn jmp_to_reg(
    reg: TargetReg
) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0xff, 4],
        op1: Some(Op1::Direct(reg)),
        op2: None,
    }
    .into_raw()
    .encode()
}

/// ## conditional jump

macro_rules! impl_cond_jump_inst {
    ($name:ident, $expr: expr) => {
        pub fn $name(
            label: String
        ) -> JumpInst {
            let opcodes = Inst {
                atomic: false,
                long_mode: false,
                opcode: $expr,
                op1: None,
                op2: Some(Op2::Imm(0, ImmByte::Bit32)),
            }
            .into_raw()
            .encode();
            JumpInst::from(opcodes, ImmByte::Bit32, label)
        }
    };
}


impl_cond_jump_inst!(ja, vec![0x0f, 0x87]);
impl_cond_jump_inst!(jb, vec![0x0f, 0x82]);
impl_cond_jump_inst!(jc, vec![0x0f, 0x82]);
impl_cond_jump_inst!(je, vec![0x0f, 0x84]);
impl_cond_jump_inst!(jg, vec![0x0f, 0x8f]);
impl_cond_jump_inst!(jl, vec![0x0f, 0x8c]);
impl_cond_jump_inst!(jo, vec![0x0f, 0x80]);
impl_cond_jump_inst!(jp, vec![0x0f, 0x8a]);
impl_cond_jump_inst!(js, vec![0x0f, 0x88]);
impl_cond_jump_inst!(jz, vec![0x0f, 0x84]);
impl_cond_jump_inst!(jae, vec![0x0f, 0x83]);
impl_cond_jump_inst!(jbe, vec![0x0f, 0x86]);
impl_cond_jump_inst!(jge, vec![0x0f, 0x8d]);
impl_cond_jump_inst!(jle, vec![0x0f, 0x8e]);
impl_cond_jump_inst!(jpe, vec![0x0f, 0x8a]);
impl_cond_jump_inst!(jpo, vec![0x0f, 0x8b]);
impl_cond_jump_inst!(jna, vec![0x0f, 0x86]);
impl_cond_jump_inst!(jnb, vec![0x0f, 0x83]);
impl_cond_jump_inst!(jnc, vec![0x0f, 0x83]);
impl_cond_jump_inst!(jne, vec![0x0f, 0x85]);
impl_cond_jump_inst!(jng, vec![0x0f, 0x8e]);
impl_cond_jump_inst!(jnl, vec![0x0f, 0x8d]);
impl_cond_jump_inst!(jno, vec![0x0f, 0x81]);
impl_cond_jump_inst!(jnp, vec![0x0f, 0x8b]);
impl_cond_jump_inst!(jns, vec![0x0f, 0x89]);
impl_cond_jump_inst!(jnz, vec![0x0f, 0x85]);
impl_cond_jump_inst!(jnae, vec![0x0f, 0x82]);
impl_cond_jump_inst!(jnbe, vec![0x0f, 0x87]);
impl_cond_jump_inst!(jnge, vec![0x0f, 0x8c]);
impl_cond_jump_inst!(jnle, vec![0x0f, 0x8f]);


/// ## nop

#[inline]
pub fn nop() -> Vec<u8> {
    nop1()
}

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

pub fn nop2() -> Vec<u8> {
    let r = Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![66, 0x90],
        op1: None,
        op2: None,
    }
    .into_raw()
    .encode();
    debug_assert_eq!(dbg!(r.len()), 2);
    r
}

pub fn nop3() -> Vec<u8> {
    let r = nop_multi_reg(Op1::Direct(TargetReg::from(0)));
    debug_assert_eq!(r.len(), 3);
    r
}

pub fn nop4() -> Vec<u8> {
    let r = nop_multi_reg(Op1::DeRef(TargetReg::from(0), u8::MAX as usize));
    debug_assert_eq!(r.len(), 4);
    r
}

pub fn nop5() -> Vec<u8> {
    let r = nop_multi_reg(Op1::ScaleBase(TargetReg::from(0), TargetReg::from(0), ScaledIndex::Id, u8::MAX as usize));
    debug_assert_eq!(r.len(), 5);
    r
}

pub fn nop6() -> Vec<u8> {
    let r = Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![66, 0x0f, 0x1f],
        op1: Some(Op1::ScaleBase(TargetReg::from(0), TargetReg::from(0), ScaledIndex::Id, u8::MAX as usize)),
        op2: None,
    }
    .into_raw()
    .encode();
    debug_assert_eq!(dbg!(r.len()), 6);
    r
}

pub fn nop7() -> Vec<u8> {
    let r = nop_multi_reg(Op1::DeRef(TargetReg::from(0), u32::MAX as usize));
    debug_assert_eq!(r.len(), 7);
    r
}

pub fn nop8() -> Vec<u8> {
    let r = nop_multi_reg(Op1::ScaleBase(TargetReg::from(0), TargetReg::from(0), ScaledIndex::Id, u32::MAX as usize));
    debug_assert_eq!(r.len(), 8);
    r
}

pub fn nop9() -> Vec<u8> {
    let r = Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![66, 0x0f, 0x1f],
        op1: Some(Op1::ScaleBase(TargetReg::from(0), TargetReg::from(0), ScaledIndex::Id, u32::MAX as usize)),
        op2: None,
    }
    .into_raw()
    .encode();
    debug_assert_eq!(dbg!(r.len()), 9);
    r
}

pub fn nop_multi_reg(op1: Op1) -> Vec<u8> {
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
        op2: Some(Op2::Imm(imm as u64, ImmByte::Bit16)),
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
        op2: Some(Op2::Imm(imm as u64, ImmByte::Bit16)),
    }
    .into_raw()
    .encode()
}
