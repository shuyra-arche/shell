use crate::fixtures::event::Event;
use crate::fixtures::grant::Grant;
use crate::fixtures::record;
use crate::fixtures::reply::Reply;
use crate::fixtures::system_api::identity::take_reply;
use crate::fixtures::system_client::identity::{call_fails, direct_fails};
use crate::fixtures::task_id::TaskId;
use crate::fixtures::wait::Wait;
use crate::fixtures::wire::Wire;

pub struct Face {
    owner: TaskId,
    token: u64,
}

impl Face {
    pub fn direct(owner: TaskId, grant: Grant, token: u64) -> Result<Self, ()> {
        record::push(Event::FaceDirect {
            owner: owner.get(),
            mark: grant.mark(),
            token,
        });
        if direct_fails() {
            record::push(Event::FaceDirectFailed {
                owner: owner.get(),
                mark: grant.mark(),
                token,
            });
            return Err(());
        }
        Ok(Self { owner, token })
    }

    pub fn call(&self, wire: Wire, wait: Wait) -> Result<Reply, ()> {
        let task = match wire {
            Wire::Resolve(task) => task.get(),
        };
        let wait = match wait {
            Wait::AtMost(wait) => wait,
        };
        record::push(Event::Call { task, wait });
        if call_fails() {
            record::push(Event::CallFailed { task, wait });
            return Err(());
        }
        let _ = (self.owner, self.token);
        Ok(take_reply())
    }
}
