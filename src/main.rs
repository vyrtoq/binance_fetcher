mod settings;
use env_logger::Builder;
use log;
use tokio; // Add missing import for tokio crate
use binance_spot_connector_rust::{
    hyper::{BinanceHttpClient, Error},
    market::{self, klines::KlineInterval}
};
use redis::{Client, Commands};
use settings::Settings;

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

    // Connect to Redis, using .clone() because Redis does not support opening a connection on a &String
    // .clone() creates a new String (instead of &String) object with the same value as the original
    let redis_client = Client::open(settings.redis_connection_string.clone()).expect("Failed to connect to Redis");
    let mut redis_con = redis_client.get_connection().expect("Failed to connect to Redis server");
    
    // Store the data into Redis with "BTCUSDT" as the key
    match redis_con.set::<_, _, ()>("BTCUSDT", &data) {
        Ok(_) => log::info!("Data stored successfully"),
        Err(e) => log::error!("Failed to store data: {}", e),
    }
    Ok(())
}