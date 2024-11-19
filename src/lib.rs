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
    use std::fs::File;
    use std::io::Write;

    use insts::x86_64::inst_list::*;
    use insts::x86_64::registers::*;
    use insts::x86_64::Op1;

    use crate::page_manage::PageHandle;
    use crate::page_manage::PageSize;

    // let r = lea(
    //     false,
    //     true,
    //     Op1::ScaleBase(
    //         TargetReg::from(Register64::Rcx as u8),
    //         TargetReg::from(Register64::Rdx as u8),
    //         ScaledIndex::Id,
    //         0,
    //     ),
    //     TargetReg::from(Register64::Rax as u8),
    // );
    /*
    let r = mov(
        false,
        true,
        Op1::Direct(TargetReg::from(Register64::Rcx as u8)),
        TargetReg::from(Register64::Rax as u8),
    );
    // */
    /*
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
        Op1::Direct(TargetReg::from(Register64::Rax)),
        4,
    );
    let r3 = near_ret();
    let src = [
        // r.iter(),
        // r1.iter(),
        r2.iter(),
        r3.iter(),
    ]
    .into_iter()
    .flatten()
    .cloned()
    .collect::<Vec<u8>>();

    let mut f = File::options()
        .append(true)
        .create(true)
        .open("./outbytes.bin")
        .unwrap();
    f.write(&src).unwrap();

    let r = PageHandle::from(PageSize::from(src.capacity()), &src);

    let code: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(r.get_ptr()) };

    assert_eq!((code)(1), 5);
}
