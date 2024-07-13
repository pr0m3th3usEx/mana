use crate::{
    builders::configuration_builder::inputs::{BotConfigInput, StopLossConfigInput},
    value_objects::{
        bet::Bet, order::stop_loss::StopLossOrder, priority_fee::PriorityFee,
        slippage_tolerance::SlippageTolerance,
    },
};

pub const DEFAULT_SLIPPAGE_TOLERANCE: f64 = 0.5;
pub const DEFAULT_PRIORITY_FEE: f64 = 0.0001;

#[derive(Debug, Default)]
pub struct BotSettings {
    stop_loss_orders: Vec<StopLossOrder>,
    slippage_tolerance: SlippageTolerance,
    priority_fee: PriorityFee,
    bet: Bet,
}

impl BotSettings {
    pub fn slippage_tolerance(&self) -> SlippageTolerance {
        self.slippage_tolerance
    }

    pub fn priority_fee(&self) -> PriorityFee {
        self.priority_fee
    }

    pub fn bet(&self) -> Bet {
        self.bet
    }

    pub fn stop_loss_orders(&self) -> &Vec<StopLossOrder> {
        &self.stop_loss_orders
    }
}

impl TryFrom<BotConfigInput> for BotSettings {
    type Error = &'static str;

    fn try_from(input: BotConfigInput) -> Result<Self, Self::Error> {
        let slippage_tolerance = input
            .slippage
            .map_or(Ok(SlippageTolerance::default()), SlippageTolerance::new)?;
        let priority_fee = input
            .priority_fee
            .map_or(Ok(PriorityFee::default()), PriorityFee::new)?;
        let bet = Bet::new(input.bet)?;
        let stop_loss_orders: Vec<StopLossOrder> = match input.orders {
            Some(orders_input) => match orders_input.stop_loss {
                Some(stop_loss_orders_input) => {
                    let mut sl_orders = Vec::with_capacity(stop_loss_orders_input.len());

                    for stop_loss in stop_loss_orders_input {
                        sl_orders.push(match stop_loss {
                            StopLossConfigInput::Fixed(params) => StopLossOrder::Fixed {
                                floor: params.floor,
                            },
                            StopLossConfigInput::Trailing(params) => StopLossOrder::Trailing {
                                max_drop: params.drop,
                            },
                            StopLossConfigInput::Performance(params) => {
                                StopLossOrder::Performance {
                                    profit: params.profit,
                                }
                            }
                            StopLossConfigInput::Time(params) => StopLossOrder::TimeBased {
                                duration: params.duration,
                            },
                        });
                    }
                    sl_orders
                }
                None => vec![],
            },
            None => vec![],
        };

        Ok(Self {
            bet,
            priority_fee,
            slippage_tolerance,
            stop_loss_orders,
        })
    }
}
