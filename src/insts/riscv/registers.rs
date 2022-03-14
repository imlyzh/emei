

pub type CReg = Reg;

// 3bits
pub type WCReg = u8;
/*

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CReg (pub WCReg);

impl CReg {
    pub fn new(value: u8) -> CReg {
        assert!(value <= 0b111);
        CReg(value)
    }
}

impl Into<Reg> for CReg {
    fn into(self) -> Reg {
        Reg(self.0 + X8)
    }
}
 */

pub const CX0: WCReg = 0;
pub const CX1: WCReg = 1;
pub const CX2: WCReg = 2;
pub const CX3: WCReg = 3;
pub const CX4: WCReg = 4;
pub const CX5: WCReg = 5;
pub const CX6: WCReg = 6;
pub const CX7: WCReg = 7;

// alias registers

pub const CS0: WCReg = CX0;
pub const CS1: WCReg = CX1;
pub const CA0: WCReg = CX2;
pub const CA1: WCReg = CX3;
pub const CA2: WCReg = CX4;
pub const CA3: WCReg = CX5;
pub const CA4: WCReg = CX6;
pub const CA5: WCReg = CX7;

pub const C_FUNCTION_RETURN_VALUE_REGISTERS: [WCReg; 2] = [CA0, CA1];

pub const C_FUNCTION_ARGUMENT_REGISTERS: [WCReg; 6] = [CA0, CA1, CA2, CA3, CA4, CA5];

pub const C_SAVED_REGISTERS: [WReg; 2] = [CS0, CS1];

// 5bits
pub type WReg = u8;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Reg (pub WReg);

pub type Reg = lyuu_commons::isa::riscv::Reg;

/*
impl Reg {
    pub fn new(value: u8) -> Reg {
        assert!(value <= 0b11111);
        Reg(value)
    }
}
 */

 /*
impl TryInto<CReg> for Reg {
    type Error = ();
    fn try_into(self) -> Result<CReg, Self::Error> {
        if self.0 >= X8 && self.0 <= X15 {
            Ok(CReg::new(self.0 - X8))
        } else {
            Err(())
        }
    }
}
 */

pub const X0: WReg = 0;
pub const X1: WReg = 1;
pub const X2: WReg = 2;
pub const X3: WReg = 3;
pub const X4: WReg = 4;
pub const X5: WReg = 5;
pub const X6: WReg = 6;
pub const X7: WReg = 7;
pub const X8: WReg = 8;
pub const X9: WReg = 9;
pub const X10: WReg = 10;
pub const X11: WReg = 11;
pub const X12: WReg = 12;
pub const X13: WReg = 13;
pub const X14: WReg = 14;
pub const X15: WReg = 15;
pub const X16: WReg = 16;
pub const X17: WReg = 17;
pub const X18: WReg = 18;
pub const X19: WReg = 19;
pub const X20: WReg = 20;
pub const X21: WReg = 21;
pub const X22: WReg = 22;
pub const X23: WReg = 23;
pub const X24: WReg = 24;
pub const X25: WReg = 25;
pub const X26: WReg = 26;
pub const X27: WReg = 27;
pub const X28: WReg = 28;
pub const X29: WReg = 29;
pub const X30: WReg = 30;
pub const X31: WReg = 31;

// alias WRegisters

pub const ZERO: WReg = X0;

pub const RA: WReg = X1;

pub const SP: WReg = X2;
pub const GP: WReg = X3;
pub const TP: WReg = X4;
pub const FP: WReg = X8;

pub const T0: WReg = X5;
pub const T1: WReg = X6;
pub const T2: WReg = X7;
pub const T3: WReg = X28;
pub const T4: WReg = X29;
pub const T5: WReg = X30;
pub const T6: WReg = X31;

pub const A0: WReg = X10;
pub const A1: WReg = X11;
pub const A2: WReg = X12;
pub const A3: WReg = X13;
pub const A4: WReg = X14;
pub const A5: WReg = X15;
pub const A6: WReg = X16;
pub const A7: WReg = X17;

pub const S0: WReg = X8;
pub const S1: WReg = X9;
pub const S2: WReg = X18;
pub const S3: WReg = X19;
pub const S4: WReg = X20;
pub const S5: WReg = X21;
pub const S6: WReg = X22;
pub const S7: WReg = X23;
pub const S8: WReg = X24;
pub const S9: WReg = X25;
pub const S10: WReg = X26;
pub const S11: WReg = X27;

pub const POINTER_REGISTERS: [WReg; 4] = [SP, GP, TP, FP];

pub const TEMPORARY_REGISTERS: [WReg; 7] = [T0, T1, T2, T3, T4, T5, T6];

pub const FUNCTION_RETURN_VALUE_REGISTERS: [WReg; 2] = [A0, A1];

pub const FUNCTION_ARGUMENT_REGISTERS: [WReg; 8] = [A0, A1, A2, A3, A4, A5, A6, A7];

pub const SAVED_REGISTERS: [WReg; 12] = [S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11];

// float WRegisters

pub const F0: WReg = 0;
pub const F1: WReg = 1;
pub const F2: WReg = 2;
pub const F3: WReg = 3;
pub const F4: WReg = 4;
pub const F5: WReg = 5;
pub const F6: WReg = 6;
pub const F7: WReg = 7;
pub const F8: WReg = 8;
pub const F9: WReg = 9;
pub const F10: WReg = 10;
pub const F11: WReg = 11;
pub const F12: WReg = 12;
pub const F13: WReg = 13;
pub const F14: WReg = 14;
pub const F15: WReg = 15;
pub const F16: WReg = 16;
pub const F17: WReg = 17;
pub const F18: WReg = 18;
pub const F19: WReg = 19;
pub const F20: WReg = 20;
pub const F21: WReg = 21;
pub const F22: WReg = 22;
pub const F23: WReg = 23;
pub const F24: WReg = 24;
pub const F25: WReg = 25;
pub const F26: WReg = 26;
pub const F27: WReg = 27;
pub const F28: WReg = 28;
pub const F29: WReg = 29;
pub const F30: WReg = 30;
pub const F31: WReg = 31;

// alias WRegisters

pub const FT0: WReg = F0;
pub const FT1: WReg = F1;
pub const FT2: WReg = F2;
pub const FT3: WReg = F3;
pub const FT4: WReg = F4;
pub const FT5: WReg = F5;
pub const FT6: WReg = F6;
pub const FT7: WReg = F7;
pub const FT8: WReg = F28;
pub const FT9: WReg = F29;
pub const FT10: WReg = F30;
pub const FT11: WReg = F31;

pub const FA0: WReg = F11;
pub const FA1: WReg = F12;
pub const FA2: WReg = F13;
pub const FA3: WReg = F14;
pub const FA4: WReg = F15;
pub const FA5: WReg = F16;
pub const FA6: WReg = F17;
pub const FA7: WReg = F18;

pub const FS0: WReg = F8;
pub const FS1: WReg = F9;
pub const FS2: WReg = F18;
pub const FS3: WReg = F19;
pub const FS4: WReg = F20;
pub const FS5: WReg = F21;
pub const FS6: WReg = F22;
pub const FS7: WReg = F23;
pub const FS8: WReg = F24;
pub const FS9: WReg = F25;
pub const FS10: WReg = F26;
pub const FS11: WReg = F27;

pub const FLOAT_TEMPORARY_REGISTERS: [WReg; 12] = [FT0, FT1, FT2, FT3, FT4, FT5, FT6, FT7, FT8, FT9, FT10, FT11];

pub const FLOAT_FUNCTION_RETURN_VALUE_REGISTERS: [WReg; 2] = [FA0, FA1];

pub const FLOAT_FUNCTION_ARGUMENT_REGISTERS: [WReg; 8] = [FA0, FA1, FA2, FA3, FA4, FA5, FA6, FA7];

pub const FLOAT_SAVED_REGISTERS: [WReg; 12] = [FS0, FS1, FS2, FS3, FS4, FS5, FS6, FS7, FS8, FS9, FS10, FS11];
