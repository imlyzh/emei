use std::{cell::RefCell, collections::HashMap, ops::{AddAssign, DerefMut}};

use crate::insts::ImmByte;

#[derive(Debug, Clone, Default)]
pub struct JumpInst {
    pub opcodes: Vec<u8>,
    pub label: String,
    pub modify_range: (usize, usize),
}

impl JumpInst {
    pub fn from(opcodes: Vec<u8>, imm_byte: ImmByte, label: String) -> Self {
        let right = opcodes.len();
        let left = right - match imm_byte {
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
}

#[derive(Debug, Clone)]
pub enum InstUnit {
    Inst(Vec<u8>),
    JumpInst(JumpInst),
}

impl InstUnit {
    pub fn len(&self) -> usize {
        match self {
            InstUnit::Inst(inst) => inst.len(),
            InstUnit::JumpInst(jump_inst) => jump_inst.len(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InstBuffer {
    pub buf: RefCell<Vec<InstUnit>>,
    pub label_buf: RefCell<HashMap<String, usize>>,
    pub offset: RefCell<usize>
}

impl InstBuffer {
    pub fn inst(&self, i: Vec<u8>) {
        self.offset.borrow_mut().deref_mut().add_assign(i.len());
        self.buf.borrow_mut().push(InstUnit::Inst(i));
    }
    pub fn jump(&self, i: JumpInst) {
        self.offset.borrow_mut().deref_mut().add_assign(i.len());
        self.buf.borrow_mut().push(InstUnit::JumpInst(i));
    }
    pub fn label(&self, label: String) {
        self.label_buf.borrow_mut().insert(label, self.offset.borrow().clone());
    }

    pub fn dump(self) -> Vec<u8> {
        let mut buf = vec![];
        for inst in self.buf.borrow().iter() {
            match inst {
                InstUnit::Inst(i) => {
                    buf.extend(i.iter());
                },
                InstUnit::JumpInst(j) => {
                    let mut opcodes: Vec<u8> = vec![];
                    opcodes.extend(j.opcodes[..j.modify_range.0].iter());
                    buf.extend(self.label_buf.borrow().get(&j.label).unwrap().to_ne_bytes());
                    opcodes.extend(j.opcodes[j.modify_range.1..].iter());
                    buf.extend(opcodes.iter());
                }
            }
        }
        buf
    }
}
