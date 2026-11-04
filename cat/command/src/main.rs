#![no_std]
#![no_main]

#[mold::entry]
fn main() -> Result<(), env::Reason> {
    cat::run()
}
