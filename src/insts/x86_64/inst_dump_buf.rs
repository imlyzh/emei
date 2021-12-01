use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{AddAssign, DerefMut},
};

use crate::insts::ImmByte;

/*
#[derive(Debug, Clone, Default)]
pub struct CallInst(pub JumpInst);

*/

#[derive(Debug, Clone, Default)]
pub struct JumpInst {
    pub opcodes: Vec<u8>,
    pub label: String,
    pub modify_range: (usize, usize),
}

impl JumpInst {
    pub fn from(opcodes: Vec<u8>, imm_byte: ImmByte, label: String) -> Self {
        let right = opcodes.len();
        let left = right
            - match imm_byte {
                ImmByte::Bit8 => 1,
                ImmByte::Bit16 => 2,
                ImmByte::Bit32 => 4,
                ImmByte::Bit64 => 8,
            };
        JumpInst {
            opcodes,
            label,
            modify_range: (left, right),
        }
    }
    pub fn len(&self) -> usize {
        self.opcodes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Clone)]
pub struct LinkError(pub String);

#[derive(Debug, Clone)]
pub enum InstUnit {
    Inst(Vec<u8>),
    JumpInst(JumpInst),
    // CallInst(CallInst),
}

impl InstUnit {
    pub fn len(&self) -> usize {
        match self {
            InstUnit::Inst(inst) => inst.len(),
            InstUnit::JumpInst(jump_inst) => jump_inst.len(),
            // InstUnit::CallInst(call_inst) => call_inst.0.len(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
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

    pub fn inst(&self, i: Vec<u8>) {
        self.offset
            .borrow_mut()
            .deref_mut()
            .add_assign(i.len() as u32);
        self.buf.borrow_mut().push(InstUnit::Inst(i));
    }

    pub fn jump(&self, i: JumpInst) {
        self.offset
            .borrow_mut()
            .deref_mut()
            .add_assign(i.len() as u32);
        self.buf.borrow_mut().push(InstUnit::JumpInst(i));
    }

    pub fn label(&self, label: String) {
        self.label_buf
            .borrow_mut()
            .insert(label, *self.offset.borrow());
    }

    pub fn dump(&self, base_addr: u32, buf: &mut Vec<u8>) -> Result<(), LinkError> {
        for inst in self.buf.borrow().iter() {
            match inst {
                InstUnit::Inst(i) => {
                    buf.extend(i.iter());
                }
                InstUnit::JumpInst(j) => {
                    let obj = self
                        .label_buf
                        .borrow()
                        .get(&j.label)
                        .cloned()
                        .ok_or_else(|| LinkError(j.label.clone()))?;
                    let obj = base_addr + obj;
                    buf.extend(j.opcodes[..j.modify_range.0].iter());
                    buf.extend(obj.to_ne_bytes());
                    buf.extend(j.opcodes[j.modify_range.1..].iter());
                } /*
                  InstUnit::CallInst(j) => {
                      let j = &j.0;
                      let obj = fun_table.get(&j.label).ok_or(LinkError::FunctionNotFound(j.label.clone()))?;
                      buf.extend(j.opcodes[..j.modify_range.0].iter());
                      buf.extend(obj.to_ne_bytes());
                      buf.extend(j.opcodes[j.modify_range.1..].iter());
                  }
                   */
            }
        }
        Ok(())
    }
}
