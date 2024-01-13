# Binance Spot Connector in Rust

This project is a Rust application that fetches candlestick data from the Binance Spot API and stores it in a Redis database. The application specifically fetches data for the BTCUSDT pair with a 1-minute interval.

## Dependencies

- Rust
- Redis
- Binance Spot API

## Setup

1. Install Rust: Follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).
2. Install Redis: Follow the instructions on the [official Redis website](https://redis.io/download).

## Running the Application

1. Start your Redis server.
2. Run the application with the following command:

```bash
cargo run