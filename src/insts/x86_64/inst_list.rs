use crate::insts::{
    inst_dump_buf::JumpInst,
    x86_64::{Imm, SSEInst},
    ImmByte,
};

use super::{registers::*, Inst, Op1};

/// ## mov
/// - mov
pub fn mov(is_atomic: bool, is_long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x89],
        op1: Some(op1),
        op2: Some(op2),
        imm: None,
    }
    .into_raw()
    .encode()
}

pub fn mov_zero_extend_bit8(
    is_atomic: bool,
    is_long_mode: bool,
    op1: Op1,
    op2: TargetReg,
) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x0f, 0xb6],
        op1: Some(op1),
        op2: Some(op2),
        imm: None,
    }
    .into_raw()
    .encode()
}

pub fn mov_zero_extend_bit16(
    is_atomic: bool,
    is_long_mode: bool,
    op1: Op1,
    op2: TargetReg,
) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x0f, 0xb7],
        op1: Some(op1),
        op2: Some(op2),
        imm: None,
    }
    .into_raw()
    .encode()
}

pub fn mov_sign_extend_bit8(
    is_atomic: bool,
    is_long_mode: bool,
    op1: Op1,
    op2: TargetReg,
) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x0f, 0xbe],
        op1: Some(op1),
        op2: Some(op2),
        imm: None,
    }
    .into_raw()
    .encode()
}

pub fn mov_sign_extend_bit16(
    is_atomic: bool,
    is_long_mode: bool,
    op1: Op1,
    op2: TargetReg,
) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x0f, 0xbf],
        op1: Some(op1),
        op2: Some(op2),
        imm: None,
    }
    .into_raw()
    .encode()
}

pub fn mov_sign_extend_bit32(
    is_atomic: bool,
    is_long_mode: bool,
    op1: Op1,
    op2: TargetReg,
) -> Vec<u8> {
    Inst {
        atomic: is_atomic,
        long_mode: is_long_mode,
        opcode: vec![0x63],
        op1: Some(op1),
        op2: Some(op2),
        imm: None,
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
        op2: Some(op2),
        imm: None,
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
        op2: None,
        imm: Some(Imm(op2, imm_byte)),
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
        op2: None,
        imm: Some(Imm(op2, ImmByte::Bit32)),
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
        op2: None,
        imm: Some(Imm(reg as u64, ImmByte::Bit8)),
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
        op2: None,
        imm: Some(Imm(imm as u64, ImmByte::Bit32)),
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
        op2: None,
        imm: Some(Imm(imm as u64, ImmByte::Bit32)),
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
        op2: None,
        imm: Some(Imm(imm as u64, ImmByte::Bit32)),
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
        op2: None,
        imm: Some(Imm(imm as u64, ImmByte::Bit8)),
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
        op2: Some(op2),
        imm: None,
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
        op2: Some(op2),
        imm: None,
    }
    .into_raw()
    .encode()
}

pub fn lea(atomic: bool, long_mode: bool, op1: TargetReg, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x8d],
        op1: Some(Op1::Direct(op1)),
        op2: Some(op2),
        imm: None,
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
        imm: None,
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
        op2: None,
        imm: Some(Imm::from(op1)),
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
        op2: None,
        imm: Some(Imm::from(imm)),
    }
    .into_raw()
    .encode()
}

pub fn sub_imm(atomic: bool, long_mode: bool, op1: Op1, imm: u32) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x81, 5],
        op1: Some(op1),
        op2: None,
        imm: Some(Imm::from(imm)),
    }
    .into_raw()
    .encode()
}

pub fn sub_signed_imm8(atomic: bool, long_mode: bool, op1: Op1, imm: u8) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x83, 5],
        op1: Some(op1),
        op2: None,
        imm: Some(Imm::from(imm)),
    }
    .into_raw()
    .encode()
}

/// - sub: Subtract r32 from r/m32
pub fn sub(atomic: bool, long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x29],
        op1: Some(op1),
        op2: Some(op2),
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - sub_rev: Subtract r/m32 from r32
pub fn sub_rev(atomic: bool, long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x2b],
        op1: Some(op1),
        op2: Some(op2),
        imm: None,
    }
    .into_raw()
    .encode()
}

// todo: sbb

pub fn dec(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0xff, 1],
        op1: Some(op1),
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

pub fn dec_reg32(atomic: bool, reg: Register32) -> Vec<u8> {
    Inst {
        atomic,
        long_mode: false,
        opcode: vec![0x48],
        op1: None,
        op2: None,
        imm: Some(Imm::from(reg)),
    }
    .into_raw()
    .encode()
}

/// neg

pub fn neg(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0xf6],
        op1: Some(op1),
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

/// ## mul

pub fn mul_byte_first_reg(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0xf6, 4],
        op1: Some(op1),
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

pub fn mul_first_reg(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0xf7, 4],
        op1: Some(op1),
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

pub fn imul_byte_first_reg(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0xf6, 5],
        op1: Some(op1),
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

pub fn imul_first_reg(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0xf7, 5],
        op1: Some(op1),
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

pub fn imul_reg(atomic: bool, long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x0f, 0xaf],
        op1: Some(op1),
        op2: Some(op2),
        imm: None,
    }
    .into_raw()
    .encode()
}

pub fn imul_reg_and_imm8(
    atomic: bool,
    long_mode: bool,
    op1: Op1,
    op2: TargetReg,
    imm: u8,
) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x6b],
        op1: Some(op1),
        op2: Some(op2),
        imm: Some(Imm::from(imm)),
    }
    .into_raw()
    .encode()
}

pub fn imul_reg_and_imm32(
    atomic: bool,
    long_mode: bool,
    op1: Op1,
    op2: TargetReg,
    imm: u32,
) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0x69],
        op1: Some(op1),
        op2: Some(op2),
        imm: Some(Imm::from(imm)),
    }
    .into_raw()
    .encode()
}

/// ## div

pub fn div_byte_first_reg(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0xf6, 6],
        op1: Some(op1),
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

pub fn div_first_reg(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0xf7, 6],
        op1: Some(op1),
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

pub fn idiv_byte_first_reg(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0xf6, 6],
        op1: Some(op1),
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

pub fn idiv_first_reg(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    Inst {
        atomic,
        long_mode,
        opcode: vec![0xf7, 6],
        op1: Some(op1),
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

/// cbw

/// EAX ← sign-extend of AX.
/// RAX ← sign-extend of EAX(long mode only).
pub fn sign_extend(long_mode: bool) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x98],
        op1: None,
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

/// EDX:EAX ← sign-extend of EAX
/// RDX:RAX ← sign-extend of RAX(long mode only).
pub fn sign_extend2(long_mode: bool) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x98],
        op1: None,
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

/// ## cmp

pub fn cmp_first_reg_and_imm(long_mode: bool, imm: u32) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x3d],
        op1: None,
        op2: None,
        imm: Some(Imm::from(imm)),
    }
    .into_raw()
    .encode()
}

/// - cmp: Compare imm32 [with r/m32 | sign-extended to 64-bits with r/m64]
pub fn cmp_imm(long_mode: bool, op1: Op1, imm: u32) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x81, 7],
        op1: Some(op1),
        op2: None,
        imm: Some(Imm(imm as u64, ImmByte::Bit32)),
    }
    .into_raw()
    .encode()
}

/// - cmp_imm8: Compare imm8 with r/m8
pub fn cmp_imm8(long_mode: bool, op1: Op1, imm: u8) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x83, 7],
        op1: Some(op1),
        op2: None,
        imm: Some(Imm(imm as u64, ImmByte::Bit8)),
    }
    .into_raw()
    .encode()
}

/// - cmp: Compare r32 with r/m32(64)
pub fn cmp(long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x39, 7],
        op1: Some(op1),
        op2: Some(op2),
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - cmp_rev: Compare r/m32(64) with r32
pub fn cmp_rev(long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x3b, 7],
        op1: Some(op1),
        op2: Some(op2),
        imm: None,
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
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - test_first_reg

pub fn test_first_reg_and_imm8(imm: u8) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0xa8],
        op1: None,
        op2: None,
        imm: Some(Imm(imm as u64, ImmByte::Bit8)),
    }
    .into_raw()
    .encode()
}

pub fn test_first_reg(long_mode: bool, imm: Imm) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0xa9],
        op1: None,
        op2: None,
        imm: Some(imm),
    }
    .into_raw()
    .encode()
}

pub fn test_imm8(long_mode: bool, op1: Op1, imm: u8) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0xf6, 0],
        op1: Some(op1),
        op2: None,
        imm: Some(Imm(imm as u64, ImmByte::Bit8)),
    }
    .into_raw()
    .encode()
}

/// - test_imm: Test imm32 [with r/m32 | sign-extended to 64-bits with r/m64]
pub fn test_imm(long_mode: bool, op1: Op1, op2: TargetReg, imm: u32) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0xf7, 0],
        op1: Some(op1),
        op2: Some(op2),
        imm: Some(Imm(imm as u64, ImmByte::Bit32)),
    }
    .into_raw()
    .encode()
}

// - test_u8
pub fn test_u8(long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x84],
        op1: Some(op1),
        op2: Some(op2),
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - test
pub fn test(long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x85],
        op1: Some(op1),
        op2: Some(op2),
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - int1
#[inline]
pub fn int1() -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0xf1],
        op1: None,
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - int3
#[inline]
pub fn int3() -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0xcc],
        op1: None,
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - int
#[inline]
pub fn int(imm: u8) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0xcd],
        op1: None,
        op2: None,
        imm: Some(Imm::from(imm)),
    }
    .into_raw()
    .encode()
}

/// - into
#[inline]
pub fn into() -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0xce],
        op1: None,
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - syscall
#[inline]
pub fn syscall() -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0x0f, 0x05],
        op1: None,
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - sysenter
#[inline]
pub fn sysenter() -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0x0f, 0x34],
        op1: None,
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - call

pub fn call_relative_addr(label: String) -> JumpInst {
    let opcodes = Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0xe9],
        op1: None,
        op2: None,
        imm: Some(Imm(0, ImmByte::Bit32)),
    }
    .into_raw()
    .encode();
    JumpInst::from(opcodes, ImmByte::Bit32, label)
}

/// jit use it
pub fn call_addr_literal(addr: Imm) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0x9a],
        op1: None,
        op2: None,
        imm: Some(addr),
    }
    .into_raw()
    .encode()
}

pub fn call_reg(op1: Op1) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0x9a],
        op1: Some(op1),
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

/// ## jmp

pub fn jmp(label: String) -> JumpInst {
    let opcodes = Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0xe9],
        op1: None,
        op2: None,
        imm: Some(Imm(0, ImmByte::Bit32)),
    }
    .into_raw()
    .encode();
    JumpInst::from(opcodes, ImmByte::Bit32, label)
}

/// jit use it
pub fn jmp_addr_literal(addr: Imm) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0xea],
        op1: None,
        op2: None,
        imm: Some(addr),
    }
    .into_raw()
    .encode()
}

/// runtime_symbol only
pub fn jmp_to_runtime_symbol(label: String) -> JumpInst {
    let opcodes = Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0xea],
        op1: None,
        op2: None,
        imm: Some(Imm(0, ImmByte::Bit32)),
    }
    .into_raw()
    .encode();
    JumpInst::from(opcodes, ImmByte::Bit32, label)
}

pub fn jmp_to_reg(reg: TargetReg) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![0xff, 4],
        op1: Some(Op1::Direct(reg)),
        op2: None,
        imm: None,
    }
    .into_raw()
    .encode()
}

/// ## conditional jump

macro_rules! impl_cond_jump_inst {
    ($name:ident, $expr: expr) => {
        pub fn $name(label: String) -> JumpInst {
            let opcodes = Inst {
                atomic: false,
                long_mode: false,
                opcode: $expr,
                op1: None,
                op2: None,
                imm: Some(Imm(0, ImmByte::Bit32)),
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

/// ## logic inst

/// ### and

pub fn and_first_reg_imm32(long_mode: bool, imm: u32) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x25],
        op1: None,
        op2: None,
        imm: Some(Imm::from(imm)),
    }
    .into_raw()
    .encode()
}

pub fn and_reg_imm32(long_mode: bool, op1: Op1, imm: u32) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x81, 4],
        op1: Some(op1),
        op2: None,
        imm: Some(Imm::from(imm)),
    }
    .into_raw()
    .encode()
}

pub fn and_reg_imm8(long_mode: bool, op1: Op1, imm: u8) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x83, 4],
        op1: Some(op1),
        op2: None,
        imm: Some(Imm::from(imm)),
    }
    .into_raw()
    .encode()
}

pub fn and(long_mode: bool, op1: Op1, reg: TargetReg) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x21],
        op1: Some(op1),
        op2: Some(reg),
        imm: None,
    }
    .into_raw()
    .encode()
}

pub fn and_rev(long_mode: bool, op1: Op1, reg: TargetReg) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x23],
        op1: Some(op1),
        op2: Some(reg),
        imm: None,
    }
    .into_raw()
    .encode()
}

/// ### or

pub fn or_first_reg_imm32(long_mode: bool, imm: u32) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x0d],
        op1: None,
        op2: None,
        imm: Some(Imm::from(imm)),
    }
    .into_raw()
    .encode()
}

pub fn or_reg_imm32(long_mode: bool, op1: Op1, imm: u32) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x81, 1],
        op1: Some(op1),
        op2: None,
        imm: Some(Imm::from(imm)),
    }
    .into_raw()
    .encode()
}

pub fn or_reg_imm8(long_mode: bool, op1: Op1, imm: u8) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x83, 1],
        op1: Some(op1),
        op2: None,
        imm: Some(Imm::from(imm)),
    }
    .into_raw()
    .encode()
}

pub fn or(long_mode: bool, op1: Op1, reg: TargetReg) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x09],
        op1: Some(op1),
        op2: Some(reg),
        imm: None,
    }
    .into_raw()
    .encode()
}

pub fn or_rev(long_mode: bool, op1: Op1, reg: TargetReg) -> Vec<u8> {
    Inst {
        atomic: false,
        long_mode,
        opcode: vec![0x0b],
        op1: Some(op1),
        op2: Some(reg),
        imm: None,
    }
    .into_raw()
    .encode()
}

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
        imm: None,
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
        imm: None,
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
    let r = nop_multi_reg(Op1::ScaleBase(
        TargetReg::from(0),
        TargetReg::from(0),
        ScaledIndex::Id,
        u8::MAX as usize,
    ));
    debug_assert_eq!(r.len(), 5);
    r
}

pub fn nop6() -> Vec<u8> {
    let r = Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![66, 0x0f, 0x1f],
        op1: Some(Op1::ScaleBase(
            TargetReg::from(0),
            TargetReg::from(0),
            ScaledIndex::Id,
            u8::MAX as usize,
        )),
        op2: None,
        imm: None,
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
    let r = nop_multi_reg(Op1::ScaleBase(
        TargetReg::from(0),
        TargetReg::from(0),
        ScaledIndex::Id,
        u32::MAX as usize,
    ));
    debug_assert_eq!(r.len(), 8);
    r
}

pub fn nop9() -> Vec<u8> {
    let r = Inst {
        atomic: false,
        long_mode: false,
        opcode: vec![66, 0x0f, 0x1f],
        op1: Some(Op1::ScaleBase(
            TargetReg::from(0),
            TargetReg::from(0),
            ScaledIndex::Id,
            u32::MAX as usize,
        )),
        op2: None,
        imm: None,
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
        imm: None,
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
        op2: None,
        imm: Some(Imm(imm as u64, ImmByte::Bit16)),
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
        op2: None,
        imm: Some(Imm(imm as u64, ImmByte::Bit16)),
    }
    .into_raw()
    .encode()
}

/// ## SSE inst

/// - movss
/// movss xmm1, xmm2/m32
/// Merge scalar single-precision floating-point value from xmm2 to xmm1 register.
/// Load scalar single-precision floating-point value from m32 to xmm1 register.
pub fn movss(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    SSEInst {
        opcode: vec![0xf3, 0x0f, 0x10],
        op1: Some(op1),
        op2: Some(TargetReg::from(op2 as u8)),
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - movss_rev
/// movss xmm2/m32, xmm1
pub fn movss_rev(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    SSEInst {
        opcode: vec![0xf3, 0x0f, 0x11],
        op1: Some(op1),
        op2: Some(TargetReg::from(op2 as u8)),
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - movsd
/// movsd xmm1, xmm2/m32
/// Move scalar double-precision floating-point value from xmm2 to xmm1 register.
/// Load scalar double-precision floating-point value from m64 to xmm1 register.
pub fn movsd(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    SSEInst {
        opcode: vec![0xf2, 0x0f, 0x10],
        op1: Some(op1),
        op2: Some(TargetReg::from(op2 as u8)),
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - movsd_rev
/// movsd xmm2/m32, xmm1
pub fn movsd_rev(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    SSEInst {
        opcode: vec![0xf2, 0x0f, 0x11],
        op1: Some(op1),
        op2: Some(TargetReg::from(op2 as u8)),
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - addss
/// addss xmm1, xmm2/m32
/// Add scalar single-precision floating-point value from xmm2 to xmm1 register.
pub fn addss(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    SSEInst {
        opcode: vec![0xf3, 0x0f, 0x58],
        op1: Some(op1),
        op2: Some(TargetReg::from(op2 as u8)),
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - addsd
/// addsd xmm1, xmm2/m64
/// Add scalar double-precision floating-point value from xmm2 to xmm1 register.
pub fn addsd(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    SSEInst {
        opcode: vec![0xf2, 0x0f, 0x58],
        op1: Some(op1),
        op2: Some(TargetReg::from(op2 as u8)),
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - subss
/// subss xmm1, xmm2/m32
/// Subtract scalar single-precision floating-point value from xmm2 from xmm1 register.
pub fn subss(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    SSEInst {
        opcode: vec![0xf3, 0x0f, 0x5c],
        op1: Some(op1),
        op2: Some(TargetReg::from(op2 as u8)),
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - subsd
/// subsd xmm1, xmm2/m64
/// Subtract scalar double-precision floating-point value from xmm2 from xmm1 register.
pub fn subsd(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    SSEInst {
        opcode: vec![0xf2, 0x0f, 0x5c],
        op1: Some(op1),
        op2: Some(TargetReg::from(op2 as u8)),
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - mulss
/// mulss xmm1, xmm2/m32
/// Multiply scalar single-precision floating-point value from xmm2 to xmm1 register.
pub fn mulss(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    SSEInst {
        opcode: vec![0xf3, 0x0f, 0x59],
        op1: Some(op1),
        op2: Some(TargetReg::from(op2 as u8)),
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - mulsd
/// mulsd xmm1, xmm2/m64
/// Multiply scalar double-precision floating-point value from xmm2 to xmm1 register.
pub fn mulsd(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    SSEInst {
        opcode: vec![0xf2, 0x0f, 0x59],
        op1: Some(op1),
        op2: Some(TargetReg::from(op2 as u8)),
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - divss
/// divss xmm1, xmm2/m32
/// Divide scalar single-precision floating-point value from xmm2 by xmm1 register.
pub fn divss(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    SSEInst {
        opcode: vec![0xf3, 0x0f, 0x5e],
        op1: Some(op1),
        op2: Some(TargetReg::from(op2 as u8)),
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - divsd
/// divsd xmm1, xmm2/m64
/// Divide scalar double-precision floating-point value from xmm2 by xmm1 register.
pub fn divsd(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    SSEInst {
        opcode: vec![0xf2, 0x0f, 0x5e],
        op1: Some(op1),
        op2: Some(TargetReg::from(op2 as u8)),
        imm: None,
    }
    .into_raw()
    .encode()
}

/// - sqrtss/sqrtsd operations
#[repr(u8)]
pub enum FcmpOp {
    Eq = 0,
    Lt = 1,
    Le = 2,
    Unord = 3,
    Neq = 4,
    Nlt = 5,
    Nle = 6,
    Ord = 7,
}

/// - cmpss
/// cmpss xmm1, xmm2/m32
/// Compare scalar single-precision floating-point value from xmm2 to xmm1 register.
pub fn cmpss(op1: Op1, op2: RegisterXmm, imm: FcmpOp) -> Vec<u8> {
    SSEInst {
        opcode: vec![0xf3, 0x0f, 0xC2],
        op1: Some(op1),
        op2: Some(TargetReg::from(op2 as u8)),
        imm: Some(Imm(imm as u64, ImmByte::Bit8)),
    }
    .into_raw()
    .encode()
}

/// - cmpsd
/// cmpsd xmm1, xmm2/m64
/// Compare scalar double-precision floating-point value from xmm2 to xmm1 register.
pub fn cmpsd(op1: Op1, op2: RegisterXmm, imm: FcmpOp) -> Vec<u8> {
    SSEInst {
        opcode: vec![0xf2, 0x0f, 0xC2],
        op1: Some(op1),
        op2: Some(TargetReg::from(op2 as u8)),
        imm: Some(Imm(imm as u64, ImmByte::Bit8)),
    }
    .into_raw()
    .encode()
}

/// - sqrtss
/// sqrtss xmm1, xmm2/m32
/// Compute square root of scalar single-precision floating-point value in xmm2 and store result in xmm1.
pub fn sqrtss(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    SSEInst {
        opcode: vec![0xf3, 0x0f, 0x51],
        op1: Some(op1),
        op2: Some(TargetReg::from(op2 as u8)),
        imm: None,
    }
    .into_raw()
    .encode()
}
