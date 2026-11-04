#![no_std]
#![no_main]
extern crate shell_server;
#[mold::entry]
fn main() -> Result<(), env::Reason> {
    shell_server::run().map_err(|message| {
        system_support::debug::put(message);
        86
    })
}
