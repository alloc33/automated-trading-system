use std::sync::Arc;

use crate::App;

pub mod order;

pub struct TradeExecutor {
    pub app: Arc<App>,
}

impl TradeExecutor {
    pub fn new(app: Arc<App>) -> Self {
        Self { app }
    }
}
