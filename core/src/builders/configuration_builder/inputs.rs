use serde::Deserialize;

use crate::value_objects::{
    configuration::{community::Community, token_info::TokenInfo},
    token::{token_address::TokenAddress, token_name::TokenName, token_symbol::TokenSymbol},
};

/**

TOKEN CONFIGURATION

**/

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommunityInput {
    pub reddit: Option<String>,
    pub discord: Option<String>,
    pub telegram: Option<String>,
    pub twitter: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfoInput {
    pub name: String,
    pub symbol: String,
    pub address: String,
    pub decimals: u8,
    pub community: Option<CommunityInput>,
}

impl TryInto<(TokenInfo, Community)> for TokenInfoInput {
    type Error = &'static str;

    fn try_into(self) -> Result<(TokenInfo, Community), Self::Error> {
        let token_name = TokenName::new(self.name)?;
        let token_symbol = TokenSymbol::new(self.symbol)?;
        let token_address = TokenAddress::new(self.address.as_str())?;

        let token_info = TokenInfo::new(token_name, token_address, token_symbol, self.decimals);
        let community = Community::try_from(self.community.unwrap_or_default())?;

        Ok((token_info, community))
    }
}

/**

BOT CONFIGURATION

**/

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum StopLossConfigInput {
    Fixed(FixedStopLossInput),
    Trailing(TrailingStopLossInput),
    Performance(PerformanceStopLossInput),
    Time(TimeBasedStopLossInput),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedStopLossInput {
    pub floor: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrailingStopLossInput {
    pub drop: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceStopLossInput {
    pub profit: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeBasedStopLossInput {
    pub duration: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum MetricsConfigInput {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotOrdersConfigInput {
    pub stop_loss: Option<Vec<StopLossConfigInput>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotConfigInput {
    pub slippage: Option<f64>,
    pub priority_fee: Option<f64>,
    pub bet: f64,
    pub orders: Option<BotOrdersConfigInput>,
}

/**

NODE CONFIGURATION

**/

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConfigInput {
    pub provider: String,
    pub url: Option<String>,
}

/**

ROOT CONFIGURATION

**/

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationFileInput {
    pub token: TokenInfoInput,
    pub bot: BotConfigInput,
    pub node: NodeConfigInput,
}
