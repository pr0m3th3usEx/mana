use mana_core::traits::{
    data_collector::{DataCollector, Event},
    observability::publisher::Publisher,
};

use data_collector::{MockDataCollector, MockDataRepository};
use futures::FutureExt;

mod data_collector;

async fn test_consume() {
    let data_source = MockDataRepository::from(vec!["HELLO".to_string()]);
    let data_collector = MockDataCollector::new(data_source);

    let result_one = data_collector.consume().await;

    assert!(result_one.is_ok());
    assert_eq!(result_one.unwrap(), Some("HELLO".to_string()));

    let result_two = data_collector.consume().await;

    assert!(result_two.is_ok());
    assert_eq!(result_two.unwrap(), None);
}

#[test]
fn it_test_data_collector_consumer() {
    futures::executor::block_on(test_consume());
}

async fn test_subscribe() {
    let data_source = MockDataRepository::from(vec!["HELLO".to_string()]);
    let mut data_collector = MockDataCollector::new(data_source);

    data_collector.subscribe(Event::New, |data| {
        async move {
            assert_eq!(data, "HELLO");
        }
        .boxed()
    });

    let hello = data_collector.consume().await.unwrap().unwrap();
    data_collector.notify(&Event::New, hello).await;
}

#[test]
fn it_test_data_collector_subscription() {
    futures::executor::block_on(test_subscribe());
}
