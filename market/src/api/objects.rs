use apca::api::v2::account::Account as AlpacaAccount;

pub enum Account {
    AlpacaAccount(AlpacaAccount),
}
