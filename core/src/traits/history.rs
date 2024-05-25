use chrono::{DateTime, Utc};

pub trait History<T> {
    type InsertError;

    fn insert(&mut self, timestamp: DateTime<Utc>, data: T) -> Result<(), Self::InsertError>;
}
