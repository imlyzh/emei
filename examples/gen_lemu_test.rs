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
    println!("add x0: {:?}", inst_list);

    let inst_list = [
        add(Reg::new(X1), Reg::new(X1), Reg::new(X1)),
    ];
    println!("add self: {:?}", inst_list);

    let inst_list = [
        sub(Reg::new(X1), Reg::new(X1), Reg::new(X0)),
    ];
    println!("sub x0: {:?}", inst_list);

    let inst_list = [
        sub(Reg::new(X1), Reg::new(X1), Reg::new(X1)),
    ];
    println!("sub self: {:?}", inst_list);

    let inst_list = [
        jal(Reg::new(X0), -24),
    ];
    println!("jal -6: {:?}", inst_list);
}
