mod page_manage;
mod insts;

use page_manage::*;
use insts::x86_64::Operator1;
use insts::x86_64::registers::*;
use insts::x86_64::inst_list::*;

fn main() {
    // let page_size = PageSize::from_system();

    let src = [
        0x48, 0x89, 0xf8,                   // mov %rdi, %rax
        0x48, 0x83, 0xc0, 0x04,             // add $4, %rax
        0xc3                                // ret
        ];

    let src = [0x48, 0x89, 0xf8];
    let src1 = [0x48, 0x83, 0xc0, 0x04];
    let r = mov(
        false,
        true,
        Operator1::Direct(TargetReg::from(Register64::RAX as u8)),
        TargetReg::from(Register64::RDI as u8));
    let r1 = add_imm8(
            false,
            true,
            Operator1::Direct(TargetReg::from(Register64::RAX as u8)),
            4);

    println!("src:{:?}, gen:{:?}", src, r);
    println!("src1:{:?}, gen1:{:?}", src1, r1);
    // let r = PageHandle::from(page_size, &src);

    // let code: extern "C" fn(i32) -> i32 = unsafe { std::mem::transmute(r.get_ptr()) };

    // dbg!((code)(1));
}
