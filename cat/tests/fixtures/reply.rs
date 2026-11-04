use crate::fixtures::binding::Binding;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reply {
    Binding(Option<Binding>),
    Refused,
}
