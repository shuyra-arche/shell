use crate::fixtures::task_id::TaskId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wire {
    Resolve(TaskId),
}
