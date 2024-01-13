use serde::{Serialize, Deserialize};

// candle example:
// [
//     1705153800000,
//     "42706.01000000",
//     "42730.00000000",
//     "42686.51000000",
//     "42693.11000000",
//     "27.57985000",
//     1705153859999,
//     "1177738.32841250",
//     1726,
//     "12.30648000",
//     "525533.62556260",
//     "0"
//   ],

#[derive(Debug, Serialize, Deserialize)]
pub struct Candle {
    pub open_time: u64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub close_time: u64,
    pub base_asset_volume: String,
    pub number_of_trades: u32,
    pub taker_buy_volume: String,
    pub taker_buy_base_asset_volume: String,
    pub ignore: String,
}