use std::cell::Cell;

use crate::fixtures::task_id::TaskId;

thread_local! {
    static SELF_ID: Cell<usize> = const { Cell::new(1) };
}

pub fn self_id() -> TaskId {
    TaskId::new(SELF_ID.with(Cell::get))
}

pub fn set_self_id(id: usize) {
    SELF_ID.with(|self_id| self_id.set(id));
}

pub fn reset() {
    SELF_ID.with(|self_id| self_id.set(1));
}
