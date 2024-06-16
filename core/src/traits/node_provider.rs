use std::error::Error;

pub trait NodeProvider {
    async fn query() -> Result<(), Box<dyn Error>>;
}
