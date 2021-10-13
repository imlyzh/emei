pub mod page_manage;
pub mod insts;

#[test]
#[cfg(target_arch = "x86_64")]
fn nop_length_test() {
    use insts::x86_64::inst_list::*;
    nop1();
    nop3();
    nop4();
    nop7();
}
