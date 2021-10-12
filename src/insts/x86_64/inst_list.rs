use crate::insts::x86_64::RawInst;

use super::{Inst, Operator1, Operator2, registers::Register32};


pub fn mov(
    is_atomic: bool,
    is_long_mode: bool,
    op1: Operator1,
    op2: Register32) -> Vec<u8> {
    Inst {
        is_atomic,
        is_long_mode,
        opcode: vec![0x89],
        op1: Some(op1),
        op2: Some(Operator2::Register(op2)),
    }.into_raw().encode()
}
