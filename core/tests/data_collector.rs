use core::traits::{
    data_collector::{DataCollector, DataCollectorResult, Event},
    observability::publisher::{Publisher, Subscriber},
};
use std::{collections::HashMap, sync::RwLock};

use core::traits::data_repository::{DataRepository, DataRepositoryError, DataRepositoryResult};

/**
 *
 * Mock Data Repository
 *
 */

pub struct MockDataRepository<T: Clone> {
    index: RwLock<usize>,
    seed: Vec<T>,
}

impl<T: Clone> DataRepository<T> for MockDataRepository<T> {
    async fn get_data(&self) -> DataRepositoryResult<Option<T>> {
        let mut index = self
            .index
            .write()
            .map_err(|_| DataRepositoryError::ServiceError("lock_poisoned".to_string()))?;

        match self.seed.get(*index) {
            Some(data) => {
                *index += 1;
                Ok(Some(data.clone()))
            }
            None => Ok(None),
        }
    }
}

impl From<Vec<String>> for MockDataRepository<String> {
    fn from(seed: Vec<String>) -> Self {
        Self {
            index: RwLock::new(0),
            seed,
        }
    }
}

/**
 *
 * Mock Data Collector
 *
 */

pub struct MockDataCollector<T: Clone> {
    source: MockDataRepository<T>,
    subscribers: HashMap<Event, Vec<Subscriber<String>>>,
}

impl<T: Clone> MockDataCollector<T> {
    pub fn new(source: MockDataRepository<T>) -> Self {
        Self {
            source,
            subscribers: HashMap::new(),
        }
    }
}

impl DataCollector<String> for MockDataCollector<String> {
    async fn consume(&self) -> DataCollectorResult<Option<String>> {
        let data = self.source.get_data().await?;

        Ok(data)
    }

    async fn start(&self) {
        // Not need for tests
        todo!()
    }
}

impl Publisher<Event, String> for MockDataCollector<String> {
    async fn notify(&self, event: &Event, data: String) {
        let listeners = self.subscribers.get(event);

        match listeners {
            Some(listeners) => {
                for listener in listeners {
                    listener(data.clone()).await;
                }
            }
            None => (),
        }
    }

    fn subscribe(&mut self, event: Event, observer: Subscriber<String>) {
        match self.subscribers.get_mut(&event) {
            Some(subscribers) => subscribers.push(observer),
            None => {
                self.subscribers.insert(event, vec![observer]);
            }
        };
    }
}
