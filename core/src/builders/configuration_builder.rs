use std::{fs::File, io::Read};

use serde::Deserialize;
use thiserror::Error;

use crate::{entities::configuration::Configuration, traits::builder::Builder};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationFileInput {
    pub token: TokenInfoInput,
    pub bot: BotConfigInput,
    pub node: NodeConfigInput,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TokenInfoInput {
    pub name: String,
    pub symbol: String,
    pub address: String,
    pub community: Option<CommunityInput>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommunityInput {
    pub reddit: Option<String>,
    pub discord: Option<String>,
    pub telegram: Option<String>,
    pub twitter: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NodeConfigInput {
    pub provider: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BotConfigInput {
    pub slippage: Option<f64>,
    pub gas: Option<f64>,
    pub orders: Option<BotOrdersConfigInput>,
    pub metrics: Option<Vec<MetricsConfigInput>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BotOrdersConfigInput {
    pub stop_loss: Option<Vec<StopLossConfigInput>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StopLossConfigInput {
    #[serde(rename = "type")]
    pub order_type: String,
    pub period: Option<u64>,
    pub value: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MetricsConfigInput {
    pub name: String,
    pub period: Option<u64>,
}

pub struct ConfigurationBuilder {
    config_file: Option<String>,
}

impl ConfigurationBuilder {
    pub fn from_file(config_file: impl ToString) -> Self {
        Self {
            config_file: Some(config_file.to_string()),
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigurationBuilderError {
    #[error("File error: {0}")]
    FileError(String),
    #[error("Parsing Error: {0}")]
    ParsingError(String),
}

impl Builder<Configuration> for ConfigurationBuilder {
    type Error = ConfigurationBuilderError;

    fn build(self) -> Result<Configuration, Self::Error> {
        let mut content: String = "".to_string();
        let file_path = self.config_file.unwrap_or("./config.yaml".to_string());

        let _ = File::open(file_path)
            .map_err(|err| ConfigurationBuilderError::FileError(err.to_string()))?
            .read_to_string(&mut content);

        // Try Deserialization
        let yaml: ConfigurationFileInput = serde_yaml::from_str(&content)
            .map_err(|err| ConfigurationBuilderError::ParsingError(err.to_string()))?;

        Ok(yaml
            .try_into()
            .map_err(|err: &str| ConfigurationBuilderError::ParsingError(err.to_string()))?)
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::builder::Builder;

    use super::ConfigurationBuilder;

    #[test]
    fn test_build_configuration() {
        let config_file = "./tests/config/config1.test.yaml";
        let config = ConfigurationBuilder::from_file(config_file);

        let result = config.build();

        assert!(result.is_ok())
    }
}
