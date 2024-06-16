use crate::{
    builders::configuration_builder::ConfigurationFileInput,
    formulas::{bollinger::Bollinger, ema::EMA, rsi::RSI, sma::SMA},
    value_objects::configuration::{
        bot_settings::BotSettings, community::Community, node_info::NodeInfo, token_info::TokenInfo,
    },
};

#[derive(Debug)]
pub enum Metrics {
    EMA(EMA),
    RSI(RSI),
    SMA(SMA),
    Bollinger(Bollinger),
}

#[derive(Debug, Default)]
pub struct Configuration {
    token_info: TokenInfo, // URL
    community: Community,
    node: NodeInfo,
    bot_settings: BotSettings,
}

impl TryInto<Configuration> for ConfigurationFileInput {
    type Error = &'static str;

    fn try_into(self) -> Result<Configuration, Self::Error> {
        println!("{:#?}", self);

        Ok(Configuration::default())
    }
}
