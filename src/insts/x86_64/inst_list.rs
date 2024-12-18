use crate::insts::x86_64::{
    inst,
    registers::{RegisterXmm, ScaledIndex, TargetReg},
    sse_inst, Imm, Op1,
};

use super::ImmByte;

/// ## mov
/// - mov
/// mov op1(reg/mem) into op2(reg)
pub fn mov(is_atomic: bool, is_long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    inst(is_atomic, is_long_mode, &[0x89], Some(op1), Some(op2), None)
}

pub fn mov_zero_extend_bit8(
    is_atomic: bool,
    is_long_mode: bool,
    op1: Op1,
    op2: TargetReg,
) -> Vec<u8> {
    inst(
        is_atomic,
        is_long_mode,
        &[0x0f, 0xb6],
        Some(op1),
        Some(op2),
        None,
    )
}

pub fn mov_zero_extend_bit16(
    is_atomic: bool,
    is_long_mode: bool,
    op1: Op1,
    op2: TargetReg,
) -> Vec<u8> {
    inst(
        is_atomic,
        is_long_mode,
        &[0x0f, 0xb7],
        Some(op1),
        Some(op2),
        None,
    )
}

pub fn mov_sign_extend_bit8(
    is_atomic: bool,
    is_long_mode: bool,
    op1: Op1,
    op2: TargetReg,
) -> Vec<u8> {
    inst(
        is_atomic,
        is_long_mode,
        &[0x0f, 0xbe],
        Some(op1),
        Some(op2),
        None,
    )
}

pub fn mov_sign_extend_bit16(
    is_atomic: bool,
    is_long_mode: bool,
    op1: Op1,
    op2: TargetReg,
) -> Vec<u8> {
    inst(
        is_atomic,
        is_long_mode,
        &[0x0f, 0xbf],
        Some(op1),
        Some(op2),
        None,
    )
}

pub fn mov_sign_extend_bit32(
    is_atomic: bool,
    is_long_mode: bool,
    op1: Op1,
    op2: TargetReg,
) -> Vec<u8> {
    inst(is_atomic, is_long_mode, &[0x63], Some(op1), Some(op2), None)
}

/// - mov_rev
/// mov_rev is the same as mov, but the source and destination operands are reversed.
pub fn mov_rev(is_atomic: bool, is_long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    inst(is_atomic, is_long_mode, &[0x8b], Some(op1), Some(op2), None)
}

/// - mov_imm[16/32/64(long_mode)]_into_reg
pub fn mov_imm_into_reg(is_atomic: bool, is_long_mode: bool, op1: TargetReg, op2: u64) -> Vec<u8> {
    let imm_byte = if is_long_mode {
        ImmByte::Bit64
    } else {
        ImmByte::Bit32
    };
    inst(
        is_atomic,
        is_long_mode,
        &[0xb8],
        Some(Op1::Direct(op1)),
        None,
        Some(Imm(op2, imm_byte)),
    )
}

pub fn imm_sign_extend_into_reg(
    is_atomic: bool,
    is_long_mode: bool,
    op1: TargetReg,
    op2: u64,
) -> Vec<u8> {
    inst(
        is_atomic,
        is_long_mode,
        &[0x7c],
        Some(Op1::Direct(op1)),
        None,
        Some(Imm(op2, ImmByte::Bit32)),
    )
}

///  movs == movsq
pub fn movs(atomic: bool) -> Vec<u8> {
    inst(atomic, false, &[0xa5], None, None, None)
}

/// ## push
///
/// - push reg
pub fn push_reg(atomic: bool, reg: TargetReg) -> Vec<u8> {
    inst(
        atomic,
        false,
        &[0x60],
        None,
        None,
        Some(Imm(reg as u64, ImmByte::Bit8)),
    )
}

/// - push_imm
pub fn push_imm(atomic: bool, imm: u32) -> Vec<u8> {
    inst(
        atomic,
        false,
        &[0x60],
        None,
        None,
        Some(Imm(imm as u64, ImmByte::Bit32)),
    )
}

/// - push_all
pub fn push_all(atomic: bool) -> Vec<u8> {
    inst(atomic, false, &[0x60], None, None, None)
}

/// ## add
/// - add_to_eax(rax)

pub fn add_first_reg(atomic: bool, long_mode: bool, imm: u32) -> Vec<u8> {
    inst(
        atomic,
        long_mode,
        &[0x05],
        None,
        None,
        Some(Imm(imm as u64, ImmByte::Bit32)),
    )
}

pub fn add_imm32(atomic: bool, long_mode: bool, op1: Op1, imm: u32) -> Vec<u8> {
    inst(
        atomic,
        long_mode,
        &[0x81],
        Some(op1),
        None,
        Some(Imm(imm as u64, ImmByte::Bit32)),
    )
}

pub fn add_imm8(atomic: bool, long_mode: bool, op1: Op1, imm: u8) -> Vec<u8> {
    inst(
        atomic,
        long_mode,
        &[0x83],
        Some(op1),
        None,
        Some(Imm(imm as u64, ImmByte::Bit8)),
    )
}

pub fn add(atomic: bool, long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    inst(atomic, long_mode, &[0x01], Some(op1), Some(op2), None)
}

/// - add_rev: add_rev is the same as add, but the source and destination operands are reversed.
pub fn add_rev(atomic: bool, long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    inst(atomic, long_mode, &[0x03], Some(op1), Some(op2), None)
}

pub fn lea(atomic: bool, long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    inst(atomic, long_mode, &[0x8d], Some(op1), Some(op2), None)
}

pub fn inc(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    inst(atomic, long_mode, &[0xfe], Some(op1), None, None)
}

// pub fn inc_reg32(atomic: bool, op1: Register32) -> Vec<u8> {
//     inst(atomic, false, &[0x40], None, None, Some(Imm::from(op1)))
// }

/// ## sub

pub fn sub_first_reg(atomic: bool, imm: u32) -> Vec<u8> {
    inst(atomic, false, &[0x2d], None, None, Some(Imm::from(imm)))
}

pub fn sub_imm(atomic: bool, long_mode: bool, op1: Op1, imm: u32) -> Vec<u8> {
    inst(
        atomic,
        long_mode,
        &[0x81, 5],
        Some(op1),
        None,
        Some(Imm::from(imm)),
    )
}

pub fn sub_signed_imm8(atomic: bool, long_mode: bool, op1: Op1, imm: u8) -> Vec<u8> {
    inst(
        atomic,
        long_mode,
        &[0x83, 5],
        Some(op1),
        None,
        Some(Imm::from(imm)),
    )
}

/// - sub: Subtract r32 from r/m32
pub fn sub(atomic: bool, long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    inst(atomic, long_mode, &[0x29], Some(op1), Some(op2), None)
}

/// - sub_rev: Subtract r/m32 from r32
pub fn sub_rev(atomic: bool, long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    inst(atomic, long_mode, &[0x2b], Some(op1), Some(op2), None)
}

// todo: sbb

pub fn dec(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    inst(atomic, long_mode, &[0xff, 1], Some(op1), None, None)
}

// pub fn dec_reg32(atomic: bool, reg: Register32) -> Vec<u8> {
//     inst(atomic, false, &[0x48], None, None, Some(Imm::from(reg)))
// }

/// neg

pub fn neg(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    inst(atomic, long_mode, &[0xf6], Some(op1), None, None)
}

/// ## mul

pub fn mul_byte_first_reg(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    inst(atomic, long_mode, &[0xf6, 4], Some(op1), None, None)
}

pub fn mul_first_reg(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    inst(atomic, long_mode, &[0xf7, 4], Some(op1), None, None)
}

pub fn imul_byte_first_reg(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    inst(atomic, long_mode, &[0xf6, 5], Some(op1), None, None)
}

pub fn imul_first_reg(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    inst(atomic, long_mode, &[0xf7, 5], Some(op1), None, None)
}

pub fn imul_reg(atomic: bool, long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    inst(atomic, long_mode, &[0x0f, 0xaf], Some(op1), Some(op2), None)
}

pub fn imul_reg_and_imm8(
    atomic: bool,
    long_mode: bool,
    op1: Op1,
    op2: TargetReg,
    imm: u8,
) -> Vec<u8> {
    inst(
        atomic,
        long_mode,
        &[0x6b],
        Some(op1),
        Some(op2),
        Some(Imm::from(imm)),
    )
}

pub fn imul_reg_and_imm32(
    atomic: bool,
    long_mode: bool,
    op1: Op1,
    op2: TargetReg,
    imm: u32,
) -> Vec<u8> {
    inst(
        atomic,
        long_mode,
        &[0x69],
        Some(op1),
        Some(op2),
        Some(Imm::from(imm)),
    )
}

/// ## div

pub fn div_byte_first_reg(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    inst(atomic, long_mode, &[0xf6, 6], Some(op1), None, None)
}

pub fn div_first_reg(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    inst(atomic, long_mode, &[0xf7, 6], Some(op1), None, None)
}

pub fn idiv_byte_first_reg(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    inst(atomic, long_mode, &[0xf6, 6], Some(op1), None, None)
}

pub fn idiv_first_reg(atomic: bool, long_mode: bool, op1: Op1) -> Vec<u8> {
    inst(atomic, long_mode, &[0xf7, 6], Some(op1), None, None)
}

/// cbw

/// EAX ← sign-extend of AX.
/// RAX ← sign-extend of EAX(long mode only).
pub fn sign_extend(long_mode: bool) -> Vec<u8> {
    inst(false, long_mode, &[0x98], None, None, None)
}

/// EDX:EAX ← sign-extend of EAX
/// RDX:RAX ← sign-extend of RAX(long mode only).
pub fn sign_extend2(long_mode: bool) -> Vec<u8> {
    inst(false, long_mode, &[0x98], None, None, None)
}

/// ## cmp

pub fn cmp_first_reg_and_imm(long_mode: bool, imm: u32) -> Vec<u8> {
    inst(false, long_mode, &[0x3d], None, None, Some(Imm::from(imm)))
}

/// - cmp: Compare imm32 [with r/m32 | sign-extended to 64-bits with r/m64]
pub fn cmp_imm(long_mode: bool, op1: Op1, imm: u32) -> Vec<u8> {
    inst(
        false,
        long_mode,
        &[0x81, 7],
        Some(op1),
        None,
        Some(Imm(imm as u64, ImmByte::Bit32)),
    )
}

/// - cmp_imm8: Compare imm8 with r/m8
pub fn cmp_imm8(long_mode: bool, op1: Op1, imm: u8) -> Vec<u8> {
    inst(
        false,
        long_mode,
        &[0x83, 7],
        Some(op1),
        None,
        Some(Imm(imm as u64, ImmByte::Bit8)),
    )
}

/// - cmp: Compare r32 with r/m32(64)
pub fn cmp(long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    inst(false, long_mode, &[0x39, 7], Some(op1), Some(op2), None)
}

/// - cmp_rev: Compare r/m32(64) with r32
pub fn cmp_rev(long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    inst(false, long_mode, &[0x3b, 7], Some(op1), Some(op2), None)
}

/// - cmps: Compares quadword at address (R|E)SI with quadword at address (R|E)DI and sets the status flags accordingly.
pub fn cmps(long_mode: bool) -> Vec<u8> {
    inst(false, long_mode, &[0xa7], None, None, None)
}

/// - test_first_reg

pub fn test_first_reg_and_imm8(imm: u8) -> Vec<u8> {
    inst(
        false,
        false,
        &[0xa8],
        None,
        None,
        Some(Imm(imm as u64, ImmByte::Bit8)),
    )
}

pub fn test_first_reg(long_mode: bool, imm: Imm) -> Vec<u8> {
    inst(false, long_mode, &[0xa9], None, None, Some(imm))
}

pub fn test_imm8(long_mode: bool, op1: Op1, imm: u8) -> Vec<u8> {
    inst(
        false,
        long_mode,
        &[0xf6, 0],
        Some(op1),
        None,
        Some(Imm(imm as u64, ImmByte::Bit8)),
    )
}

/// - test_imm: Test imm32 [with r/m32 | sign-extended to 64-bits with r/m64]
pub fn test_imm(long_mode: bool, op1: Op1, op2: TargetReg, imm: u32) -> Vec<u8> {
    inst(
        false,
        long_mode,
        &[0xf7, 0],
        Some(op1),
        Some(op2),
        Some(Imm(imm as u64, ImmByte::Bit32)),
    )
}

// - test_u8
pub fn test_u8(long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    inst(false, long_mode, &[0x84], Some(op1), Some(op2), None)
}

/// - test
pub fn test(long_mode: bool, op1: Op1, op2: TargetReg) -> Vec<u8> {
    inst(false, long_mode, &[0x85], Some(op1), Some(op2), None)
}

/// - int1
#[inline]
pub fn int1() -> Vec<u8> {
    inst(false, false, &[0xf1], None, None, None)
}

/// - int3
#[inline]
pub fn int3() -> Vec<u8> {
    inst(false, false, &[0xcc], None, None, None)
}

/// - int
#[inline]
pub fn int(imm: u8) -> Vec<u8> {
    inst(false, false, &[0xcd], None, None, Some(Imm::from(imm)))
}

/// - into
#[inline]
pub fn into() -> Vec<u8> {
    inst(false, false, &[0xce], None, None, None)
}

/// - syscall
#[inline]
pub fn syscall() -> Vec<u8> {
    inst(false, false, &[0x0f, 0x05], None, None, None)
}

/// - sysenter
#[inline]
pub fn sysenter() -> Vec<u8> {
    inst(false, false, &[0x0f, 0x34], None, None, None)
}

/// - call

// pub fn call_relative_addr(label: String) -> JumpInst {
//     let opcodes = inst(
//         false,
//         false,
//         &[0xe9],
//         None,
//         None,
//         Some(Imm(0, ImmByte::Bit32)),
//     );
//     JumpInst::from(opcodes, ImmByte::Bit32, label)
// }

/// jit use it
pub fn call_addr_literal(addr: Imm) -> Vec<u8> {
    inst(false, false, &[0x9a], None, None, Some(addr))
}

pub fn call_reg(op1: Op1) -> Vec<u8> {
    inst(false, false, &[0x9a], Some(op1), None, None)
}

/// ## jmp

// pub fn jmp(label: String) -> JumpInst {
//     let opcodes = inst(
//         false,
//         false,
//         &[0xe9],
//         None,
//         None,
//         Some(Imm(0, ImmByte::Bit32)),
//     );
//     JumpInst::from(opcodes, ImmByte::Bit32, label)
// }

/// jit use it
pub fn jmp_addr_literal(addr: Imm) -> Vec<u8> {
    inst(false, false, &[0xea], None, None, Some(addr))
}

/// runtime_symbol only
// pub fn jmp_to_runtime_symbol(label: String) -> JumpInst {
//     let opcodes = inst(
//         false,
//         false,
//         &[0xea],
//         None,
//         None,
//         Some(Imm(0, ImmByte::Bit32)),
//     );
//     JumpInst::from(opcodes, ImmByte::Bit32, label)
// }

pub fn jmp_to_reg(reg: TargetReg) -> Vec<u8> {
    inst(false, false, &[0xff, 4], Some(Op1::Direct(reg)), None, None)
}

/// ## conditional jump

macro_rules! impl_cond_jump_inst {
    ($name:ident, $expr: expr) => {
        pub fn $name(addr: u64) -> Vec<u8> {
            inst(
                false,
                false,
                $expr,
                None,
                None,
                Some(Imm(addr, ImmByte::Bit64)),
            )
        }
    };
}

impl_cond_jump_inst!(ja, &[0x0f, 0x87]);
impl_cond_jump_inst!(jb, &[0x0f, 0x82]);
impl_cond_jump_inst!(jc, &[0x0f, 0x82]);
impl_cond_jump_inst!(je, &[0x0f, 0x84]);
impl_cond_jump_inst!(jg, &[0x0f, 0x8f]);
impl_cond_jump_inst!(jl, &[0x0f, 0x8c]);
impl_cond_jump_inst!(jo, &[0x0f, 0x80]);
impl_cond_jump_inst!(jp, &[0x0f, 0x8a]);
impl_cond_jump_inst!(js, &[0x0f, 0x88]);
impl_cond_jump_inst!(jz, &[0x0f, 0x84]);
impl_cond_jump_inst!(jae, &[0x0f, 0x83]);
impl_cond_jump_inst!(jbe, &[0x0f, 0x86]);
impl_cond_jump_inst!(jge, &[0x0f, 0x8d]);
impl_cond_jump_inst!(jle, &[0x0f, 0x8e]);
impl_cond_jump_inst!(jpe, &[0x0f, 0x8a]);
impl_cond_jump_inst!(jpo, &[0x0f, 0x8b]);
impl_cond_jump_inst!(jna, &[0x0f, 0x86]);
impl_cond_jump_inst!(jnb, &[0x0f, 0x83]);
impl_cond_jump_inst!(jnc, &[0x0f, 0x83]);
impl_cond_jump_inst!(jne, &[0x0f, 0x85]);
impl_cond_jump_inst!(jng, &[0x0f, 0x8e]);
impl_cond_jump_inst!(jnl, &[0x0f, 0x8d]);
impl_cond_jump_inst!(jno, &[0x0f, 0x81]);
impl_cond_jump_inst!(jnp, &[0x0f, 0x8b]);
impl_cond_jump_inst!(jns, &[0x0f, 0x89]);
impl_cond_jump_inst!(jnz, &[0x0f, 0x85]);
impl_cond_jump_inst!(jnae, &[0x0f, 0x82]);
impl_cond_jump_inst!(jnbe, &[0x0f, 0x87]);
impl_cond_jump_inst!(jnge, &[0x0f, 0x8c]);
impl_cond_jump_inst!(jnle, &[0x0f, 0x8f]);

/// ## logic inst

/// ### and

pub fn and_first_reg_imm32(long_mode: bool, imm: u32) -> Vec<u8> {
    inst(false, long_mode, &[0x25], None, None, Some(Imm::from(imm)))
}

pub fn and_reg_imm32(long_mode: bool, op1: Op1, imm: u32) -> Vec<u8> {
    inst(
        false,
        long_mode,
        &[0x81, 4],
        Some(op1),
        None,
        Some(Imm::from(imm)),
    )
}

pub fn and_reg_imm8(long_mode: bool, op1: Op1, imm: u8) -> Vec<u8> {
    inst(
        false,
        long_mode,
        &[0x83, 4],
        Some(op1),
        None,
        Some(Imm::from(imm)),
    )
}

pub fn and(long_mode: bool, op1: Op1, reg: TargetReg) -> Vec<u8> {
    inst(false, long_mode, &[0x21], Some(op1), Some(reg), None)
}

pub fn and_rev(long_mode: bool, op1: Op1, reg: TargetReg) -> Vec<u8> {
    inst(false, long_mode, &[0x23], Some(op1), Some(reg), None)
}

/// ### or

pub fn or_first_reg_imm32(long_mode: bool, imm: u32) -> Vec<u8> {
    inst(false, long_mode, &[0x0d], None, None, Some(Imm::from(imm)))
}

pub fn or_reg_imm32(long_mode: bool, op1: Op1, imm: u32) -> Vec<u8> {
    inst(
        false,
        long_mode,
        &[0x81, 1],
        Some(op1),
        None,
        Some(Imm::from(imm)),
    )
}

pub fn or_reg_imm8(long_mode: bool, op1: Op1, imm: u8) -> Vec<u8> {
    inst(
        false,
        long_mode,
        &[0x83, 1],
        Some(op1),
        None,
        Some(Imm::from(imm)),
    )
}

pub fn or(long_mode: bool, op1: Op1, reg: TargetReg) -> Vec<u8> {
    inst(false, long_mode, &[0x09], Some(op1), Some(reg), None)
}

pub fn or_rev(long_mode: bool, op1: Op1, reg: TargetReg) -> Vec<u8> {
    inst(false, long_mode, &[0x0b], Some(op1), Some(reg), None)
}

/// ## nop

#[inline]
pub fn nop() -> Vec<u8> {
    nop1()
}

pub fn nop1() -> Vec<u8> {
    let r = inst(false, false, &[0x90], None, None, None);
    debug_assert_eq!(dbg!(r.len()), 1);
    r
}

pub fn nop2() -> Vec<u8> {
    let r = inst(false, false, &[66, 0x90], None, None, None);
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
    let r = inst(
        false,
        false,
        &[66, 0x0f, 0x1f],
        Some(Op1::ScaleBase(
            TargetReg::from(0),
            TargetReg::from(0),
            ScaledIndex::Id,
            u8::MAX as usize,
        )),
        None,
        None,
    );
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
    let r = inst(
        false,
        false,
        &[66, 0x0f, 0x1f],
        Some(Op1::ScaleBase(
            TargetReg::from(0),
            TargetReg::from(0),
            ScaledIndex::Id,
            u32::MAX as usize,
        )),
        None,
        None,
    );
    debug_assert_eq!(dbg!(r.len()), 9);
    r
}

pub fn nop_multi_reg(op1: Op1) -> Vec<u8> {
    inst(false, false, &[0x0f, 0x1f], Some(op1), None, None)
}

/// ## ret

pub fn near_ret() -> Vec<u8> {
    inst(false, false, &[0xc3], None, None, None)
}

pub fn far_ret() -> Vec<u8> {
    inst(false, false, &[0xcb], None, None, None)
}

pub fn near_ret_imm16(imm: u16) -> Vec<u8> {
    inst(
        false,
        false,
        &[0xc2],
        None,
        None,
        Some(Imm(imm as u64, ImmByte::Bit16)),
    )
}

pub fn far_ret_imm16(imm: u16) -> Vec<u8> {
    inst(
        false,
        false,
        &[0xca],
        None,
        None,
        Some(Imm(imm as u64, ImmByte::Bit16)),
    )
}

/// ## SSE inst

/// - movss
/// movss xmm1, xmm2/m32
/// Merge scalar single-precision floating-point value from xmm2 to xmm1 register.
/// Load scalar single-precision floating-point value from m32 to xmm1 register.
pub fn movss(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    sse_inst(
        &[0xf3, 0x0f, 0x10],
        Some(op1),
        Some(TargetReg::from(op2 as u8)),
        None,
    )
}

/// - movss_rev
/// movss xmm2/m32, xmm1
pub fn movss_rev(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    sse_inst(
        &[0xf3, 0x0f, 0x11],
        Some(op1),
        Some(TargetReg::from(op2 as u8)),
        None,
    )
}

/// - movsd
/// movsd xmm1, xmm2/m32
/// Move scalar double-precision floating-point value from xmm2 to xmm1 register.
/// Load scalar double-precision floating-point value from m64 to xmm1 register.
pub fn movsd(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    sse_inst(
        &[0xf2, 0x0f, 0x10],
        Some(op1),
        Some(TargetReg::from(op2 as u8)),
        None,
    )
}

/// - movsd_rev
/// movsd xmm2/m32, xmm1
pub fn movsd_rev(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    sse_inst(
        &[0xf2, 0x0f, 0x11],
        Some(op1),
        Some(TargetReg::from(op2 as u8)),
        None,
    )
}

/// - addss
/// addss xmm1, xmm2/m32
/// Add scalar single-precision floating-point value from xmm2 to xmm1 register.
pub fn addss(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    sse_inst(
        &[0xf3, 0x0f, 0x58],
        Some(op1),
        Some(TargetReg::from(op2 as u8)),
        None,
    )
}

/// - addsd
/// addsd xmm1, xmm2/m64
/// Add scalar double-precision floating-point value from xmm2 to xmm1 register.
pub fn addsd(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    sse_inst(
        &[0xf2, 0x0f, 0x58],
        Some(op1),
        Some(TargetReg::from(op2 as u8)),
        None,
    )
}

/// - subss
/// subss xmm1, xmm2/m32
/// Subtract scalar single-precision floating-point value from xmm2 from xmm1 register.
pub fn subss(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    sse_inst(
        &[0xf3, 0x0f, 0x5c],
        Some(op1),
        Some(TargetReg::from(op2 as u8)),
        None,
    )
}

/// - subsd
/// subsd xmm1, xmm2/m64
/// Subtract scalar double-precision floating-point value from xmm2 from xmm1 register.
pub fn subsd(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    sse_inst(
        &[0xf2, 0x0f, 0x5c],
        Some(op1),
        Some(TargetReg::from(op2 as u8)),
        None,
    )
}

/// - mulss
/// mulss xmm1, xmm2/m32
/// Multiply scalar single-precision floating-point value from xmm2 to xmm1 register.
pub fn mulss(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    sse_inst(
        &[0xf3, 0x0f, 0x59],
        Some(op1),
        Some(TargetReg::from(op2 as u8)),
        None,
    )
}

/// - mulsd
/// mulsd xmm1, xmm2/m64
/// Multiply scalar double-precision floating-point value from xmm2 to xmm1 register.
pub fn mulsd(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    sse_inst(
        &[0xf2, 0x0f, 0x59],
        Some(op1),
        Some(TargetReg::from(op2 as u8)),
        None,
    )
}

/// - divss
/// divss xmm1, xmm2/m32
/// Divide scalar single-precision floating-point value from xmm2 by xmm1 register.
pub fn divss(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    sse_inst(
        &[0xf3, 0x0f, 0x5e],
        Some(op1),
        Some(TargetReg::from(op2 as u8)),
        None,
    )
}

/// - divsd
/// divsd xmm1, xmm2/m64
/// Divide scalar double-precision floating-point value from xmm2 by xmm1 register.
pub fn divsd(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    sse_inst(
        &[0xf2, 0x0f, 0x5e],
        Some(op1),
        Some(TargetReg::from(op2 as u8)),
        None,
    )
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
    sse_inst(
        &[0xf3, 0x0f, 0xC2],
        Some(op1),
        Some(TargetReg::from(op2 as u8)),
        Some(Imm(imm as u64, ImmByte::Bit8)),
    )
}

/// - cmpsd
/// cmpsd xmm1, xmm2/m64
/// cmpsd(xmm2/m64, xmm1)
/// Compare scalar double-precision floating-point value from xmm2 to xmm1 register.
pub fn cmpsd(op1: Op1, op2: RegisterXmm, imm: FcmpOp) -> Vec<u8> {
    sse_inst(
        &[0xf2, 0x0f, 0xC2],
        Some(op1),
        Some(TargetReg::from(op2 as u8)),
        Some(Imm(imm as u64, ImmByte::Bit8)),
    )
}

/// - sqrtss
/// sqrtss xmm1, xmm2/m32
/// sqrtss(xmm2/mem, xmm1)
/// Compute square root of scalar single-precision floating-point value in xmm2 and store result in xmm1.
pub fn sqrtss(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    sse_inst(
        &[0xf3, 0x0f, 0x51],
        Some(op1),
        Some(TargetReg::from(op2 as u8)),
        None,
    )
}

/// ## SIMD Inst
/// - movupd
/// movupd xmm1, xmm2/m128
/// movupd(xmm2/m128, xmm1)
/// Move unaligned packed double-precision floating-point values from xmm2/mem to xmm1.
pub fn movupd(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    sse_inst(
        &[0x66, 0x0F, 0x10],
        Some(op1),
        Some(TargetReg::from(op2 as u8)),
        None,
    )
}

/// movupd xmm2/m128, xmm1
/// movupd_rev(xmm2/m128, xmm1)
/// Move unaligned packed double-precision floating-point from xmm1 to xmm2/mem.
pub fn movupd_rev(op1: Op1, op2: RegisterXmm) -> Vec<u8> {
    sse_inst(
        &[0x66, 0x0F, 0x10],
        Some(op1),
        Some(TargetReg::from(op2 as u8)),
        None,
    )
}
