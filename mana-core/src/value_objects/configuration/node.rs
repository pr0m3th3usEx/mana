use std::str::FromStr;

use url::Url;

use crate::builders::configuration_builder::inputs::NodeConfigInput;

pub const MAINNET_URL: &str = "https://api.mainnet-beta.solana.com";

#[derive(Debug, PartialEq, Eq)]
pub enum NodeProvider {
    Mainnet,
    QuickNode,
    GetBlock,
    Chainstack,
    Helius,
}

impl FromStr for NodeProvider {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mainnet" | "mainnet-beta" => Ok(Self::Mainnet),
            "quicknode" => Ok(Self::QuickNode),
            "chainstack" => Ok(Self::Chainstack),
            "getblock" => Ok(Self::GetBlock),
            "helius" => Ok(Self::Helius),
            _ => Err("unknown provider"),
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Mainnet(Url),
    QuickNode(Url),
    GetBlock(Url),
    Chainstack(Url),
    Helius(Url),
}

impl Default for Node {
    fn default() -> Self {
        Self::Mainnet(Url::parse(MAINNET_URL).unwrap())
    }
}

impl TryFrom<NodeConfigInput> for Node {
    type Error = &'static str;

    fn try_from(input: NodeConfigInput) -> Result<Self, Self::Error> {
        let provider: NodeProvider = input.provider.parse()?;

        match provider {
            NodeProvider::Mainnet => Ok(Self::default()),
            other => {
                let Some(url) = input.url else {
                    return Err("NodeConfigInput: no URL define for custom node");
                };
                let url = Url::parse(url.as_str()).map_err(|_| "NodeConfigInput: Invalid URL")?;

                match other {
                    NodeProvider::QuickNode => Ok(Self::QuickNode(url)),
                    NodeProvider::GetBlock => Ok(Self::GetBlock(url)),
                    NodeProvider::Chainstack => Ok(Self::Chainstack(url)),
                    NodeProvider::Helius => Ok(Self::Helius(url)),
                    _ => panic!("NodeConfigInput: Impossible case"),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::NodeProvider;

    #[rstest]
    #[case("MainNet", Ok(NodeProvider::Mainnet))]
    #[case("Quicknode", Ok(NodeProvider::QuickNode))]
    #[case("Helius", Ok(NodeProvider::Helius))]
    #[case("Chainstack", Ok(NodeProvider::Chainstack))]
    #[case("Getblock", Ok(NodeProvider::GetBlock))]
    #[case("other", Err("unknown provider"))]
    fn test_parsing_node_provider(
        #[case] name: String,
        #[case] expected: Result<NodeProvider, &'static str>,
    ) {
        let result: Result<NodeProvider, &'static str> = name.parse();

        assert_eq!(result, expected);
    }
}
