#[cfg(feature = "global")]
use crate::global::FUTWAITER;
use crate::FutWaiter;
use std::future::Future;

/// Object completion behaviour
pub trait Waitable {
    fn wait(self) -> impl Future<Output = ()> + Send;
}

/// The ability for an object that provides the `Waitable` interface to add its
/// waitable `Future` to the `FutWaiter` container
pub trait WaitObserver: Waitable + Clone {
    fn wait_observer(self, fut_waiter: &FutWaiter) -> Self;
    #[cfg(feature = "global")]
    fn global_wait_observer(self) -> Self;
}

impl<W> WaitObserver for W
where
    W: Waitable + Clone + 'static,
{
    fn wait_observer(self, fut_waiter: &FutWaiter) -> Self {
        fut_waiter.push(self.clone().wait());
        self
    }

    #[cfg(feature = "global")]
    fn global_wait_observer(self) -> Self {
        match *FUTWAITER.lock() {
            Some(ref fw) => self.wait_observer(fw),
            None => panic!("global FUTWAITER is not set"),
        }
    }
}
