use libemei::insts::riscv::registers::*;
use libemei::insts::riscv::rv32::rv32i::*;

fn main() {
    let i = jalr(Reg::new(X0), Reg::new(X1), 0);
    dbg!(i.to_be_bytes());
}
