mod insts;
mod page_manage;

use insts::x86_64::inst_list::*;
use insts::x86_64::registers::*;
use insts::x86_64::Op1;
use page_manage::*;

fn _history() {
    /*
    let src = [
        0x48, 0x89, 0xf8,                   // mov %rdi, %rax
        0x48, 0x83, 0xc0, 0x04,             // add $4, %rax
        0xc3                                // ret
        ];

    let r = [0x48, 0x89, 0xf8];
    let r1 = [0x48, 0x83, 0xc0, 0x04];
    let r2 = [0xc3];
    println!("r:{:?}, r1:{:?}, r2:{:?}", r, r1, r2);
    */
    // println!("r:{:?}, r1:{:?}, r2:{:?}", r, r1, r2);
}

fn main() {
    let page_size = PageSize::from_system();

    let r = mov(
        false,
        true,
        Op1::Direct(TargetReg::from(Register64::R8 as u8)),
        TargetReg::from(Register64::Rdi as u8),
    );
    let r1 = mov(
        false,
        true,
        Op1::Direct(TargetReg::from(Register64::Rax as u8)),
        TargetReg::from(Register64::R8 as u8),
    );
    let r2 = add_imm8(
        false,
        true,
        Op1::Direct(TargetReg::from(Register64::Rax as u8)),
        4,
    );
    let r3 = near_ret();
    let src = [r.iter(), r1.iter(), r2.iter(), r3.iter()]
        .into_iter()
        .flatten()
        .cloned()
        .collect::<Vec<u8>>();

    let r = PageHandle::from(page_size, &src);

    let code: extern "C" fn(i32) -> i32 = unsafe { std::mem::transmute(r.get_ptr()) };

    dbg!((code)(1));
}
