use std::cell::RefCell;

use crate::fixtures::event::Event;
use crate::fixtures::pie::Pie;
use crate::fixtures::record;

thread_local! {
    static PIES: RefCell<Vec<Pie>> = const { RefCell::new(Vec::new()) };
}

pub fn pies() -> impl Iterator<Item = Pie> {
    record::push(Event::Pies);
    PIES.with(|pies| pies.borrow().clone().into_iter())
}

pub fn set(pies: Vec<Pie>) {
    PIES.with(|current| *current.borrow_mut() = pies);
}

pub fn reset() {
    PIES.with(|pies| pies.borrow_mut().clear());
}
