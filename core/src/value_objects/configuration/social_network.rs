use std::str::FromStr;

use thiserror::Error;

#[derive(Debug)]
pub enum SocialNetwork {
    Discord,
    Twitter,
    Reddit,
    Telegram,
}

#[derive(Debug, Error)]
pub enum SocialNetworkError {
    #[error("Unknown social network: {0}")]
    Unknown(String),
}

impl FromStr for SocialNetwork {
    type Err = SocialNetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "discord" => Ok(Self::Discord),
            "twitter" | "x" => Ok(Self::Twitter),
            "reddit" => Ok(Self::Reddit),
            "telegram" => Ok(Self::Telegram),
            other => Err(SocialNetworkError::Unknown(other.to_string())),
        }
    }
}
