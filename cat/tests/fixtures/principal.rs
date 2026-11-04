use crate::fixtures::authority::Authority;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Principal {
    pub authority: Authority,
    pub slot: usize,
}
