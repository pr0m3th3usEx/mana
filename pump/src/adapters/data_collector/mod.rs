use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    utils::{bc::get_bonding_curve_info, price::pump_token_price},
    value_objects::token_liquidity::PumpTokenLiquidity,
};
use async_stream::stream;
use chrono::Utc;
use futures::Stream;
use mana_core::{
    traits::data_collector::{DataCollector, DataCollectorError, DataCollectorResult},
    value_objects::{configuration::token_info::TokenInfo, tick::Tick},
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::LAMPORTS_PER_SOL;

pub struct PumpDataCollector<'a> {
    token_info: &'a TokenInfo,
    rpc: &'a RpcClient,
    delay: Duration,
    is_running: Arc<Mutex<bool>>,
}

impl<'a> PumpDataCollector<'a> {
    pub fn new(token_info: &'a TokenInfo, rpc: &'a RpcClient, delay: Duration) -> Self {
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

        let bonding_curve = get_bonding_curve_info(&self.token_info.address);

        stream! {
            loop {
                {
                    let is_running = self.is_running.lock().expect("poisoned");
                    if !(*is_running) {
                        break;
                    }
                }
                thread::sleep(self.delay);

                // Get amount of lamports in liquidity pool from owner account
                let sol_pool =  self.rpc
                    .get_account(&bonding_curve.0) // Owner account
                    .map_err(|err| DataCollectorError::ServiceError(err.to_string()))?
                    .lamports as f64 / LAMPORTS_PER_SOL as f64 ;
                let tokens_pool = self.rpc
                    .get_token_account_balance(&bonding_curve.1) // Associated Token account
                    .map_err(|err| DataCollectorError::ServiceError(err.to_string()))?
                    .ui_amount
                    .expect("should have an amount");
                let price_per_token = pump_token_price(sol_pool);

                yield Ok(Tick(Utc::now(), PumpTokenLiquidity::new(tokens_pool, sol_pool, price_per_token)));
            }
        }
    }

    async fn stop(&self) -> () {
        *self.is_running.lock().expect("lock poisoned") = false;
    }
}
