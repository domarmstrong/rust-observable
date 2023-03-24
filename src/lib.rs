use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};

static SUBSCRIPTION_ID: AtomicI64 = AtomicI64::new(0);

pub struct Observable<T> {
    value: T,
    subscribers: HashMap<i64, fn()>,
}

impl<T> Observable<T> {
    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
        for (_, callback) in self.subscribers.iter() {
            callback();
        }
    }

    pub fn subscribe(&mut self, callback: fn()) -> impl FnOnce() + '_ {
        let id = SUBSCRIPTION_ID.fetch_add(1, Ordering::Relaxed);
        self.subscribers.insert(id, callback);
        move || {
            self.subscribers.remove(&id);
        }
    }
}

pub fn observable<T>(initial_value: T) -> Observable<T> {
    Observable {
        value: initial_value,
        subscribers: HashMap::new(),
    }
}