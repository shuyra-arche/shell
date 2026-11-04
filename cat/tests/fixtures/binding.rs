use crate::fixtures::current::Current;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Binding {
    pub current: Current,
}
