use std::sync::atomic::{AtomicI64, Ordering};
use observable;

#[test]
fn can_update() {
    let mut ob = observable::observable(4);
    assert_eq!(ob.get(), &4);

    ob.set(8);

    assert_eq!(ob.get(), &8);
}

static SUBSCRIBE_CALL_COUNT: AtomicI64 = AtomicI64::new(0);

#[test]
fn can_subscribe_and_unsubscribe() {
    fn callback1() {
        SUBSCRIBE_CALL_COUNT.fetch_add(1, Ordering::Relaxed);
    }
    fn callback2() {
        SUBSCRIBE_CALL_COUNT.fetch_add(1, Ordering::Relaxed);
    }

    let mut ob = observable::observable(10);

    let unsubscribe1 = ob.subscribe(callback1);
    ob.set(11);
    assert_eq!(SUBSCRIBE_CALL_COUNT.load(Ordering::Relaxed), 1);

    let unsubscribe2 = ob.subscribe(callback2);
    ob.set(12);
    assert_eq!(SUBSCRIBE_CALL_COUNT.load(Ordering::Relaxed), 3);

    ob.unsubscribe(&unsubscribe1);
    ob.set(13);
    assert_eq!(SUBSCRIBE_CALL_COUNT.load(Ordering::Relaxed), 4);

    ob.unsubscribe(&unsubscribe2);
    ob.set(14);
    assert_eq!(SUBSCRIBE_CALL_COUNT.load(Ordering::Relaxed), 4);
}
