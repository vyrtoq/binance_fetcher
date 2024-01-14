mod settings;
mod candle;
mod redis_store;

use env_logger::Builder;
use log;
use tokio; 
use binance_spot_connector_rust::{
    hyper::{BinanceHttpClient, Error},
    market::{self, klines::KlineInterval}
};

// own modules
use settings::Settings;
use candle::Candle;
use serde_json;
use redis_store::store_candles;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let config = config::Config::builder()
        .add_source(config::File::with_name("Settings.toml"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let settings: Settings = config.try_deserialize().unwrap();    
    let client = BinanceHttpClient::default(); // without credentials because of bad header bug
    
    // Get candlesticks for BTCUSDT with a 1 minute interval
    let data = client.send(market::klines("BTCUSDT", KlineInterval::Minutes1)).await
        .expect("Request failed")
        .into_body_str().await
        .expect("Failed to read response body");
    log::info!("Data was fetched correctly from the API");

    // Convert the data into a Vec<Candle>
    let candles: Vec<Candle> = serde_json::from_str(&data).expect("Failed to parse data");

    store_candles(settings.redis_connection_string, candles).expect("Failed processing candles storage");
    
    Ok(())
}