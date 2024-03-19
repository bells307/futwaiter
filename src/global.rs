use crate::FutWaiter;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::future::Future;

lazy_static! {
    /// Global [`FutWaiter`] object
    pub(crate) static ref FUTWAITER: Mutex<Option<FutWaiter>> = Mutex::new(None);
}

/// Set global [`FutWaiter`]
pub fn set_global() {
    let mut lock = FUTWAITER.lock();

    match *lock {
        Some(_) => panic!("global FUTWAITER already set"),
        None => *lock = Some(FutWaiter::default()),
    }
}

/// Push [`Future`] to the global [`FutWaiter`]
pub fn push<F>(fut: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    match *FUTWAITER.lock() {
        Some(ref fw) => fw.push(fut),
        None => panic!("global FUTWAITER is not set"),
    }
}

/// Move the value from static reference.
/// The global object will not be set after that.
pub fn take() -> FutWaiter {
    match FUTWAITER.lock().take() {
        Some(fw) => fw,
        None => panic!("global FUTWAITER is not set"),
    }
}
