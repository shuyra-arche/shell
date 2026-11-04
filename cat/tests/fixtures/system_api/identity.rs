use std::cell::RefCell;

pub use crate::fixtures::grant::Grant;
pub use crate::fixtures::reply::Reply;
pub use crate::fixtures::wire::Wire;

thread_local! {
    static REPLY: RefCell<Option<Reply>> = const { RefCell::new(None) };
}

pub fn set_reply(reply: Reply) {
    REPLY.with(|current| *current.borrow_mut() = Some(reply));
}

pub(crate) fn take_reply() -> Reply {
    REPLY
        .with(|reply| reply.borrow_mut().take())
        .unwrap_or(Reply::Refused)
}

pub fn reset() {
    REPLY.with(|reply| *reply.borrow_mut() = None);
}
