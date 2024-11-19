pub mod inst_dump_buf;
pub mod inst_list;
pub mod registers;

use std::panic;

use registers::{modrm, AddrMode, ScaledIndex, TargetReg, APPEND_SIB};

use self::registers::sib;

const PREFIX_LOCK: u8 = 0xF0;

// W 3 0 = Operand size determined by CS.D
//     1 = 64 Bit Operand Size
// R 2 Extension of the ModR/M reg field
// X 1 Extension of the SIB index field
// B 0 Extension of the ModR/M r/m field, SIB base field, or Opcode reg field

pub const REX_W: u8 = 0b01001000; // 48
pub const REX_R: u8 = 0b01000100; // 44
pub const REX_X: u8 = 0b01000010; // 42
pub const REX_B: u8 = 0b01000001; // 41

///////////////////////////////////////////////////////

type ModRM = u8;
type Sib = u8;

#[derive(Debug, Clone, Copy)]
pub enum Op1 {
    Direct(TargetReg),
    DeRef(TargetReg, usize),
    ScaleBase(TargetReg, TargetReg, ScaledIndex, usize), // base index scaleindex disp
}

impl Op1 {
    fn rex_value(&self) -> u8 {
        match self {
            Op1::Direct(r) => {
                if r.is_extend() {
                    REX_B
                } else {
                    0
                }
            }
            Op1::DeRef(r, _) => {
                if r.is_extend() {
                    REX_B
                } else {
                    0
                }
            }
            Op1::ScaleBase(baser, indexr, _, _) => {
                let a = if baser.is_extend() { REX_B } else { 0 };
                let b = if indexr.is_extend() { REX_X } else { 0 };
                a | b
            }
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

fn to_modrm_sib_disp(this: Op1, src_reg: TargetReg) -> (ModRM, Option<Sib>, Vec<u8>) {
    match this {
        Op1::Direct(reg) => (
            modrm(AddrMode::Direct, reg.get_reg(), src_reg.get_reg()),
            None,
            vec![],
        ),
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
                modrm(addr_mode, APPEND_SIB, src_reg.get_reg()),
                Some(sib(base, scale, index)),
                addr_mode.encode_disp(disp),
            )
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Imm(u64, ImmByte);

#[derive(Debug, Clone, Copy)]
pub enum ImmByte {
    Bit8,
    Bit16,
    Bit32,
    Bit64,
}

impl ImmByte {
    pub fn encode(self, imm: u64) -> Vec<u8> {
        if let ImmByte::Bit8 = self {
            (imm as u8).to_ne_bytes().to_vec()
        } else if let ImmByte::Bit16 = self {
            (imm as u16).to_ne_bytes().to_vec()
        } else if let ImmByte::Bit32 = self {
            (imm as u32).to_ne_bytes().to_vec()
        } else {
            imm.to_ne_bytes().to_vec()
        }
    }
}

// impl From<Register64> for Imm {
//     fn from(i: Register64) -> Self {
//         Self(i.reg_value() as u64, ImmByte::Bit8)
//     }
// }

// impl From<Register32> for Imm {
//     fn from(i: Register32) -> Self {
//         Self(i as u64, ImmByte::Bit8)
//     }
// }

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

fn inst(
    atomic: bool,
    long_mode: bool,
    opcode: &[u8], // 0~2bytes
    op1: Option<Op1>,
    op2: Option<TargetReg>,
    imm: Option<Imm>,
) -> Vec<u8> {
    let prefixes = if atomic { vec![PREFIX_LOCK] } else { vec![] };
    let opcode = if long_mode {
        let op1_rex = op1.map(|op1| op1.rex_value()).unwrap_or(0);
        let op2_rex = op2.map(|op2| op2.reg_value()).unwrap_or(0);
        // warning! this logic is not tested
        let op2_rex = if op1.is_none() { REX_B } else { op2_rex };
        let mut r = vec![REX_W | op1_rex | op2_rex];
        r.extend(opcode);
        r
    } else {
        opcode.to_vec()
    };
    let (modrm, sib, disp) = match (op1, op2) {
        (None, None) => (None, None, vec![]),
        (None, Some(_op2)) => panic!("unsupport None, reg instruction"), //(None, None, vec![], vec![op2 as u8]),
        (Some(op1), None) => {
            let (mod_rm, sib, disp) = to_modrm_sib_disp(op1, TargetReg::from(0));
            (Some(mod_rm), sib, disp)
        }
        (Some(op1), Some(op2)) => {
            let (mod_rm, sib, disp) = to_modrm_sib_disp(op1, op2);
            (Some(mod_rm), sib, disp)
        }
    };
    let imm = if let Some(imm) = imm {
        imm.get_imm()
    } else {
        vec![]
    };
    encode(&prefixes, &opcode, modrm, sib, &disp, &imm)
}

fn sse_inst(opcode: &[u8], op1: Option<Op1>, op2: Option<TargetReg>, imm: Option<Imm>) -> Vec<u8> {
    inst(false, false, opcode, op1, op2, imm)
}

pub fn encode(
    prefixes: &[u8],
    opcode: &[u8],
    modrm: Option<ModRM>,
    sib: Option<Sib>,
    disp: &[u8],
    imm: &[u8],
) -> Vec<u8> {
    let mut buf = vec![];
    buf.extend(prefixes);
    buf.extend(opcode);
    if let Some(x) = modrm {
        buf.push(x);
    }
    if let Some(x) = sib {
        buf.push(x);
    }
    buf.extend(disp);
    buf.extend(imm);
    buf
}
