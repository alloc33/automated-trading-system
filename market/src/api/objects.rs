use apca::api::v2::{account::Account as AlpacaAccount, asset::Asset as AlpacaAsset};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub enum Account {
    AlpacaAccount(AlpacaAccount),
}

#[derive(Debug, Deserialize)]
pub enum AssetClass {
    Crypto(Option<String>),
    Stock(Option<String>)
}

#[derive(Debug, Serialize)]
pub enum Asset {
    AlpacaAsset(AlpacaAsset),
}
