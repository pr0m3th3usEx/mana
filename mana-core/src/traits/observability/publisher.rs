use std::hash::Hash;

use futures::future::BoxFuture;

pub type Subscriber<U> = fn(data: U) -> BoxFuture<'static, ()>;

pub trait Publisher<T: Hash, U> {
    fn subscribe(&mut self, event: T, observer: Subscriber<U>);

    async fn notify(&self, event: &T, data: U);
}
