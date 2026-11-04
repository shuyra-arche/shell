use std::cell::{Cell, RefCell};
use std::collections::VecDeque;

pub use crate::fixtures::io::Io;
pub use crate::fixtures::read::Read;

use crate::fixtures::event::Event;
use crate::fixtures::record;

thread_local! {
    static INJECTED_FAILS: Cell<bool> = const { Cell::new(false) };
    static READ_FAILS: Cell<bool> = const { Cell::new(false) };
    static DRAIN_FAILS: Cell<bool> = const { Cell::new(false) };
    static WRITE_FAIL_AT: Cell<Option<usize>> = const { Cell::new(None) };
    static READS: RefCell<VecDeque<Read>> = const { RefCell::new(VecDeque::new()) };
    static WRITES: Cell<usize> = const { Cell::new(0) };
}

pub fn set_reads(reads: Vec<Read>) {
    READS.with(|script| *script.borrow_mut() = reads.into());
}

pub fn fail_injected() {
    INJECTED_FAILS.with(|failed| failed.set(true));
}

pub fn fail_read() {
    READ_FAILS.with(|failed| failed.set(true));
}

pub fn fail_drain() {
    DRAIN_FAILS.with(|failed| failed.set(true));
}

pub fn fail_write_at(index: usize) {
    WRITE_FAIL_AT.with(|fail_at| fail_at.set(Some(index)));
}

pub(crate) fn injected_fails() -> bool {
    INJECTED_FAILS.with(Cell::get)
}

pub(crate) fn read() -> Result<Read, ()> {
    record::push(Event::Read);
    if READ_FAILS.with(Cell::get) {
        record::push(Event::ReadFailed);
        return Err(());
    }
    let next = READS
        .with(|script| script.borrow_mut().pop_front())
        .expect("test read script exhausted");
    match &next {
        Read::Data(data) => record::push(Event::ReadData {
            bytes: data.bytes().to_vec(),
        }),
        Read::Eof => record::push(Event::ReadEof),
        Read::Interrupt => record::push(Event::ReadInterrupt),
        Read::Suspend => record::push(Event::ReadSuspend),
    }
    Ok(next)
}

pub(crate) fn write(bytes: &[u8]) -> Result<(), ()> {
    record::push(Event::Write {
        bytes: bytes.to_vec(),
    });
    let ordinal = WRITES.with(|writes| {
        let ordinal = writes.get() + 1;
        writes.set(ordinal);
        ordinal
    });
    if WRITE_FAIL_AT.with(Cell::get) == Some(ordinal) {
        record::push(Event::WriteFailed {
            bytes: bytes.to_vec(),
        });
        return Err(());
    }
    Ok(())
}

pub(crate) fn drain() -> Result<(), ()> {
    record::push(Event::Drain);
    if DRAIN_FAILS.with(Cell::get) {
        record::push(Event::DrainFailed);
        return Err(());
    }
    Ok(())
}

pub fn reset() {
    INJECTED_FAILS.with(|failed| failed.set(false));
    READ_FAILS.with(|failed| failed.set(false));
    DRAIN_FAILS.with(|failed| failed.set(false));
    WRITE_FAIL_AT.with(|ordinal| ordinal.set(None));
    READS.with(|script| script.borrow_mut().clear());
    WRITES.with(|writes| writes.set(0));
}
