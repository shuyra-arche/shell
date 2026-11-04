#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wait {
    AtMost(u64),
}
