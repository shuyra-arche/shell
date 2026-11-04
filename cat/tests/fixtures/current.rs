use crate::fixtures::principal::Principal;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Current {
    pub principal: Principal,
}
