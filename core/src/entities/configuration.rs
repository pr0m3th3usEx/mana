use crate::{
    builders::configuration_builder::inputs::ConfigurationFileInput,
    formulas::{bollinger::Bollinger, ema::EMA, rsi::RSI, sma::SMA},
    value_objects::configuration::{
        bot_settings::BotSettings, community::Community, node::Node, token_info::TokenInfo,
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
    node: Node,
    bot_settings: BotSettings,
}

impl TryFrom<ConfigurationFileInput> for Configuration {
    type Error = &'static str;

    fn try_from(input: ConfigurationFileInput) -> Result<Self, Self::Error> {
        let (token_info, community): (TokenInfo, Community) = input.token.try_into()?;
        let node = Node::try_from(input.node)?;
        let bot_settings = BotSettings::try_from(input.bot)?;

        Ok(Self {
            token_info,
            community,
            node,
            bot_settings,
        })
    }
}
