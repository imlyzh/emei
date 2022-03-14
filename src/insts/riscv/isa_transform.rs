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
                    IOpType::Addi => addi,
                    IOpType::Slti => slti,
                    IOpType::Sltiu => sltiu,
                    IOpType::Xori => xori,
                    IOpType::Ori => ori,
                    IOpType::Andi => andi,
                    IOpType::Slli => slli(rd, rs1, imm ),
                    IOpType::Srli => srli,
                    IOpType::Srai => srai,
                }(rd, rs1, imm)),
            RiscV::Op(optype, rd, rs1, rs2) => todo!(),
            RiscV::Fence(isfencei, pred, succ) => todo!(),
            RiscV::EOp(EOpType::Call) => todo!(),
            RiscV::EOp(EOpType::Break) => todo!(),
            RiscV::CsrOp(_, _, _, _) => todo!(),
            RiscV::CsrOpI(_, _, _, _) => todo!(),
        }
    }

}