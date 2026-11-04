#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Grant {
    Resolve,
}

impl Grant {
    pub fn mark(self) -> u8 {
        match self {
            Self::Resolve => 0x52,
        }
    }
}
