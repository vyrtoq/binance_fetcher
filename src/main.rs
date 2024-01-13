mod settings;
mod candle;
use env_logger::Builder;
use log;
use tokio; // Add missing import for tokio crate
use binance_spot_connector_rust::{
    hyper::{BinanceHttpClient, Error},
    market::{self, klines::KlineInterval}
};
use redis::{Client, Commands};
use settings::Settings;
use candle::Candle;
use serde_json;

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

    // Connect to Redis, using .clone() because Redis does not support opening a connection on a &String
    // .clone creates a new String (instead of &String) object with the same value as the original
    let redis_client = Client::open(settings.redis_connection_string.clone()).expect("Failed to connect to Redis");
    let mut redis_con = redis_client.get_connection().expect("Failed to connect to Redis server");
    
    // Store the candles in Redis with the open_time as key
    for candle in candles {
        let candle_json = serde_json::to_string(&candle).expect("Failed to serialize candle");
        // set expects 3 parameters: key, value, and return value. 
        // key is u64, value (candle_json) is String, return value is ()
        redis_con.set::<u64, String, ()>(candle.open_time, candle_json).expect("Failed to store candle in Redis");
    }

    Ok(())
}