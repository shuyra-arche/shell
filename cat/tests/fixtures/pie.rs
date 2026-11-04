use crate::fixtures::task_id::TaskId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pie {
    pub owner: TaskId,
    pub mark: u8,
    pub token: u64,
    pub alive: bool,
    pub kind: u8,
    pub vestor: u8,
}
