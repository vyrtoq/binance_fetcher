use redis::{Client, Commands};
use crate::candle::Candle;
use serde_json;

pub fn store_candles(redis_connection_string: String, candles: Vec<Candle>) -> redis::RedisResult<()> {
    let client = Client::open(redis_connection_string).expect("Failed to create Redis client");
    let mut connection = client.get_connection().expect("Failed to connect to Redis server");

    // Store the candles in Redis with the open_time as key
    for candle in candles {
        let candle_json = serde_json::to_string(&candle).expect("Failed to serialize candle");
        // set expects 3 parameters: key, value, and return value. 
        // key is u64, value (candle_json) is String, return value is ()
        connection.set::<u64, String, ()>(candle.open_time, candle_json).expect("Failed to store candle in Redis");
    }
    
    log::info!("Candles stored in Redis");
    Ok(())
}