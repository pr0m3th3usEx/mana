use std::hash::Hash;

pub type Subscriber<U> = fn(data: U) -> (dyn std::future::Future<Output = ()> + Send);

pub trait Publisher<T: Hash, U> {
    fn subscribe(&mut self, event: T, observer: Subscriber<U>);

    async fn notify(&self, event: T, data: U);
}
