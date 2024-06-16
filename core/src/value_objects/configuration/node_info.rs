use url::Url;

use crate::entities::node_provider::NodeProvider;

#[derive(Debug)]
pub struct NodeInfo {
    provider: NodeProvider,
    url: Url,
}

impl Default for NodeInfo {
    fn default() -> Self {
        Self {
            provider: Default::default(),
            url: Url::parse("https://example.net").unwrap(),
        }
    }
}

impl NodeInfo {
    pub fn new(provider: NodeProvider, url: Url) -> Self {
        Self { provider, url }
    }
}
