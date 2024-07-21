use serde::Deserialize;
use solana_sdk::transaction::{Transaction, VersionedTransaction};

#[derive(Deserialize)]
pub struct TradeTransactionResponse {
    pub transaction: String,
}

// Convert response into unsigned transaction
impl TryInto<Transaction> for TradeTransactionResponse {
    type Error = String;

    fn try_into(self) -> Result<Transaction, Self::Error> {
        let tx_bytes = bs58::decode(self.transaction)
            .into_vec()
            .map_err(|err| err.to_string())?;
        let versioned_tx: VersionedTransaction =
            bincode::deserialize(&tx_bytes).map_err(|err| err.to_string())?;

        versioned_tx
            .into_legacy_transaction()
            .ok_or("Could not convert into legacy transaction".to_string())
    }
}
