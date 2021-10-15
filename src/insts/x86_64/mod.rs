pub mod inst_list;
pub mod registers;

use std::panic;

use registers::{modrm, AddrMode, ScaledIndex, TargetReg, APPEND_SIB};

use self::registers::sib;

use super::ImmByte;

const PREFIX_LOCK: u8 = 0xF0;

// W 3 0 = Operand size determined by CS.D
//     1 = 64 Bit Operand Size
// R 2 Extension of the ModR/M reg field
// X 1 Extension of the SIB index field
// B 0 Extension of the ModR/M r/m field, SIB base field, or Opcode reg field

const REX_W: u8 = 0b01001000; // 48
const REX_R: u8 = 0b01000100; // 44
const REX_X: u8 = 0b01000010; // 42
const REX_B: u8 = 0b01000001; // 41


///////////////////////////////////////////////////////

type ModRM = u8;
type Sib = u8;

#[derive(Debug, Clone, Copy)]
pub enum Op1 {
    Direct(TargetReg),
    DeRef(TargetReg, usize),
    ScaleBase(TargetReg, TargetReg, ScaledIndex, usize),
}

fn usize_boxed_length(u: usize) -> AddrMode {
    if u == 0 {
        registers::AddrMode::RegRef
    } else if u8::MAX as usize >= u {
        registers::AddrMode::Disp8
    } else if u32::MAX as usize >= u {
        registers::AddrMode::Disp32
    } else {
        panic!("usize is too big to be boxed")
    }
}

impl Op1 {
    pub fn to_modrm_sib_disp(self, src_reg: TargetReg) -> (ModRM, Option<Sib>, Vec<u8>) {
        match self {
            Op1::Direct(reg) => (modrm(AddrMode::Direct, reg, src_reg), None, vec![]),
            Op1::DeRef(reg, disp) => {
                let addr_mode = usize_boxed_length(disp);
                (
                    modrm(addr_mode, reg, src_reg),
                    None,
                    addr_mode.encode_disp(disp),
                )
            }
            Op1::ScaleBase(base, index, scale, disp) => {
                let addr_mode = usize_boxed_length(disp);
                (
                    modrm(addr_mode, *APPEND_SIB, src_reg),
                    Some(sib(base, scale, index)),
                    addr_mode.encode_disp(disp),
                )
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Op2 {
    Imm(u64, ImmByte),
    Reg(TargetReg),
}

pub struct Inst {
    pub atomic: bool,
    pub long_mode: bool,
    pub opcode: Vec<u8>, // 0~2bytes
    pub op1: Option<Op1>,
    pub op2: Option<Op2>,
}

impl Inst {
    pub fn new(atomic: bool, long_mode: bool, opcode: &[u8]) -> Self {
        Inst {
            atomic,
            long_mode,
            opcode: opcode.to_vec(),
            op1: None,
            op2: None,
        }
    }
}

impl Inst {
    #[inline]
    fn into_raw(self) -> RawInst {
        let prefixes = if self.atomic {
            vec![PREFIX_LOCK]
        } else {
            vec![]
        };
        let opcode = if self.long_mode {
            let mut r = vec![REX_W];
            r.extend(self.opcode);
            r
        } else {
            self.opcode
        };
        let (mod_rm, sib, disp, imm) = match (self.op1, self.op2) {
            (None, None) => (None, None, vec![], vec![]),
            (None, Some(Op2::Imm(value, imm_byte))) => {
                (None, None, vec![], imm_byte.encode(value))
            }
            (None, Some(Op2::Reg(op2))) => (None, None, vec![], vec![op2 as u8]),
            (Some(op1), None) => {
                let (mod_rm, sib, disp) = op1.to_modrm_sib_disp(TargetReg::from(0));
                (Some(mod_rm), sib, disp, vec![])
            }
            (Some(op1), Some(Op2::Reg(op2))) => {
                let (mod_rm, sib, disp) = op1.to_modrm_sib_disp(op2);
                (Some(mod_rm), sib, disp, vec![])
            }
            (Some(op1), Some(Op2::Imm(value, imm_byte))) => {
                let (mod_rm, sib, disp) = op1.to_modrm_sib_disp(TargetReg::from(0));
                (Some(mod_rm), sib, disp, imm_byte.encode(value))
            }
        };
        RawInst {
            prefixes,
            opcode,
            modrm: mod_rm,
            sib: sib,
            disp,
            imm,
        }
    }
}

pub struct RawInst {
    pub prefixes: Vec<u8>, // 0~4bytes
    pub opcode: Vec<u8>,   // 0~3bytes
    pub modrm: Option<ModRM>,
    pub sib: Option<Sib>,
    pub disp: Vec<u8>, // 1/2/4bytes
    pub imm: Vec<u8>,  // 1/2/4bytes
}

impl RawInst {
    /*
    #[inline]
    pub fn check(&self) -> bool {
        const RANGE: [usize; 3] = [1, 2, 4];
        self.prefixes.len() > 4 &&
        self.opcode.len() > 3 &&
        RANGE.contains(&self.disp.len()) &&
        RANGE.contains(&self.imm.len())
    }
    // */

    #[inline]
    pub fn encode(self) -> Vec<u8> {
        let mut buf = vec![];
        buf.extend(self.prefixes);
        buf.extend(self.opcode);
        if let Some(x) = self.modrm {
            buf.push(x);
        }
        if let Some(x) = self.sib {
            buf.push(x);
        }
        buf.extend(self.disp);
        buf.extend(self.imm);
        buf
    }
}
