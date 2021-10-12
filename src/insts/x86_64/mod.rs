mod registers;


pub struct RawInst<'a> {
    pub prefixes: &'a [u8], // 0~4bytes
    pub opcode: &'a [u8],   // 0~3bytes
    pub modrm: &'a [u8],    // 0~1byte
    pub sib: &'a [u8],      // 0~1byte
    pub disp: &'a [u8],     // 1/2/4bytes
    pub imm: &'a [u8],      // 1/2/4bytes
}

impl<'a> RawInst<'a> {
    #[inline]
    pub fn new() -> Self {
        todo!()
    }

    #[inline]
    pub fn check(&self) -> bool {
        const RANGE: [usize; 3] = [1, 2, 4];
        self.prefixes.len() > 4 &&
        self.opcode.len() > 3 &&
        self.modrm.len() > 1 &&
        self.sib.len() > 1 &&
        RANGE.contains(&self.disp.len()) &&
        RANGE.contains(&self.imm.len())
    }

    #[inline]
    pub fn encode(&'a self) -> Box<[u8]> {
        [
            self.prefixes,
            self.opcode,
            self.modrm,
            self.sib,
            self.disp,
        ]
        .iter()
        .rev()
        .flat_map(|x| x.iter()).cloned().collect::<Vec<u8>>()
        .into_boxed_slice()
    }
}