pub trait Observer<T> {
    async fn trigger(&self, data: T);
}
