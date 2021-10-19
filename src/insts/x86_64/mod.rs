pub mod inst_list;
pub mod registers;

use std::panic;

use registers::{modrm, AddrMode, ScaledIndex, TargetReg, APPEND_SIB, Register32};

use self::registers::{Register64, sib};

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
    ScaleBase(TargetReg, TargetReg, ScaledIndex, usize), // base index scaleindex disp
}

#[cfg(target_arch = "x86")]
impl Op1 {
    fn rex_value(&self) -> u8 {
        0
    }
}

#[cfg(target_arch = "x86_64")]
impl Op1 {
    fn rex_value(&self) -> u8 {
        match self {
            Op1::Direct(r) => if r.is_extend() {REX_B} else {0},
            Op1::DeRef(r, _) => if r.is_extend() {REX_B} else {0},
            Op1::ScaleBase(baser, indexr, _, _) => {
                let a = if baser.is_extend() {REX_B} else {0};
                let b = if indexr.is_extend() {REX_X} else {0};
                a | b
            },
        }
    }
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
            Op1::Direct(reg) => (modrm(AddrMode::Direct, reg.get_reg(), src_reg.get_reg()), None, vec![]),
            Op1::DeRef(reg, disp) => {
                let addr_mode = usize_boxed_length(disp);
                (
                    modrm(addr_mode, reg.get_reg(), src_reg.get_reg()),
                    None,
                    addr_mode.encode_disp(disp),
                )
            }
            Op1::ScaleBase(base, index, scale, disp) => {
                let addr_mode = usize_boxed_length(disp);
                (
                    modrm(addr_mode, *APPEND_SIB, src_reg.get_reg()),
                    Some(sib(base, scale, index)),
                    addr_mode.encode_disp(disp),
                )
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Imm(u64, ImmByte);

impl From<Register64> for Imm {
    fn from(i: Register64) -> Self {
        Self(i.reg_value() as u64, ImmByte::Bit8)
    }
}

impl From<Register32> for Imm {
    fn from(i: Register32) -> Self {
        Self(i as u64, ImmByte::Bit8)
    }
}

impl From<u8> for Imm {
    fn from(i: u8) -> Self {
        Self(i as u64, ImmByte::Bit8)
    }
}

impl From<u16> for Imm {
    fn from(i: u16) -> Self {
        Self(i as u64, ImmByte::Bit16)
    }
}

impl From<u32> for Imm {
    fn from(i: u32) -> Self {
        Self(i as u64, ImmByte::Bit32)
    }
}

impl From<u64> for Imm {
    fn from(i: u64) -> Self {
        Self(i, ImmByte::Bit64)
    }
}

impl Imm {
    pub fn get_imm(&self) -> Vec<u8> {
        self.1.encode(self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Op2 {
    Imm(Imm),
    Reg(TargetReg),
}

#[cfg(target_arch = "x86")]
impl Op2 {
    fn rex_value(&self) -> u8 {
        0
    }
}

#[cfg(target_arch = "x86_64")]
impl Op2 {
    fn rex_value(&self) -> u8 {
        match self {
            Op2::Imm(Imm(_, _)) => 0,
            Op2::Reg(r) => if r.is_extend() {
                REX_R
            } else {
                0
            },
        }
    }
}

pub struct Inst {
    pub atomic: bool,
    pub long_mode: bool,
    pub opcode: Vec<u8>, // 0~2bytes
    pub op1: Option<Op1>,
    pub op2: Option<TargetReg>,
    pub imm: Option<Imm>,
}

impl Inst {
    pub fn new(atomic: bool, long_mode: bool, opcode: &[u8]) -> Self {
        Inst {
            atomic,
            long_mode,
            opcode: opcode.to_vec(),
            op1: None,
            op2: None,
            imm: None,
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
            let op1_rex = self.op1.map(|op1| op1.rex_value()).unwrap_or(0);
            let op2_rex = self.op2.map(|op2| op2.reg_value()).unwrap_or(0);
            // warning! this logic is not tested
            let op2_rex = if self.op1.is_none() {
                REX_B
            } else {
                op2_rex
            };
            let mut r = vec![REX_W | op1_rex | op2_rex];
            r.extend(self.opcode);
            r
        } else {
            self.opcode
        };
        let (mod_rm, sib, disp) = match (self.op1, self.op2) {
            (None, None) => (None, None, vec![]),
            (None, Some(op2)) => panic!("unsupport None, reg instruction"), //(None, None, vec![], vec![op2 as u8]),
            (Some(op1), None) => {
                let (mod_rm, sib, disp) = op1.to_modrm_sib_disp(TargetReg::from(0));
                (Some(mod_rm), sib, disp)
            }
            (Some(op1), Some(op2)) => {
                let (mod_rm, sib, disp) = op1.to_modrm_sib_disp(op2);
                (Some(mod_rm), sib, disp)
            }
        };
        let imm = if let Some(imm) = self.imm {
            imm.get_imm()
        } else {
            vec![]
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
