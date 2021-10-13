pub mod inst_list;
pub mod registers;

use std::panic;

use registers::{modrm, AddrMode, ScaledIndex, TargetReg, APPEND_SIB};

use self::registers::sib;

const PREFIX_LOCK: u8 = 0xF0;
const PREFIX_LONG_MODE: u8 = 0b01001000; // 48

type ModRM = u8;
type Sib = u8;

#[derive(Debug, Clone, Copy)]
pub enum Operator1 {
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

impl Operator1 {
    pub fn to_modrm_sib_disp(self, src_reg: TargetReg) -> (ModRM, Option<Sib>, Vec<u8>) {
        match self {
            Operator1::Direct(reg) => (modrm(AddrMode::Direct, reg, src_reg), None, vec![]),
            Operator1::DeRef(reg, disp) => {
                let addr_mode = usize_boxed_length(disp);
                (
                    modrm(addr_mode, reg, src_reg),
                    None,
                    addr_mode.encode_disp(disp),
                )
            }
            Operator1::ScaleBase(base, index, scale, disp) => {
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

#[derive(Debug, Clone, Copy)]
pub enum Operator2 {
    Imm(u64, ImmByte),
    Register(TargetReg),
}

pub struct Inst {
    pub atomic: bool,
    pub long_mode: bool,
    pub opcode: Vec<u8>, // 0~2bytes
    pub op1: Option<Operator1>,
    pub op2: Option<Operator2>,
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
            let mut r = vec![PREFIX_LONG_MODE];
            r.extend(self.opcode);
            r
        } else {
            self.opcode
        };
        let (mod_rm, sib, disp, imm) = match (self.op1, self.op2) {
            (None, None) => (None, None, vec![], vec![]),
            (None, Some(Operator2::Imm(value, imm_byte))) => {
                (None, None, vec![], imm_byte.encode(value))
            }
            (None, Some(Operator2::Register(op2))) => (None, None, vec![], vec![op2 as u8]),
            (Some(op1), None) => {
                let (mod_rm, sib, disp) = op1.to_modrm_sib_disp(TargetReg::from(0));
                (Some(mod_rm), sib, disp, vec![])
            }
            (Some(op1), Some(Operator2::Register(op2))) => {
                let (mod_rm, sib, disp) = op1.to_modrm_sib_disp(op2);
                (Some(mod_rm), sib, disp, vec![])
            }
            (Some(op1), Some(Operator2::Imm(value, imm_byte))) => {
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
