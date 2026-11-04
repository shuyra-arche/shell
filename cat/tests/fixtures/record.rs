use std::cell::RefCell;

use crate::fixtures::event::Event;

thread_local! {
    static EVENTS: RefCell<Vec<Event>> = const { RefCell::new(Vec::new()) };
}

pub fn push(event: Event) {
    EVENTS.with(|events| events.borrow_mut().push(event));
}

pub fn events() -> Vec<Event> {
    EVENTS.with(|events| events.borrow().clone())
}

pub fn reset() {
    EVENTS.with(|events| events.borrow_mut().clear());
}
