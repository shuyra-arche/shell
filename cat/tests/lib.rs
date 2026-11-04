#![allow(dead_code)]

extern crate alloc;
extern crate self as env;
extern crate self as execution;
extern crate self as pipe_client;
extern crate self as resource;
extern crate self as shell_api;
extern crate self as shell_client;
extern crate self as system_api;
extern crate self as system_client;
extern crate self as terminal_client;

#[path = "../command/src/run.rs"]
mod production;

pub mod fixtures;
pub mod identity;

pub use fixtures::authority::Authority;
pub use fixtures::binding::Binding;
pub use fixtures::direction::Direction;
pub use fixtures::execution::boot;
pub use fixtures::face::Face;
pub use fixtures::grant::Grant;
pub use fixtures::io::Io;
pub use fixtures::pie::Pie;
pub use fixtures::port::Port;
pub use fixtures::principal::Principal;
pub use fixtures::read::Read;
pub use fixtures::record;
pub use fixtures::reply::Reply;
pub use fixtures::resource::raw;
pub use fixtures::shell_api::MARK;
pub use fixtures::shell_client::{Boot, copy};
pub use fixtures::task_id::TaskId;
pub use fixtures::unit;
pub use fixtures::wait::Wait;
pub use fixtures::wire::Wire;

pub type Reason = usize;

#[cfg(test)]
mod cases;
