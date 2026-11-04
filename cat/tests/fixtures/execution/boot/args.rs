use std::cell::RefCell;

thread_local! {
    static ARGS: RefCell<Vec<usize>> = const { RefCell::new(Vec::new()) };
}

pub fn args() -> &'static [usize] {
    let values = ARGS.with(|args| args.borrow().clone());
    Box::leak(values.into_boxed_slice())
}

pub fn set(values: &[usize]) {
    ARGS.with(|args| *args.borrow_mut() = values.to_vec());
}

pub fn reset() {
    ARGS.with(|args| args.borrow_mut().clear());
}
