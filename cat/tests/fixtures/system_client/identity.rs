use std::cell::Cell;

pub use crate::fixtures::face::Face;

thread_local! {
    static DIRECT_FAILS: Cell<bool> = const { Cell::new(false) };
    static CALL_FAILS: Cell<bool> = const { Cell::new(false) };
}

pub fn fail_direct() {
    DIRECT_FAILS.with(|failed| failed.set(true));
}

pub fn fail_call() {
    CALL_FAILS.with(|failed| failed.set(true));
}

pub(crate) fn direct_fails() -> bool {
    DIRECT_FAILS.with(Cell::get)
}

pub(crate) fn call_fails() -> bool {
    CALL_FAILS.with(Cell::get)
}

pub fn reset() {
    DIRECT_FAILS.with(|failed| failed.set(false));
    CALL_FAILS.with(|failed| failed.set(false));
}
