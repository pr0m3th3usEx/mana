use std::collections::HashMap;

use crate::{
    builders::configuration_builder::inputs::BotConfigInput, entities::configuration::Metrics,
    value_objects::order::stop_loss::StopLossOrder,
};

pub const DEFAULT_SLIPPAGE_TOLERANCE: f64 = 0.5;
pub const DEFAULT_PRIORITY_FEE: f64 = 0.0001;

#[derive(Debug, Default)]
pub struct BotSettings {
    metrics: HashMap<String, Metrics>,
    stop_loss_orders: Vec<StopLossOrder>,
    slippage_tolerance: f64,
    priority_fee: f64,
}

impl BotSettings {
    pub fn slippage_tolerance(&self) -> f64 {
        self.slippage_tolerance
    }

    pub fn priority_fee(&self) -> f64 {
        self.priority_fee
    }

    pub fn metrics(&self) -> &HashMap<String, Metrics> {
        &self.metrics
    }

    pub fn stop_loss_orders(&self) -> &Vec<StopLossOrder> {
        &self.stop_loss_orders
    }
}

impl TryFrom<BotConfigInput> for BotSettings {
    type Error = &'static str;

    fn try_from(input: BotConfigInput) -> Result<Self, Self::Error> {
        println!("{:?}", input);

        let slippage_tolerance = if let Some(slippage) = input.slippage {
            if slippage <= 0f64 {
                return Err("BotConfigInput: negative slippage tolerance");
            }
            slippage
        } else {
            DEFAULT_SLIPPAGE_TOLERANCE
        };

        let priority_fee = if let Some(priority_fee) = input.priority_fee {
            if priority_fee <= 0f64 {
                return Err("BotConfigInput: negative priority_fee tolerance");
            }
            priority_fee
        } else {
            DEFAULT_PRIORITY_FEE
        };

        Ok(Self {
            priority_fee,
            slippage_tolerance,
            stop_loss_orders: vec![],
            metrics: HashMap::new(),
        })
    }
}
