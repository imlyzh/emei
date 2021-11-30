#[cfg(target_arch = "x86")]
pub mod x86_64;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;

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
