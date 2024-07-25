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
    value_objects::{configuration::token_info::TokenInfo, tick::Tick},
};
use solana_client::rpc_client::RpcClient;

pub struct PumpDataCollector<'a> {
    token_info: TokenInfo,
    rpc: &'a RpcClient,
    delay: Duration,
    is_running: Arc<Mutex<bool>>,
}

impl<'a> PumpDataCollector<'a> {
    pub fn new(token_info: TokenInfo, rpc: &'a RpcClient, delay: Duration) -> Self {
        Self {
            token_info,
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

                // Data collection


                yield Ok(Tick(Utc::now(), PumpTokenLiquidity::new(1_000_000_000f64, 20.0)));
            }
        }
    }

    async fn stop(&self) -> () {
        *self.is_running.lock().expect("lock poisoned") = false;
    }
}
