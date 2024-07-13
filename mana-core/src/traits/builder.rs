pub trait Builder<T> {
    type Error;

    fn build(self) -> Result<T, Self::Error>;
}
