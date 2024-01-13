use env_logger::Builder;
use binance_spot_connector_rust::{
    http::Credentials,
    hyper::{BinanceHttpClient, Error},
    market::{self, klines::KlineInterval},
    trade
};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let credentials = Credentials::from_hmac("api-key".to_owned(), "api-secret".to_owned());
    let client = BinanceHttpClient::default().credentials(credentials);

    // Get candlesticks for BTCUSDT with a 1 minute interval
    let data = client.send(market::klines("BTCUSDT", KlineInterval::Minutes1)).await
        .expect("Request failed")
        .into_body_str().await
        .expect("Failed to read response body");
    log::info!("{}", data);

    // Get the last 10 candlesticks for BTCUSDT with a 1 hour interval
    let data = client.send(market::klines("BTCUSDT", KlineInterval::Hours1).limit(10)).await
        .expect("Request failed")
        .into_body_str().await
        .expect("Failed to read response body");
    log::info!("{}", data);

    // Get account information
    let data = client.send(trade::account()).await
        .expect("Request failed")
        .into_body_str().await
        .expect("Failed to read response body");
    log::info!("{}", data);

    Ok(())
}