use crate::fixtures::direction::Direction;
use crate::fixtures::data::Data;
use crate::fixtures::event::Event;
use crate::fixtures::execution::boot::args;
use crate::fixtures::grant::Grant;
use crate::fixtures::read::Read;
use crate::fixtures::record;
use crate::fixtures::reply::Reply;
use crate::fixtures::resource::raw;
use crate::fixtures::shell_client;
use crate::fixtures::system_api::identity;
use crate::fixtures::system_client;
use crate::fixtures::terminal_client;
use crate::fixtures::unit;
use crate::production;

use super::{binding, greeting, pie, resolve_pie, shell_pie};

fn shell_ready() {
    crate::fixtures::reset();
    args::set(&[4]);
    unit::set_self_id(42);
    raw::set(vec![shell_pie(4, 9)]);
}

fn terminal_ready() {
    crate::fixtures::reset();
    args::set(&[4]);
    unit::set_self_id(42);
    raw::set(vec![resolve_pie(6, 77)]);
    identity::set_reply(Reply::Binding(Some(binding(7, 3))));
}

#[test]
fn selected_shell_boot_failure_returns_reason_one_without_fallback() {
    shell_ready();
    shell_client::fail_take();

    assert_eq!(production::run(), Err(1));
    assert_eq!(
        record::events(),
        vec![Event::Pies, Event::BootTake, Event::BootTakeFailed]
    );
}

#[test]
fn selected_shell_source_port_failure_returns_reason_one_without_fallback() {
    shell_ready();
    shell_client::fail_source_port();

    assert_eq!(production::run(), Err(1));
    assert_eq!(
        record::events(),
        vec![
            Event::Pies,
            Event::BootTake,
            Event::PortFailed {
                name: "source",
                direction: Direction::Read,
            },
        ]
    );
}

#[test]
fn selected_shell_copy_port_failure_returns_reason_one_without_fallback() {
    shell_ready();
    shell_client::fail_copy_port();

    assert_eq!(production::run(), Err(1));
    assert_eq!(
        record::events(),
        vec![
            Event::Pies,
            Event::BootTake,
            Event::Port {
                name: "source",
                direction: Direction::Read,
                id: 1,
            },
            Event::PortFailed {
                name: "copy",
                direction: Direction::Write,
            },
        ]
    );
}

#[test]
fn selected_shell_copy_failure_returns_reason_one_without_fallback() {
    shell_ready();
    shell_client::fail_copy();

    assert_eq!(production::run(), Err(1));
    assert_eq!(
        record::events(),
        vec![
            Event::Pies,
            Event::BootTake,
            Event::Port {
                name: "source",
                direction: Direction::Read,
                id: 1,
            },
            Event::Port {
                name: "copy",
                direction: Direction::Write,
                id: 2,
            },
            Event::Copy {
                source: 1,
                copy: 2,
                uppercase: false,
            },
            Event::CopyFailed {
                source: 1,
                copy: 2,
                uppercase: false,
            },
        ]
    );
}

#[test]
fn missing_args_returns_reason_one_before_io() {
    crate::fixtures::reset();

    assert_eq!(production::run(), Err(1));
    assert!(record::events().is_empty());
}

#[test]
fn injected_failure_returns_reason_one() {
    crate::fixtures::reset();
    args::set(&[4]);
    terminal_client::fail_injected();

    assert_eq!(production::run(), Err(1));
    assert_eq!(
        record::events(),
        vec![
            Event::Pies,
            Event::Injected { owner: 4 },
            Event::InjectedFailed,
        ]
    );
}

#[test]
fn missing_resolve_mark_pie_returns_reason_one() {
    crate::fixtures::reset();
    args::set(&[4]);
    raw::set(vec![pie(1, 0x99, 1)]);

    assert_eq!(production::run(), Err(1));
    assert_eq!(
        record::events(),
        vec![
            Event::Pies,
            Event::Injected { owner: 4 },
            Event::Pies,
        ]
    );
}

#[test]
fn direct_binding_failure_returns_reason_one() {
    terminal_ready();
    system_client::identity::fail_direct();

    assert_eq!(production::run(), Err(1));
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
            Event::FaceDirectFailed {
                owner: 6,
                mark: Grant::Resolve.mark(),
                token: 77,
            },
        ]
    );
}

#[test]
fn call_failure_returns_reason_one() {
    terminal_ready();
    system_client::identity::fail_call();

    assert_eq!(production::run(), Err(1));
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
            Event::CallFailed {
                task: 42,
                wait: 1000,
            },
        ]
    );
}

#[test]
fn binding_reply_without_binding_returns_reason_one() {
    terminal_ready();
    identity::set_reply(Reply::Binding(None));

    assert_eq!(production::run(), Err(1));
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
        ]
    );
}

#[test]
fn non_binding_reply_returns_reason_one() {
    terminal_ready();
    identity::set_reply(Reply::Refused);

    assert_eq!(production::run(), Err(1));
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
        ]
    );
}

#[test]
fn greeting_write_failure_returns_reason_one_without_read() {
    terminal_ready();
    let greeting = greeting(42, 7, 3);
    terminal_client::fail_write_at(1);

    assert_eq!(production::run(), Err(1));
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
                bytes: greeting.clone(),
            },
            Event::WriteFailed { bytes: greeting },
        ]
    );
}

#[test]
fn read_failure_returns_reason_one_without_drain() {
    terminal_ready();
    terminal_client::fail_read();

    assert_eq!(production::run(), Err(1));
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
            Event::ReadFailed,
        ]
    );
}

#[test]
fn data_write_failure_returns_reason_one_without_further_read_or_drain() {
    terminal_ready();
    terminal_client::set_reads(vec![
        Read::Data(Data::new(b"abc".to_vec())),
        Read::Eof,
    ]);
    terminal_client::fail_write_at(2);

    assert_eq!(production::run(), Err(1));
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
                bytes: b"abc".to_vec(),
            },
            Event::Write {
                bytes: b"abc".to_vec(),
            },
            Event::WriteFailed {
                bytes: b"abc".to_vec(),
            },
        ]
    );
}

#[test]
fn drain_failure_returns_reason_one() {
    terminal_ready();
    terminal_client::set_reads(vec![Read::Eof]);
    terminal_client::fail_drain();

    assert_eq!(production::run(), Err(1));
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
            Event::DrainFailed,
        ]
    );
}
