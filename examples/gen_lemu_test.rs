use libemei::insts::riscv::rv32::rv32i::*;
use libemei::insts::riscv::registers::*;

fn main() {
    let inst_list = [
        lui(Reg::new(X1), 114514),
        addi(Reg::new(X1), Reg::new(X1), 1919),
    ];
    println!("lui addi: {:?}", inst_list);

    let inst_list = [
        add(Reg::new(X1), Reg::new(X1), Reg::new(X0)),
    ];
    println!("add self: {:?}", inst_list);
}
