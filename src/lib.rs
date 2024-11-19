#![doc=include_str!("../README.md")]

pub mod insts;
pub mod page_manage;

/*
macro_rules! debug_code_no_ret {
    ($code:expr) => {
        let page_size = PageSize::from_system();
        let mut r = $code;
        r.append(&mut near_ret());
        let r = PageHandle::from(page_size, &r);
        let code: extern "C" fn() = unsafe { std::mem::transmute(r.get_ptr()) };
        dbg!((code)());
    };
}
 */

#[test]
#[cfg(target_arch = "x86_64")]
fn x86_64_test() {
    use insts::x86_64::inst_list::*;
    use insts::x86_64::registers::*;
    use insts::x86_64::Op1;

    use crate::page_manage::PageHandle;
    use crate::page_manage::PageSize;

    let r = lea(
        false,
        true,
        Op1::ScaleBase(
            TargetReg::from(Register64::Rcx as u8),
            TargetReg::from(Register64::Rdx as u8),
            ScaledIndex::Id,
            0,
        ),
        TargetReg::from(Register64::Rax as u8),
    );
    /*
    let r = mov(
        false,
        true,
        Op1::Direct(TargetReg::from(Register64::Rcx as u8)),
        TargetReg::from(Register64::Rax as u8),
    );
    */
    // /*
    let r1 = mov(
        false,
        true,
        Op1::Direct(TargetReg::from(Register64::Rax as u8)),
        TargetReg::from(Register64::R8 as u8),
    );
    //  */
    let r2 = add_imm8(
        false,
        true,
        Op1::Direct(TargetReg::from(Register64::Rax as u8)),
        4,
    );
    let r3 = near_ret();
    let src = [
        r.iter(),
        // r1.iter(),
        // r2.iter(),
        r3.iter(),
    ]
    .into_iter()
    .flatten()
    .cloned()
    .collect::<Vec<u8>>();

    let r = PageHandle::from(PageSize::from(src.capacity()), &src);

    let code: extern "C" fn(u64, u64) -> u64 = unsafe { std::mem::transmute(r.get_ptr()) };

    assert_eq!((code)(1, 3), 4);
}

#[test]
#[cfg(target_arch = "riscv")]
fn rv_add_test() {
    use insts::riscv::registers::*;
    use insts::riscv::rv32::rv32i::*;

    use crate::page_manage::PageHandle;
    use crate::page_manage::PageSize;
}

/*
#[test]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
fn nop_length_test() {
    use crate::insts::x86_64::inst_list::*;
    use crate::page_manage::*;
    debug_code_no_ret!(nop1());
    debug_code_no_ret!(nop2());
    debug_code_no_ret!(nop3());
    debug_code_no_ret!(nop4());
    debug_code_no_ret!(nop5());
    debug_code_no_ret!(nop6());
    debug_code_no_ret!(nop7());
    debug_code_no_ret!(nop8());
    debug_code_no_ret!(nop9());
}
 */
