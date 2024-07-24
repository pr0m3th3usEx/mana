use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::value_objects::token_liquidity::PumpTokenLiquidity;
use async_stream::stream;
use chrono::Utc;
use futures::Stream;
use mana_core::{
    traits::data_collector::{DataCollector, DataCollectorResult},
    value_objects::tick::Tick,
};
use solana_client::rpc_client::RpcClient;

pub struct PumpDataCollector<'a> {
    rpc: &'a RpcClient,
    delay: Duration,
    is_running: Arc<Mutex<bool>>,
}

impl<'a> PumpDataCollector<'a> {
    pub fn new(rpc: &'a RpcClient, delay: Duration) -> Self {
        Self {
            rpc,
            delay,
            is_running: Arc::new(Mutex::new(false)),
        }
    }
}

impl<'a> DataCollector<Tick<PumpTokenLiquidity>> for PumpDataCollector<'a> {
    async fn start(&self) -> impl Stream<Item = DataCollectorResult<Tick<PumpTokenLiquidity>>> {
        {
            *self.is_running.lock().expect("poisoned") = true;
        }
        stream! {
            loop {
                {
                    let is_running = self.is_running.lock().expect("poisoned");
                    if !(*is_running) {
                        break;
                    }
                }
                thread::sleep(self.delay);
                println!("Here");
                yield Ok(Tick(Utc::now(), PumpTokenLiquidity::new(1_000_000_000f64, 20.0)));
            }
        }
    }

    async fn stop(&self) -> () {
        *self.is_running.lock().expect("lock poisoned") = false;
    }
}
