use lazy_static::lazy_static;

macro_rules! make_register_enum {
    ($name:ident, $i0:ident, $i1:ident, $i2:ident, $i3:ident, $i4:ident, $i5:ident, $i6:ident, $i7:ident) => {
        #[repr(u8)] // 4bit
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $name {
            $i0 = 0,
            $i1 = 1,
            $i2 = 2,
            $i3 = 3,
            $i4 = 4,
            $i5 = 5,
            $i6 = 6,
            $i7 = 7,
        }
    }
}

make_register_enum!(Register8, AL, CL, DL, BL, AH, CH, DH, BH);
make_register_enum!(Register16, AX, CX, DX, BX, SP, BP, SI, DI);
make_register_enum!(Register32, EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI);
#[cfg(target_arch="x86_64")]
// make_register_enum!(Register64, todo);
make_register_enum!(RegisterMme, MM0, MM1, MM2, MM3, MM4, MM5, MM6, MM7);
make_register_enum!(RegisterXmm, XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7);


#[repr(u8)] // 4bit
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddrMode {
    RegRef  = 0,
    Disp8   = 1,
    Disp32  = 2,
    Direct  = 3,
}


pub type Digit = u8;
// #[cfg(target_arch="x86")]
pub type TargetReg = Register32;
#[cfg(target_arch="x86_64")]
// pub type AddressRegister = Register64;

lazy_static! {
    pub static ref APPEND_SIB: TargetReg = unsafe { std::mem::transmute_copy(&4u8) };
    pub static ref DISP32: TargetReg = unsafe { std::mem::transmute_copy(&5u8) };
}


#[inline]
pub fn modrm(
    digit: Digit,
    addr_mode: AddrMode,
    dgt_reg: TargetReg,
    src_reg: TargetReg) -> u8 {
    let r = dgt_reg as u8;
    let r = r + ((addr_mode as u8) << 3u8);
    let r = r + ((src_reg as u8) << 5u8);
    let r = r + ((digit as u8) << 8u8);
    r
}

#[repr(u8)] // 2bit
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScaledIndex {
    Id   = 0,
    Mul2 = 1,
    Mul4 = 2,
    Mul8 = 3,
}

#[derive(Debug, Clone, Copy)]
pub struct SibInvalidError();

#[inline]
fn sib_check(base: &TargetReg, index: &TargetReg) -> Result<(), SibInvalidError> {
    if let TargetReg::EBP = base {
        return Err(SibInvalidError());
    }
    if let TargetReg::ESP = index {
        return Err(SibInvalidError());
    }
    Ok(())
}

#[inline]
pub fn sib(base: TargetReg, scale: ScaledIndex, index: TargetReg) -> u8 {
    sib_check(&base, &index).unwrap();
    let r = base as u8;
    let r = r + ((index as u8) << 3u8);
    let r = r + ((scale as u8) << 6u8);
    r
}