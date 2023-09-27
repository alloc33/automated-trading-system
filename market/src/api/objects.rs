use apca::api::v2::{account::Account as AlpacaAccount, asset::Asset as AlpacaAsset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub enum Account {
    AlpacaAccount(AlpacaAccount),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AssetClass {
    #[serde(rename = "us_equity")]
    UsEquity,
    #[serde(rename = "crypto")]
    Crypto,
}

#[derive(Debug, Serialize)]
pub enum Asset {
    AlpacaAsset(AlpacaAsset),
}
