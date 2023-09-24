use apca::api::v2::account::Account as AlpacaAccount;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Account {
    AlpacaAccount(AlpacaAccount),
}
