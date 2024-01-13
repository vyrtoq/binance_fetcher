use env_logger::Builder;
use log;
use tokio; // Add missing import for tokio crate
use binance_spot_connector_rust::{
    hyper::{BinanceHttpClient, Error},
    market::{self, klines::KlineInterval}
};
use redis::{Client, Commands};

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
    log::info!("Data was fetched correctly from the API");

    let redis_client = Client::open("redis://127.0.0.1:6379").expect("Failed to connect to Redis"); // Connect to Redis server on localhost at port 6379
    let mut redis_con = redis_client.get_connection().expect("Failed to connect to Redis server");
    
    // Store the data into Redis with "BTCUSDT" as the key
    match redis_con.set::<_, _, ()>("BTCUSDT", &data) {
        Ok(_) => log::info!("Data stored successfully"),
        Err(e) => log::error!("Failed to store data: {}", e),
    }
    Ok(())
}