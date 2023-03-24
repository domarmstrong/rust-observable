use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};

static TOKEN_NUMBER: AtomicI64 = AtomicI64::new(0);

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct UniqToken {
    id: i64,
}

impl UniqToken {
    fn new() -> UniqToken {
        UniqToken {
            id: TOKEN_NUMBER.fetch_add(1, Ordering::Relaxed)
        }
    }
}

static SUBSCRIPTION_ID: AtomicI64 = AtomicI64::new(0);

pub struct Observable<T> {
    value: T,
    subscribers: HashMap<UniqToken, fn()>,
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

    pub fn subscribe(&mut self, callback: fn()) -> UniqToken {
        let cleanup_token = UniqToken::new();
        self.subscribers.insert(cleanup_token.clone(), callback);
        cleanup_token
    }

    pub fn unsubscribe(&mut self, token: &UniqToken) {
        self.subscribers.remove(token);
    }
}

pub fn observable<T>(initial_value: T) -> Observable<T> {
    Observable {
        value: initial_value,
        subscribers: HashMap::new(),
    }
}