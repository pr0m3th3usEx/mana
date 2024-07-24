use std::fmt::Debug;

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Tick<T>(pub DateTime<Utc>, pub T)
where
    T: Debug;
