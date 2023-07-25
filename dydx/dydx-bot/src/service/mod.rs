pub mod dydx;

use eyre::Result;

pub trait TradeBot {
    fn close_all_positions(&self) -> Result<(), String>;
}