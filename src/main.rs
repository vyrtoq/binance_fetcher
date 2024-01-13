use env_logger::Builder;
use log;
use tokio; // Add missing import for tokio crate
use binance_spot_connector_rust::{
    hyper::{BinanceHttpClient, Error},
    market::{self, klines::KlineInterval}
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    
    let client = BinanceHttpClient::default(); // without credentials because of bad header bug

    // Get candlesticks for BTCUSDT with a 1 minute interval
    let data = client.send(market::klines("BTCUSDT", KlineInterval::Minutes1)).await
        .expect("Request failed")
        .into_body_str().await
        .expect("Failed to read response body");
    log::info!("{}", data);

    let db = DB::open_default("rocks.db")?;

    // Store the data into the database with "BTCUSDT" as the key
    db.put("BTCUSDT", data)?;
    
    Ok(())
}