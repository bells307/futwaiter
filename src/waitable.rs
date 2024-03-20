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
pub trait Waiter: Waitable + Clone {
    fn waiter(self, fut_waiter: &FutWaiter) -> Self;
}

impl<W> Waiter for W
where
    W: Waitable + Clone + 'static,
{
    fn waiter(self, fut_waiter: &FutWaiter) -> Self {
        fut_waiter.push(self.clone().wait());
        self
    }
}

#[cfg(feature = "global")]
pub trait GlobalWaiter: Waiter {
    fn global_waiter(self) -> Self;
}

#[cfg(feature = "global")]
impl<W> GlobalWaiter for W
where
    W: Waitable + Clone + 'static,
{
    #[cfg(feature = "global")]
    fn global_waiter(self) -> Self {
        match *FUTWAITER.lock() {
            Some(ref fw) => self.waiter(fw),
            None => panic!("global FUTWAITER is not set"),
        }
    }
}
