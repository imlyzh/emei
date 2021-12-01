use super::registers::Reg;
use super::*;

pub fn lui(rd: Reg, imm: u32) -> Inst {
    u(0b0110111, rd, imm)
}

pub fn auipc(rd: Reg, imm: u32) -> Inst {
    u(0b0010111, rd, imm)
}

pub fn jal(rd: Reg, imm: i32) -> Inst {
    let imm12_19 = ((imm >> 12) & 0b11111111) as u8;
    let imm11 = ((imm >> 11) & 0b1) as u8;
    let imm1_10 = ((imm >> 1) & 0b1111111111) as u8;
    let imm20 = ((imm >> 31) & 0b1) as u8;
    j(0b1101111, rd, imm12_19, imm11, imm1_10, imm20)
}

pub fn jalr(rd: Reg, rs1: Reg, imm: u16) -> Inst {
    i(0b1100111, rd, 000, rs1, imm)
}

pub fn branch(funct: u8, rs1: Reg, rs2: Reg, imm: i16) -> Inst {
    let imm11 = ((imm >> 11) & 0b1) as u8;
    let imm1_4 = ((imm >> 1) & 0b1111) as u8;
    let imm5_10 = ((imm >> 5) & 0b111111) as u8;
    let imm12 = ((imm >> 15) & 0b1) as u8;
    b(0b1100011, imm11, imm1_4, funct, rs1, rs2, imm5_10, imm12)
}

macro_rules! br_impl {
    ($name:ident, $funct:expr) => {
        pub fn $name(rs1: Reg, rs2: Reg, imm: i16) -> Inst {
            branch($funct, rs1, rs2, imm)
        }
    };
}

br_impl!(beq, 0b000);
br_impl!(bne, 0b001);
br_impl!(blt, 0b100);
br_impl!(bge, 0b101);
br_impl!(bltu, 0b110);
br_impl!(bgeu, 0b111);


pub fn load_data(funct: u8, rd: Reg, rs1: Reg, imm: u16) -> Inst {
    i(0b0000011, rd, funct, rs1, imm)
}

macro_rules! ld_impl {
    ($name:ident, $funct:expr) => {
        pub fn $name(rd: Reg, rs1: Reg, imm: u16) -> Inst {
            load_data($funct, rd, rs1, imm)
        }
    };
}

ld_impl!(lb, 0b000);
ld_impl!(lh, 0b001);
ld_impl!(lw, 0b010);
ld_impl!(lbu, 0b100);
ld_impl!(lhu, 0b101);

pub fn mathi(funct: u8, rd: Reg, rs1: Reg, imm: u16) -> Inst {
    i(0b0010011, rd, funct, rs1, imm)
}

macro_rules! mathi_impl {
    ($name:ident, $funct:expr) => {
        pub fn $name(rd: Reg, rs1: Reg, imm: u16) -> Inst {
            mathi($funct, rd, rs1, imm)
        }
    };
}

mathi_impl!(addi, 0b000);
mathi_impl!(slti, 0b010);
mathi_impl!(sltiu, 0b011);
mathi_impl!(xori, 0b100);
mathi_impl!(ori, 0b110);
mathi_impl!(andi, 0b111);

pub fn slli(rd: Reg, rs1: Reg, shamt: u16) -> Inst {
    r(0b0010011, rd, 0b001, rs1, shamt as u8, 0b0)
}

pub fn srli(rd: Reg, rs1: Reg, shamt: u16) -> Inst {
    r(0b0010011, rd, 0b101, rs1, shamt as u8, 0b0)
}

pub fn srai(rd: Reg, rs1: Reg, shamt: u16) -> Inst {
    r(0b0010011, rd, 0b101, rs1, shamt as u8, 0b0100000)
}

pub fn math(funct3: u8, funct7: u8, rd: Reg, rs1: Reg, rs2: Reg) -> Inst {
    r(0b0110011, rd, funct3, rs1, rs2, funct7)
}

macro_rules! math_impl {
    ($name:ident, $funct3:expr, $funct7:expr) => {
        pub fn $name(rd: Reg, rs1: Reg, rs2: Reg) -> Inst {
            math($funct3, $funct7, rd, rs1, rs2)
        }
    };
}

math_impl!(add, 0b000, 0b0000000);
math_impl!(sub, 0b000, 0b0100000);
math_impl!(sll, 0b001, 0b0000000);
math_impl!(slt, 0b010, 0b0000000);
math_impl!(sltu, 0b011, 0b0000000);
math_impl!(xor, 0b100, 0b0000000);
math_impl!(srl, 0b101, 0b0000000);
math_impl!(sra, 0b101, 0b0100000);
math_impl!(or, 0b110, 0b0000000);
math_impl!(and, 0b111, 0b0000000);

pub fn fence(pred: u8, succ: u8) -> Inst {
    let succ = succ & 0b11111;
    r(0b0001111, 0b00000, 0b000, 0b00000, pred, succ)
}

pub fn fence_i() -> Inst {
    i(0b0001111, 0b00000, 0b001, 0b00000, 0b0)
}

pub fn ecall() -> Inst {
    i(0b1110011, 0b00000, 0b000, 0b00000, 0b0)
}

pub fn ebreak() -> Inst {
    i(0b1110011, 0b00000, 0b000, 0b00000, 0b1)
}

pub fn csrrw(rd: Reg, rs1: Reg, csr: u16) -> Inst {
    i(0b1110011, rd, 0b001, rs1, csr)
}

pub fn csrrs(rd: Reg, rs1: Reg, csr: u16) -> Inst {
    i(0b1110011, rd, 0b010, rs1, csr)
}

pub fn csrrc(rd: Reg, rs1: Reg, csr: u16) -> Inst {
    i(0b1110011, rd, 0b011, rs1, csr)
}

pub fn csrrwi(rd: Reg, zimm: u8, csr: u16) -> Inst {
    i(0b1110011, rd, 0b101, zimm, csr)
}

pub fn cssrrsi(rd: Reg, zimm: u8, csr: u16) -> Inst {
    i(0b1110011, rd, 0b110, zimm, csr)
}

pub fn csrrci(rd: Reg, zimm: u8, csr: u16) -> Inst {
    i(0b1110011, rd, 0b111, zimm, csr)
}

