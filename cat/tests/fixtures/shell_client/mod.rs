use std::cell::Cell;

pub use crate::fixtures::boot::Boot;
pub use crate::fixtures::port::Port;

use crate::fixtures::event::Event;
use crate::fixtures::record;

thread_local! {
    static TAKE_FAILS: Cell<bool> = const { Cell::new(false) };
    static SOURCE_PORT_FAILS: Cell<bool> = const { Cell::new(false) };
    static COPY_PORT_FAILS: Cell<bool> = const { Cell::new(false) };
    static COPY_FAILS: Cell<bool> = const { Cell::new(false) };
}

pub fn fail_take() {
    TAKE_FAILS.with(|failed| failed.set(true));
}

pub fn fail_source_port() {
    SOURCE_PORT_FAILS.with(|failed| failed.set(true));
}

pub fn fail_copy_port() {
    COPY_PORT_FAILS.with(|failed| failed.set(true));
}

pub fn fail_copy() {
    COPY_FAILS.with(|failed| failed.set(true));
}

pub(crate) fn take_fails() -> bool {
    TAKE_FAILS.with(Cell::get)
}

pub(crate) fn port_fails(name: &str) -> bool {
    match name {
        "source" => SOURCE_PORT_FAILS.with(Cell::get),
        "copy" => COPY_PORT_FAILS.with(Cell::get),
        _ => false,
    }
}

pub fn copy(source: Port, copy: Port, uppercase: bool) -> Result<(), ()> {
    let source_id = source.id();
    let copy_id = copy.id();
    record::push(Event::Copy {
        source: source_id,
        copy: copy_id,
        uppercase,
    });
    if COPY_FAILS.with(Cell::get) {
        record::push(Event::CopyFailed {
            source: source_id,
            copy: copy_id,
            uppercase,
        });
        return Err(());
    }
    Ok(())
}

pub fn reset() {
    TAKE_FAILS.with(|failed| failed.set(false));
    SOURCE_PORT_FAILS.with(|failed| failed.set(false));
    COPY_PORT_FAILS.with(|failed| failed.set(false));
    COPY_FAILS.with(|failed| failed.set(false));
}
