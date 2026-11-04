use crate::fixtures::direction::Direction;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Pies,
    BootTake,
    BootTakeFailed,
    Port {
        name: &'static str,
        direction: Direction,
        id: u64,
    },
    PortFailed {
        name: &'static str,
        direction: Direction,
    },
    Copy {
        source: u64,
        copy: u64,
        uppercase: bool,
    },
    CopyFailed {
        source: u64,
        copy: u64,
        uppercase: bool,
    },
    Injected {
        owner: usize,
    },
    InjectedFailed,
    FaceDirect {
        owner: usize,
        mark: u8,
        token: u64,
    },
    FaceDirectFailed {
        owner: usize,
        mark: u8,
        token: u64,
    },
    Call {
        task: usize,
        wait: u64,
    },
    CallFailed {
        task: usize,
        wait: u64,
    },
    Write {
        bytes: Vec<u8>,
    },
    WriteFailed {
        bytes: Vec<u8>,
    },
    Read,
    ReadData {
        bytes: Vec<u8>,
    },
    ReadEof,
    ReadInterrupt,
    ReadSuspend,
    ReadFailed,
    Drain,
    DrainFailed,
}
