pub mod authority;
pub mod binding;
pub mod boot;
pub mod current;
pub mod data;
pub mod direction;
pub mod event;
pub mod execution;
pub mod face;
pub mod grant;
pub mod io;
pub mod pie;
pub mod pipe_client;
pub mod port;
pub mod principal;
pub mod read;
pub mod record;
pub mod reply;
pub mod resource;
pub mod shell_api;
pub mod shell_client;
pub mod system_api;
pub mod system_client;
pub mod task_id;
pub mod terminal_client;
pub mod unit;
pub mod wait;
pub mod wire;

pub fn reset() {
    record::reset();
    unit::reset();
    execution::boot::args::reset();
    resource::raw::reset();
    shell_client::reset();
    system_api::identity::reset();
    system_client::identity::reset();
    terminal_client::reset();
}
