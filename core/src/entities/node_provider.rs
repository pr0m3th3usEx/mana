use std::default;

#[derive(Debug, Default)]
pub enum NodeProvider {
    #[default]
    Mainnet,
    QuickNode,
    GetBlock,
    Chainstack,
}
