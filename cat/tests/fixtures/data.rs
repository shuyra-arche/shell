#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Data(Vec<u8>);

impl Data {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}
