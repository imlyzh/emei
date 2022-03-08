use libemei::insts::riscv::rv32::rv32i::*;
use libemei::insts::riscv::registers::*;

fn main() {
    let inst_list = [
        lui(Reg::new(X1), 114514),
        addi(Reg::new(X1), Reg::new(X1), 1919),
    ];
    println!("{:?}", inst_list);
}
