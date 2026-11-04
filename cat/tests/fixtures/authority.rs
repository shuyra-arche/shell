#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Authority(usize);

impl Authority {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn get(self) -> usize {
        self.0
    }
}
