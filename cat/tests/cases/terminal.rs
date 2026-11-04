use crate::fixtures::event::Event;
use crate::fixtures::data::Data;
use crate::fixtures::execution::boot::args;
use crate::fixtures::grant::Grant;
use crate::fixtures::read::Read;
use crate::fixtures::record;
use crate::fixtures::reply::Reply;
use crate::fixtures::resource::raw;
use crate::fixtures::system_api::identity;
use crate::fixtures::terminal_client;
use crate::fixtures::unit;
use crate::production;

use super::{binding, greeting, pie, resolve_pie};

fn terminal_ready() {
    crate::fixtures::reset();
    args::set(&[4]);
    unit::set_self_id(42);
    raw::set(vec![resolve_pie(6, 77)]);
    identity::set_reply(Reply::Binding(Some(binding(7, 3))));
}

#[test]
fn greeting_self_id_deadline_and_first_resolve_mark_pie_are_exact() {
    crate::fixtures::reset();
    args::set(&[4]);
    unit::set_self_id(42);
    raw::set(vec![
        pie(1, 0x99, 1),
        resolve_pie(6, 77),
        resolve_pie(8, 88),
    ]);
    identity::set_reply(Reply::Binding(Some(binding(7, 3))));
    terminal_client::set_reads(vec![Read::Eof]);

    assert_eq!(production::run(), Ok(()));
    assert_eq!(
        record::events(),
        vec![
            Event::Pies,
            Event::Injected { owner: 4 },
            Event::Pies,
            Event::FaceDirect {
                owner: 6,
                mark: Grant::Resolve.mark(),
                token: 77,
            },
            Event::Call {
                task: 42,
                wait: 1000,
            },
            Event::Write {
                bytes: greeting(42, 7, 3),
            },
            Event::Read,
            Event::ReadEof,
            Event::Drain,
        ]
    );
}

#[test]
fn echoes_arbitrary_and_empty_data_verbatim() {
    terminal_ready();
    let first = vec![0x00, 0xff, 0x0a, 0x0d, 0x80];
    let empty = Vec::new();
    let third = b"cat\n".to_vec();
    terminal_client::set_reads(vec![
        Read::Data(Data::new(first.clone())),
        Read::Data(Data::new(empty.clone())),
        Read::Data(Data::new(third.clone())),
        Read::Eof,
    ]);

    assert_eq!(production::run(), Ok(()));
    assert_eq!(
        record::events(),
        vec![
            Event::Pies,
            Event::Injected { owner: 4 },
            Event::Pies,
            Event::FaceDirect {
                owner: 6,
                mark: Grant::Resolve.mark(),
                token: 77,
            },
            Event::Call {
                task: 42,
                wait: 1000,
            },
            Event::Write {
                bytes: greeting(42, 7, 3),
            },
            Event::Read,
            Event::ReadData { bytes: first },
            Event::Write { bytes: b"\x00\xff\x0a\x0d\x80".to_vec() },
            Event::Read,
            Event::ReadData { bytes: empty },
            Event::Write { bytes: Vec::new() },
            Event::Read,
            Event::ReadData { bytes: third },
            Event::Write { bytes: b"cat\n".to_vec() },
            Event::Read,
            Event::ReadEof,
            Event::Drain,
        ]
    );
}

#[test]
fn interrupt_drains_once_and_succeeds_without_another_read() {
    terminal_ready();
    terminal_client::set_reads(vec![
        Read::Data(Data::new(b"a".to_vec())),
        Read::Interrupt,
    ]);

    assert_eq!(production::run(), Ok(()));
    assert_eq!(
        record::events(),
        vec![
            Event::Pies,
            Event::Injected { owner: 4 },
            Event::Pies,
            Event::FaceDirect {
                owner: 6,
                mark: Grant::Resolve.mark(),
                token: 77,
            },
            Event::Call {
                task: 42,
                wait: 1000,
            },
            Event::Write {
                bytes: greeting(42, 7, 3),
            },
            Event::Read,
            Event::ReadData {
                bytes: b"a".to_vec(),
            },
            Event::Write {
                bytes: b"a".to_vec(),
            },
            Event::Read,
            Event::ReadInterrupt,
            Event::Drain,
        ]
    );
}

#[test]
fn suspend_drains_once_and_succeeds_without_another_read() {
    terminal_ready();
    terminal_client::set_reads(vec![Read::Suspend]);

    assert_eq!(production::run(), Ok(()));
    assert_eq!(
        record::events(),
        vec![
            Event::Pies,
            Event::Injected { owner: 4 },
            Event::Pies,
            Event::FaceDirect {
                owner: 6,
                mark: Grant::Resolve.mark(),
                token: 77,
            },
            Event::Call {
                task: 42,
                wait: 1000,
            },
            Event::Write {
                bytes: greeting(42, 7, 3),
            },
            Event::Read,
            Event::ReadSuspend,
            Event::Drain,
        ]
    );
}
