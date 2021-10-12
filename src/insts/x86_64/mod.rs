pub mod registers;
pub mod inst_list;

use std::panic;

use registers::{ScaledIndex, TargetReg, AddrMode, Digit, APPEND_SIB, modrm};

use self::registers::sib;

const PREFIX_LOCK: u8 = 0xF0;
const PREFIX_LONG_MODE: u8 = 0b01001000; // 48

type ModRM = u8;
type SIB = u8;

#[derive(Debug, Clone, Copy)]
pub enum Operator1 {
    Direct(TargetReg),
    DeRef(TargetReg, usize),
    ScaleBase(TargetReg, TargetReg, ScaledIndex),
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
    pub fn to_modrm_sib_disp(self, src_reg: TargetReg) -> (ModRM, Option<SIB>, Vec<u8>) {
        match self {
            Operator1::Direct(reg) =>
                (modrm(AddrMode::Direct, reg, src_reg), None, vec![]),
            Operator1::DeRef(reg, disp) => {
                let addr_mode = usize_boxed_length(disp);
                (modrm(addr_mode, reg, src_reg), None, addr_mode.encode_disp(disp))
            }
            Operator1::ScaleBase(base, index, scale) => {
                (modrm(AddrMode::RegRef, *APPEND_SIB, src_reg),
            Some(sib(base, scale, index)), vec![])
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

#[derive(Debug, Clone, Copy)]
pub enum Operator2 {
    Imm(u64, ImmByte),
    Register(TargetReg),
}

impl Operator2 {
    pub fn get_register(&self) -> Option<TargetReg> {
        match *self {
            Operator2::Register(reg) => Some(reg),
            _ => None,
        }
    }
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
        if let Operator2::Imm(imm, imm_byte) = self.op2.unwrap() {
            let imm = if let ImmByte::Bit8 = imm_byte {
                (imm as u8).to_ne_bytes().to_vec()
            } else if let ImmByte::Bit16 = imm_byte {
                (imm as u16).to_ne_bytes().to_vec()
            } else if let ImmByte::Bit32 = imm_byte {
                (imm as u32).to_ne_bytes().to_vec()
            } else {
                imm.to_ne_bytes().to_vec()
            };
            let (mod_rm, sib, disp) = if let Some(x) = self.op1 {
                let (mod_rm, sib, disp) = x.to_modrm_sib_disp(TargetReg::from(0));
                (Some(mod_rm), sib, disp)
            } else {
                (None, None, vec![])
            };
            RawInst {
                prefixes,
                opcode,
                modrm: mod_rm,
                sib: sib,
                disp,
                imm,
            }
        } else {
            let op2 = self.op2.unwrap().get_register().unwrap();
            let (mod_rm, sib, disp) = self.op1.unwrap().to_modrm_sib_disp(op2);
            RawInst {
                prefixes,
                opcode,
                modrm: Some(mod_rm),
                sib: sib,
                disp: disp,
                imm: vec![],
            }
        }
    }
}

pub struct RawInst {
    pub prefixes: Vec<u8>, // 0~4bytes
    pub opcode: Vec<u8>,   // 0~3bytes
    pub modrm: Option<ModRM>,
    pub sib: Option<SIB>,
    pub disp: Vec<u8>,     // 1/2/4bytes
    pub imm: Vec<u8>,      // 1/2/4bytes
}

impl RawInst {
    #[inline]
    pub fn new() -> Self {
        todo!()
    }

    #[inline]
    pub fn check(&self) -> bool {
        const RANGE: [usize; 3] = [1, 2, 4];
        self.prefixes.len() > 4 &&
        self.opcode.len() > 3 &&
        RANGE.contains(&self.disp.len()) &&
        RANGE.contains(&self.imm.len())
    }

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