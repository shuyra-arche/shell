use crate::fixtures::direction::Direction;
use crate::fixtures::event::Event;
use crate::fixtures::port::Port;
use crate::fixtures::record;
use crate::fixtures::shell_client::{port_fails, take_fails};

pub struct Boot {
    next_port: u64,
}

impl Boot {
    pub fn take() -> Result<Self, ()> {
        record::push(Event::BootTake);
        if take_fails() {
            record::push(Event::BootTakeFailed);
            return Err(());
        }
        Ok(Self { next_port: 1 })
    }

    pub fn port(&mut self, name: &'static str, direction: Direction) -> Result<Port, ()> {
        if port_fails(name) {
            record::push(Event::PortFailed { name, direction });
            return Err(());
        }
        let id = self.next_port;
        self.next_port += 1;
        record::push(Event::Port {
            name,
            direction,
            id,
        });
        Ok(Port::new(id))
    }
}
