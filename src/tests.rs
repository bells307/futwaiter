use futures::executor;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

#[test]
fn test_futwaiter() {
    executor::block_on(async {
        crate::set_global();

        let max = 1000;

        let cnt = Arc::new(AtomicUsize::new(0));

        for _ in 0..max {
            let cnt = Arc::clone(&cnt);
            crate::push(async move {
                cnt.fetch_add(1, Ordering::Release);
            });
        }

        crate::take().await;

        assert_eq!(cnt.load(Ordering::Acquire), max);
    });
}
