mod page_manage;
mod insts;

use page_manage::*;
use insts::x86_64::Operator1;
use insts::x86_64::registers::*;
use insts::x86_64::inst_list::mov;

fn main() {
    // let page_size = PageSize::from_system();

    let src = [
        0x48, 0x89, 0xf8,                   // mov %rdi, %rax
        0x48, 0x83, 0xc0, 0x04,             // add $4, %rax
        0xc3                                // ret
        ];

    let src = [0x48, 0x89, 0xf8];
    let r = mov(
        false,
        true,
        Operator1::Direct(TargetReg::from(Register64::RAX as u8)),
        TargetReg::from(Register64::RDI as u8));

    println!("src:{:?}, gen:{:?}", src, r);
    // let r = PageHandle::from(page_size, &src);

    // let code: extern "C" fn(i32) -> i32 = unsafe { std::mem::transmute(r.get_ptr()) };

    // dbg!((code)(1));
}
