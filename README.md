# futwaiter

`FutWaiter` is the container of `Future`'s that has the ability to complete them all at once.

## Example
```rust
use futures::executor;
use futures_timer::Delay;
use futwaiter::{GlobalWaiter, Waitable};
use std::{
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};

#[derive(Clone)]
struct SomeObject(Arc<AtomicU32>);

/// Implementing waitable Future
impl Waitable for SomeObject {
    async fn wait(self) {
        // some computations ...
        Delay::new(Duration::from_secs(1)).await;
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

fn main() {
    executor::block_on(async {
        futwaiter::set_global();

        let max = 1000;

        let cnt = Arc::new(AtomicU32::new(0));

        for _ in 0..max {
            let cnt = Arc::clone(&cnt);

            // Set global waiter for our object
            let _obj = SomeObject(cnt).global_waiter();
        }

        futwaiter::take().await;

        assert_eq!(cnt.load(Ordering::SeqCst), max);
    });
}

```