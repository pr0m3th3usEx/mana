use std::{collections::BTreeMap, future::Future};

use chrono::{DateTime, Utc};

pub trait History<T> {
    type InsertError;

    fn insert(
        &mut self,
        timestamp: DateTime<Utc>,
        data: T,
    ) -> impl Future<Output = Result<(), Self::InsertError>>;

    fn get(
        &self,
        timestamp: &DateTime<Utc>,
    ) -> impl Future<Output = Result<Option<T>, Self::InsertError>>;

    fn get_all(
        &self,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> impl Future<Output = Result<BTreeMap<DateTime<Utc>, T>, Self::InsertError>>;
}
