use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{AddAssign, DerefMut},
};

use crate::insts::{LinkError, riscv::untils::branch};

use super::{Inst, CInst, registers::Reg};


#[derive(Debug, Clone)]
pub struct JumpInst {
    pub opcodes: u8,
    pub funct: u8,
    pub rs1: Reg,
    pub rs2: Reg,
    pub label: String,
}

#[derive(Debug, Clone)]
pub enum InstUnit {
    CInst(CInst),
    Inst(Inst),
    JumpInst(JumpInst, u32),
}

impl InstUnit {
    pub fn size(&self) -> usize {
        match self {
            InstUnit::CInst(_) => 2,
            InstUnit::Inst(_) => 4,
            InstUnit::JumpInst(_jump_inst, _) => 4,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InstBuffer {
    pub buf: RefCell<Vec<InstUnit>>,
    pub label_buf: RefCell<HashMap<String, u32>>,
    pub offset: RefCell<u32>,
}

impl InstBuffer {
    pub fn len(&self) -> usize {
        self.buf.borrow().len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn cinst(&self, i: CInst) {
        let boxed_inst = InstUnit::CInst(i);
        self.offset
            .borrow_mut()
            .deref_mut()
            .add_assign(boxed_inst.size() as u32);
        self.buf.borrow_mut().push(boxed_inst);
    }

    pub fn inst(&self, i: Inst) {
        let boxed_inst = InstUnit::Inst(i);
        self.offset
            .borrow_mut()
            .deref_mut()
            .add_assign(boxed_inst.size() as u32);
        self.buf.borrow_mut().push(boxed_inst);
    }

    pub fn jump(&self, i: JumpInst) {
        let boxed_inst = InstUnit::JumpInst(i, *self.offset.borrow());
        self.offset
            .borrow_mut()
            .deref_mut()
            .add_assign(boxed_inst.size() as u32);
        self.buf.borrow_mut().push(boxed_inst);
    }

    pub fn label(&self, label: String) {
        self.label_buf
            .borrow_mut()
            .insert(label, *self.offset.borrow());
    }

    pub fn runtime_symbol(&self, label: String, imm: u32) {
        self.label_buf
            .borrow_mut()
            .insert(label, imm);
    }

    pub fn dump(&self, buf: &mut Vec<u8>) -> Result<(), LinkError> {
        for i in self.buf.borrow().iter() {
            match i {
                InstUnit::CInst(c) => buf.extend(&c.to_le_bytes()),
                InstUnit::Inst(i) => buf.extend(&i.to_le_bytes()),
                InstUnit::JumpInst(j, offset) => {
                    let label_offset = self.label_buf.borrow()
                        .get(&j.label)
                        .cloned()
                        .ok_or_else(|| LinkError(j.label.clone()))?;
                    let imm = label_offset as i64 - *offset as i64;
                    let i = branch(j.funct, j.rs1, j.rs2, imm as i16);
                    buf.extend(&i.to_le_bytes())
                },
            }
        }
        Ok(())
    }
}
