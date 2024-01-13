use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub redis_connection_string: String,
}