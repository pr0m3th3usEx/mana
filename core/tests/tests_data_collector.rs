use core::traits::data_collector::DataCollector;

use data_collector::{MockDataCollector, MockDataRepository};

mod data_collector;

async fn run() {
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
fn main() {
    futures::executor::block_on(run());
}
