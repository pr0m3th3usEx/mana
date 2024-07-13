use mana_core::traits::data_processor::DataProcessor;

use chrono::Utc;
use data_processor::MockDataProcessor;

mod data_processor;

async fn run() {
    let mut data_processor: MockDataProcessor<String> = MockDataProcessor::new();

    assert_eq!(data_processor.period(), 0);
    assert_eq!(data_processor.history().len(), 0);

    let dt = Utc::now();

    data_processor.update(Utc::now(), "Hello".to_string()).await;
    data_processor.update(dt, "World".to_string()).await;

    assert_eq!(data_processor.period(), 2);
    assert_eq!(data_processor.history().len(), 2);
    assert_eq!(
        data_processor.history().get(&dt),
        Some("World".to_string()).as_ref()
    );
}

#[test]
fn it_test_data_processor() {
    futures::executor::block_on(run());
}
