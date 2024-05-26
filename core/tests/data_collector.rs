use core::traits::{
    data_collector::{DataCollector, Event},
    observability::publisher::{Publisher, Subscriber},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use data_processor::MockDataProcessor;

mod data_processor;

pub enum MockDataCollectorObserver {
    MockDataProcessor(MockDataProcessor),
}

pub struct MockDataCollector {
    subscribers: HashMap<Event, Vec<Arc<RwLock<MockDataCollectorObserver>>>>,
    is_collecting: RwLock<bool>,
}

impl DataCollector<String> for MockDataCollector {
    async fn start(&self) {
        todo!()
    }

    async fn stop(&self) {
        todo!()
    }
}

impl Publisher<Event, String> for MockDataCollector {
    async fn notify(&self, event: Event, data: String) {
        todo!()
    }

    fn subscribe(&mut self, event: Event, observer: Subscriber<String>) {
        todo!()
    }
}
