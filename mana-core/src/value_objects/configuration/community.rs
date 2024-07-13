use std::collections::HashMap;

use url::Url;

use crate::builders::configuration_builder::inputs::CommunityInput;

use super::social_network::SocialNetwork;

#[derive(Debug, Default)]
pub struct Community {
    urls: HashMap<SocialNetwork, Url>,
}

impl Community {
    pub fn new(urls: HashMap<SocialNetwork, Url>) -> Community {
        Self { urls }
    }

    pub fn urls(&self) -> &HashMap<SocialNetwork, Url> {
        &self.urls
    }
}

impl TryFrom<CommunityInput> for Community {
    type Error = &'static str;

    fn try_from(input: CommunityInput) -> Result<Self, Self::Error> {
        let mut sns = HashMap::new();

        if let Some(reddit) = input.reddit {
            let url = Url::parse(reddit.as_str()).map_err(|_| "Reddit URL: invalid URL")?;
            sns.insert(SocialNetwork::Reddit, url);
        };

        if let Some(twitter) = input.twitter {
            let url = Url::parse(twitter.as_str()).map_err(|_| "Twitter/X URL: invalid URL")?;
            sns.insert(SocialNetwork::Twitter, url);
        }

        if let Some(discord) = input.discord {
            let url = Url::parse(discord.as_str()).map_err(|_| "Discord URL: invalid URL")?;
            sns.insert(SocialNetwork::Discord, url);
        }

        if let Some(telegram) = input.telegram {
            let url = Url::parse(telegram.as_str()).map_err(|_| "Telegram URL: invalid URL")?;
            sns.insert(SocialNetwork::Telegram, url);
        }

        Ok(Self::new(sns))
    }
}
