//! Register definitions

#[const_trait]
pub trait Register: Copy {
    const ADDRESS: u16;
    fn bits(&self) -> u8;
    fn from_bits(bits: u8) -> Self;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LoraSyncWordMsb(pub u8);

impl const Register for LoraSyncWordMsb {
    const ADDRESS: u16 = 0x0740;
    fn bits(&self) -> u8 {
        self.0
    }
    fn from_bits(bits: u8) -> Self {
        Self(bits)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LoraSyncWordLsb(pub u8);

impl const Register for LoraSyncWordLsb {
    const ADDRESS: u16 = 0x0741;
    fn bits(&self) -> u8 {
        self.0
    }
    fn from_bits(bits: u8) -> Self {
        Self(bits)
    }
}
