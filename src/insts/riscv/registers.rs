

// 5bits
pub type Reg = u8;

const X0: Reg = 0;
const X1: Reg = 1;
const X2: Reg = 2;
const X3: Reg = 3;
const X4: Reg = 4;
const X5: Reg = 5;
const X6: Reg = 6;
const X7: Reg = 7;
const X8: Reg = 8;
const X9: Reg = 9;
const X10: Reg = 10;
const X11: Reg = 11;
const X12: Reg = 12;
const X13: Reg = 13;
const X14: Reg = 14;
const X15: Reg = 15;
const X16: Reg = 16;
const X17: Reg = 17;
const X18: Reg = 18;
const X19: Reg = 19;
const X20: Reg = 20;
const X21: Reg = 21;
const X22: Reg = 22;
const X23: Reg = 23;
const X24: Reg = 24;
const X25: Reg = 25;
const X26: Reg = 26;
const X27: Reg = 27;
const X28: Reg = 28;
const X29: Reg = 29;
const X30: Reg = 30;
const X31: Reg = 31;

// alias registers

const ZERO: Reg = X0;

const RA: Reg = X1;

const SP: Reg = X2;
const GP: Reg = X3;
const TP: Reg = X4;
const FP: Reg = X8;

const T0: Reg = X5;
const T1: Reg = X6;
const T2: Reg = X7;
const T3: Reg = X28;
const T4: Reg = X29;
const T5: Reg = X30;
const T6: Reg = X31;

const A0: Reg = X10;
const A1: Reg = X11;
const A2: Reg = X12;
const A3: Reg = X13;
const A4: Reg = X14;
const A5: Reg = X15;
const A6: Reg = X16;
const A7: Reg = X17;

const S0: Reg = X8;
const S1: Reg = X9;
const S2: Reg = X18;
const S3: Reg = X19;
const S4: Reg = X20;
const S5: Reg = X21;
const S6: Reg = X22;
const S7: Reg = X23;
const S8: Reg = X24;
const S9: Reg = X25;
const S10: Reg = X26;
const S11: Reg = X27;

const POINTER_REGISTERS: [Reg; 4] = [SP, GP, TP, FP];

const TEMPORARY_REGISTERS: [Reg; 7] = [T0, T1, T2, T3, T4, T5, T6];

const FUNCTION_RETURN_VALUE_REGISTERS: [Reg; 2] = [A0, A1];

const FUNCTION_ARGUMENT_REGISTERS: [Reg; 8] = [A0, A1, A2, A3, A4, A5, A6, A7];

const SAVED_REGISTERS: [Reg; 12] = [S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11];

