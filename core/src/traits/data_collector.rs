use std::fmt::Debug;

use super::observability::publisher::Publisher;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Event {
    New,
}

pub trait DataCollector<T>: Publisher<Event, T> {
    // Initiates data collection from source
    async fn start(&self);

    // Stops data collection from source
    async fn stop(&self);
}
