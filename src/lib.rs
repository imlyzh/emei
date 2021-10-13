pub mod page_manage;
pub mod insts;

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

#[test]
#[cfg(target_arch = "x86_64")]
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
