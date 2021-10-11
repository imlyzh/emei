mod page_manage;

use page_manage::*;

fn main() {
    let page_size = PageSize::from_system();

    let src = [
        0x48, 0x89, 0xf8,                   // mov %rdi, %rax
        0x48, 0x83, 0xc0, 0x04,             // add $4, %rax
        0xc3                                // ret
        ];

    let r = PageHandle::from(page_size, &src);

    let code: extern "C" fn(i32) -> i32 = unsafe { std::mem::transmute(r.get_ptr()) };

    dbg!((code)(1));
}
