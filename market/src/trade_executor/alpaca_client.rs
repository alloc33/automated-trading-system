use apca;

use super::TradeManager;

pub struct AlpacaClient {
    pub client: apca::Client,
}

impl TradeManager for AlpacaClient {
    fn get_account(&self) -> Result<(), ()> {
        Ok(())
    }
}
