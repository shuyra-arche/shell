mod failures;
mod shell;
mod terminal;

use crate::fixtures::authority::Authority;
use crate::fixtures::binding::Binding;
use crate::fixtures::current::Current;
use crate::fixtures::grant::Grant;
use crate::fixtures::pie::Pie;
use crate::fixtures::principal::Principal;
use crate::fixtures::shell_api;
use crate::fixtures::task_id::TaskId;

pub(crate) fn pie(owner: usize, mark: u8, token: u64) -> Pie {
    Pie {
        owner: TaskId::new(owner),
        mark,
        token,
        alive: true,
        kind: 0,
        vestor: 0,
    }
}

pub(crate) fn shell_pie(owner: usize, token: u64) -> Pie {
    pie(owner, shell_api::MARK, token)
}

pub(crate) fn resolve_pie(owner: usize, token: u64) -> Pie {
    pie(owner, Grant::Resolve.mark(), token)
}

pub(crate) fn binding(authority: usize, slot: usize) -> Binding {
    Binding {
        current: Current {
            principal: Principal {
                authority: Authority::new(authority),
                slot,
            },
        },
    }
}

pub(crate) fn greeting(task: usize, authority: usize, slot: usize) -> Vec<u8> {
    format!("cat: task={task} principal={authority}:{slot}\n").into_bytes()
}
