use std::{fs::File, io::Read};

use inputs::ConfigurationFileInput;
use thiserror::Error;

pub mod inputs;

use crate::{entities::configuration::Configuration, traits::builder::Builder};

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
