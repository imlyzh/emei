

// 5bits
pub type Reg = u8;

pub const X0: Reg = 0;
pub const X1: Reg = 1;
pub const X2: Reg = 2;
pub const X3: Reg = 3;
pub const X4: Reg = 4;
pub const X5: Reg = 5;
pub const X6: Reg = 6;
pub const X7: Reg = 7;
pub const X8: Reg = 8;
pub const X9: Reg = 9;
pub const X10: Reg = 10;
pub const X11: Reg = 11;
pub const X12: Reg = 12;
pub const X13: Reg = 13;
pub const X14: Reg = 14;
pub const X15: Reg = 15;
pub const X16: Reg = 16;
pub const X17: Reg = 17;
pub const X18: Reg = 18;
pub const X19: Reg = 19;
pub const X20: Reg = 20;
pub const X21: Reg = 21;
pub const X22: Reg = 22;
pub const X23: Reg = 23;
pub const X24: Reg = 24;
pub const X25: Reg = 25;
pub const X26: Reg = 26;
pub const X27: Reg = 27;
pub const X28: Reg = 28;
pub const X29: Reg = 29;
pub const X30: Reg = 30;
pub const X31: Reg = 31;

// alias registers

pub const ZERO: Reg = X0;

pub const RA: Reg = X1;

pub const SP: Reg = X2;
pub const GP: Reg = X3;
pub const TP: Reg = X4;
pub const FP: Reg = X8;

pub const T0: Reg = X5;
pub const T1: Reg = X6;
pub const T2: Reg = X7;
pub const T3: Reg = X28;
pub const T4: Reg = X29;
pub const T5: Reg = X30;
pub const T6: Reg = X31;

pub const A0: Reg = X10;
pub const A1: Reg = X11;
pub const A2: Reg = X12;
pub const A3: Reg = X13;
pub const A4: Reg = X14;
pub const A5: Reg = X15;
pub const A6: Reg = X16;
pub const A7: Reg = X17;

pub const S0: Reg = X8;
pub const S1: Reg = X9;
pub const S2: Reg = X18;
pub const S3: Reg = X19;
pub const S4: Reg = X20;
pub const S5: Reg = X21;
pub const S6: Reg = X22;
pub const S7: Reg = X23;
pub const S8: Reg = X24;
pub const S9: Reg = X25;
pub const S10: Reg = X26;
pub const S11: Reg = X27;

pub const POINTER_REGISTERS: [Reg; 4] = [SP, GP, TP, FP];

pub const TEMPORARY_REGISTERS: [Reg; 7] = [T0, T1, T2, T3, T4, T5, T6];

pub const FUNCTION_RETURN_VALUE_REGISTERS: [Reg; 2] = [A0, A1];

pub const FUNCTION_ARGUMENT_REGISTERS: [Reg; 8] = [A0, A1, A2, A3, A4, A5, A6, A7];

pub const SAVED_REGISTERS: [Reg; 12] = [S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11];

