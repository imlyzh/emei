

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

// float registers

pub const F0: Reg = 0;
pub const F1: Reg = 1;
pub const F2: Reg = 2;
pub const F3: Reg = 3;
pub const F4: Reg = 4;
pub const F5: Reg = 5;
pub const F6: Reg = 6;
pub const F7: Reg = 7;
pub const F8: Reg = 8;
pub const F9: Reg = 9;
pub const F10: Reg = 10;
pub const F11: Reg = 11;
pub const F12: Reg = 12;
pub const F13: Reg = 13;
pub const F14: Reg = 14;
pub const F15: Reg = 15;
pub const F16: Reg = 16;
pub const F17: Reg = 17;
pub const F18: Reg = 18;
pub const F19: Reg = 19;
pub const F20: Reg = 20;
pub const F21: Reg = 21;
pub const F22: Reg = 22;
pub const F23: Reg = 23;
pub const F24: Reg = 24;
pub const F25: Reg = 25;
pub const F26: Reg = 26;
pub const F27: Reg = 27;
pub const F28: Reg = 28;
pub const F29: Reg = 29;
pub const F30: Reg = 30;
pub const F31: Reg = 31;

// alias registers

pub const FT0: Reg = F0;
pub const FT1: Reg = F1;
pub const FT2: Reg = F2;
pub const FT3: Reg = F3;
pub const FT4: Reg = F4;
pub const FT5: Reg = F5;
pub const FT6: Reg = F6;
pub const FT7: Reg = F7;
pub const FT8: Reg = F28;
pub const FT9: Reg = F29;
pub const FT10: Reg = F30;
pub const FT11: Reg = F31;

pub const FA0: Reg = F11;
pub const FA1: Reg = F12;
pub const FA2: Reg = F13;
pub const FA3: Reg = F14;
pub const FA4: Reg = F15;
pub const FA5: Reg = F16;
pub const FA6: Reg = F17;
pub const FA7: Reg = F18;

pub const FS0: Reg = F8;
pub const FS1: Reg = F9;
pub const FS2: Reg = F18;
pub const FS3: Reg = F19;
pub const FS4: Reg = F20;
pub const FS5: Reg = F21;
pub const FS6: Reg = F22;
pub const FS7: Reg = F23;
pub const FS8: Reg = F24;
pub const FS9: Reg = F25;
pub const FS10: Reg = F26;
pub const FS11: Reg = F27;

pub const FLOAT_TEMPORARY_REGISTERS: [Reg; 12] = [FT0, FT1, FT2, FT3, FT4, FT5, FT6, FT7, FT8, FT9, FT10, FT11];

pub const FLOAT_FUNCTION_RETURN_VALUE_REGISTERS: [Reg; 2] = [FA0, FA1];

pub const FLOAT_FUNCTION_ARGUMENT_REGISTERS: [Reg; 8] = [FA0, FA1, FA2, FA3, FA4, FA5, FA6, FA7];

pub const FLOAT_SAVED_REGISTERS: [Reg; 12] = [FS0, FS1, FS2, FS3, FS4, FS5, FS6, FS7, FS8, FS9, FS10, FS11];
