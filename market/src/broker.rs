use serde::Deserialize;

use crate::trade_executor::TradeExecutor;

#[axum::async_trait]
pub trait BrokerClient {
    async fn get_account_info(&self, trade_executor: &TradeExecutor) -> Result<(), ()>;
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Broker {
    Alpaca,
}

// #[axum::async_trait]
// impl TradingClient for Broker {
//     async fn get_account_info(&self, trade_executor: &TradeExecutor) -> Result<(), ()> {
//         match self {
//             Broker::Alpaca => {
//                 // let account = trade_executor.alpaca_client.
//                 // println!("account: {:?}", account);

//                 // DO A SPECIFIC TO ALPACA REQUESTS TRHOUGH THE ALPACA CLIENT IN TRADE EXECUTOR

//                 Ok(())
//             }
//         }
//     }
// }
