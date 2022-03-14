use lyuu_commons::isa::riscv::*;

use super::{
    inst_dump_buf::InstBuffer,
    rv32::rv32i::*,
    rv64::rv64i::*
};

impl InstBuffer {
    pub fn isa_transform(&self, i: RiscV) {
        match i {
            RiscV::Lui(rd, imm) => self.inst(lui(rd, imm)),
            RiscV::Auipc(rd, imm) => self.inst(auipc(rd, imm)),
            RiscV::Jal(rd, imm) => self.inst(jal(rd, imm)),
            RiscV::Jalr(rd, rs1, imm) => self.inst(jalr(rd, rs1, imm)),
            RiscV::Branch(brtype, rs1, rs2, imm) => self.inst(
                match brtype {
                    BrType::Eq => beq,
                    BrType::Ne => bne,
                    BrType::Lt => blt,
                    BrType::Ge => bge,
                    BrType::Ltu => bltu,
                    BrType::Geu => bgeu,
                }(rs1, rs2, imm)),
            RiscV::Load(ldtype, rd, rs1, imm) => self.inst(
                match ldtype {
                    LoadType::Byte => lb,
                    LoadType::Half => lh,
                    LoadType::Word => lw,
                    LoadType::Double => ld,
                    LoadType::ByteU => lbu,
                    LoadType::HalfU => lhu,
                    LoadType::WordU => lwu,
                }(rd, rs1, imm)),
            RiscV::Store(sttype, rs1, rs2, imm) => self.inst(
                match sttype {
                    StoreType::Byte => lb,
                    StoreType::Half => lh,
                    StoreType::Word => lw,
                    StoreType::Double => ld,
                }(rs1, rs2, imm)),
            RiscV::OpI(ioptype, rd, rs1, imm) => self.inst(
                match ioptype {
                    IOpType::Addi   => addi(rd, rs1, imm),
                    IOpType::Slti   => slti(rd, rs1, imm),
                    IOpType::Sltiu  => sltiu(rd, rs1, imm),
                    IOpType::Xori   => xori(rd, rs1, imm),
                    IOpType::Ori    => ori(rd, rs1, imm),
                    IOpType::Andi   => andi(rd, rs1, imm),
                    IOpType::Slli   => slli(rd, rs1, imm as u8),
                    IOpType::Srli   => srli(rd, rs1, imm as u8),
                    IOpType::Srai   => srai(rd, rs1, imm as u8),
                }),
            RiscV::Op(optype, rd, rs1, rs2) => self.inst(
                match optype {
                    OpType::Add     => add,
                    OpType::Sub     => sub,
                    OpType::Sll     => sll,
                    OpType::Slt     => slt,
                    OpType::Sltu    => sltu,
                    OpType::Xor     => xor,
                    OpType::Srl     => srl,
                    OpType::Sra     => sra,
                    OpType::Or      => or,
                    OpType::And     => and,
                }(rd, rs1, rs2)),
            RiscV::Fence(isfencei, pred, succ) => self.inst(
                if isfencei.0 { fencei() } else { fence(pred.0, succ.0) }),
            RiscV::EOp(EOpType::Call) => self.inst(ecall()),
            RiscV::EOp(EOpType::Break) => self.inst(ebreak()),
            RiscV::CsrOp(_, _, _, _) => todo!(),
            RiscV::CsrOpI(_, _, _, _) => todo!(),
        }
    }

}