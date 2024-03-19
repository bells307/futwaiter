mod waitable;

#[cfg(feature = "global")]
mod global;

#[cfg(test)]
mod tests;

pub use crate::waitable::{WaitObserver, Waitable};

#[cfg(feature = "global")]
pub use crate::global::{push, set_global, take};

use futures::{
    future::{BoxFuture, JoinAll},
    FutureExt,
};
use parking_lot::Mutex;
use pin_project_lite::pin_project;
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

pin_project! {
    /// The container of [`Future`]'s that has the ability to complete them all at once
    #[derive(Default)]
    pub struct FutWaiter {
        futs: Arc<Mutex<Option<Vec<BoxFuture<'static, ()>>>>>,
        #[pin]
        join_all: Arc<Mutex<Option<JoinAll<BoxFuture<'static, ()>>>>>,
    }
}

impl FutWaiter {
    pub fn new() -> Self {
        Default::default()
    }

    /// Add [`Future`] to container
    pub fn push<F>(&self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let fut = fut.boxed();
        let mut lock = self.futs.lock();
        match *lock {
            Some(ref mut futs) => futs.push(fut),
            None => *lock = Some(vec![fut]),
        };
    }
}

impl Future for FutWaiter {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let proj = self.project();

        let mut lock = proj.join_all.lock();

        match lock.take() {
            Some(mut join_all) => match join_all.poll_unpin(cx).map(|_| ()) {
                Poll::Ready(_) => {
                    // If JoinAll was previously registered, then set its value to `None`
                    *lock = None;
                    Poll::Ready(())
                }
                Poll::Pending => {
                    // Take the value back
                    *lock = Some(join_all);
                    Poll::Pending
                }
            },
            None => {
                // JoinAll was not previously set, so we take the Futures and try to complete them
                match proj.futs.lock().take() {
                    Some(futs) => {
                        let mut join_all = futures::future::join_all(futs);
                        match join_all.poll_unpin(cx) {
                            Poll::Ready(_) => Poll::Ready(()),
                            Poll::Pending => {
                                *lock = Some(join_all);
                                Poll::Pending
                            }
                        }
                    }
                    None => Poll::Ready(()),
                }
            }
        }
    }
}
