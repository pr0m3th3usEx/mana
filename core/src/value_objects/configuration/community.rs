use std::collections::HashMap;

use url::Url;

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
