use crate::fixtures::event::Event;
use crate::fixtures::read::Read;
use crate::fixtures::record;
use crate::fixtures::task_id::TaskId;
use crate::fixtures::terminal_client;

pub struct Io {
    owner: TaskId,
}

impl Io {
    pub fn injected(owner: TaskId) -> Result<Self, ()> {
        record::push(Event::Injected {
            owner: owner.get(),
        });
        if terminal_client::injected_fails() {
            record::push(Event::InjectedFailed);
            return Err(());
        }
        Ok(Self { owner })
    }

    pub fn read(&self) -> Result<Read, ()> {
        let _ = self.owner;
        terminal_client::read()
    }

    pub fn write(&self, bytes: &[u8]) -> Result<(), ()> {
        terminal_client::write(bytes)
    }

    pub fn drain(&self) -> Result<(), ()> {
        terminal_client::drain()
    }
}
