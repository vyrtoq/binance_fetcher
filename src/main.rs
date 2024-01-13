use std::{fs, io};
use env_logger::Builder;
use log;
use tokio; // Add missing import for tokio crate
use binance_spot_connector_rust::{
    http::Credentials,
    hyper::{BinanceHttpClient, Error},
    market::{self, klines::KlineInterval},
    trade
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    // Read API key from file
    let api_key = fs::read_to_string("api_key.txt")
        .expect("Failed to read API key from file");

    // Read API secret from file
    let api_secret = fs::read_to_string("api_secret.txt")
        .expect("Failed to read API secret from file");

    // Create credentials from API key and secret
    let credentials = Credentials::from_hmac(api_key.to_owned(), api_secret.to_owned());

    // Create Binance HTTP client with credentials
    let client = BinanceHttpClient::default().credentials(credentials);

    // Get candlesticks for BTCUSDT with a 1 minute interval
    let data = client.send(market::klines("BTCUSDT", KlineInterval::Minutes1)).await
        .expect("Failed to send request")
        .into_body_str().await
        .expect("Failed to read response body");
    log::info!("Candlesticks for BTCUSDT with 1 minute interval: {}", data);

    // Get the last 10 candlesticks for BTCUSDT with a 1 hour interval
    let data = client.send(market::klines("BTCUSDT", KlineInterval::Hours1).limit(10)).await
        .expect("Failed to send request")
        .into_body_str().await
        .expect("Failed to read response body");
    log::info!("Last 10 candlesticks for BTCUSDT with 1 hour interval: {}", data);

    // Get account information
    let data = client.send(trade::account()).await
        .expect("Failed to send request")
        .into_body_str().await
        .expect("Failed to read response body");
    log::info!("Account information: {}", data);

    Ok(())
}