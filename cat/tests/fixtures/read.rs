use crate::fixtures::data::Data;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Read {
    Data(Data),
    Eof,
    Interrupt,
    Suspend,
}
