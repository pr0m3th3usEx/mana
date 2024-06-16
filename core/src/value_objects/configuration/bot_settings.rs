use std::collections::HashMap;

use crate::{entities::configuration::Metrics, value_objects::order::stop_loss::StopLossOrder};

pub const DEFAULT_SLIPPAGE_TOLERANCE: f64 = 0.5;
pub const DEFAULT_MAX_GAS_FEE_ACCEPTED: f64 = 0.0001;

#[derive(Debug, Default)]
pub struct BotSettings {
    metrics: HashMap<String, Metrics>,
    stop_loss_orders: Vec<StopLossOrder>,
    slippage_tolerance: f64,
    max_gas_fees_allowed: f64,
}

impl BotSettings {
    pub fn slippage_tolerance(&self) -> f64 {
        self.slippage_tolerance
    }

    pub fn max_gas_fees_allowed(&self) -> f64 {
        self.max_gas_fees_allowed
    }

    pub fn metrics(&self) -> &HashMap<String, Metrics> {
        &self.metrics
    }

    pub fn stop_loss_orders(&self) -> &Vec<StopLossOrder> {
        &self.stop_loss_orders
    }
}
