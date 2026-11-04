use crate::cases::{pie, resolve_pie, shell_pie};
use crate::fixtures::data::Data;
use crate::fixtures::direction::Direction;
use crate::fixtures::event::Event;
use crate::fixtures::execution::boot::args;
use crate::fixtures::grant::Grant;
use crate::fixtures::pie::Pie;
use crate::fixtures::read::Read;
use crate::fixtures::record;
use crate::fixtures::reply::Reply;
use crate::fixtures::resource::raw;
use crate::fixtures::shell_client;
use crate::fixtures::system_api::identity;
use crate::fixtures::terminal_client;
use crate::fixtures::unit;
use crate::production;

use super::{binding, greeting};

#[test]
fn selects_shell_for_any_matching_pie_without_terminal_io() {
    crate::fixtures::reset();
    args::set(&[4]);
    unit::set_self_id(42);
    raw::set(vec![shell_pie(1, 1), shell_pie(4, 9)]);

    assert_eq!(production::run(), Ok(()));
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
        ]
    );
}

#[test]
fn wrong_pie_owner_falls_back_to_terminal_identity() {
    crate::fixtures::reset();
    args::set(&[4]);
    unit::set_self_id(42);
    raw::set(vec![shell_pie(5, 9), resolve_pie(6, 77)]);
    identity::set_reply(Reply::Binding(Some(binding(7, 3))));
    terminal_client::set_reads(vec![Read::Data(Data::new(b"x".to_vec())), Read::Eof]);

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
                bytes: b"x".to_vec(),
            },
            Event::Write {
                bytes: b"x".to_vec(),
            },
            Event::Read,
            Event::ReadEof,
            Event::Drain,
        ]
    );
}

#[test]
fn wrong_pie_mark_falls_back_to_terminal_identity() {
    crate::fixtures::reset();
    args::set(&[4]);
    unit::set_self_id(42);
    raw::set(vec![pie(4, 0x99, 9), resolve_pie(6, 77)]);
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
fn outer_shell_selector_ignores_alive_kind_and_vestor() {
    crate::fixtures::reset();
    args::set(&[4]);
    unit::set_self_id(42);
    raw::set(vec![Pie {
        alive: false,
        kind: 1,
        vestor: 2,
        ..shell_pie(4, 9)
    }]);
    shell_client::fail_take();

    assert_eq!(production::run(), Err(1));
    assert_eq!(
        record::events(),
        vec![Event::Pies, Event::BootTake, Event::BootTakeFailed]
    );
}
