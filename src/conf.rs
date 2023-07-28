use miette::{miette, Report};

use config::{Config, ConfigError, File};

pub const KEY_SECRET: &'static str = "secret_key";
pub const KEY_EDGEDB_INSTANCE: &str = "edgedb_instance";
pub const KEY_PORT: &str = "port";
pub const DEFAULT_PORT: u16 = 3721;
pub const ALPHANUMERIC: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub fn get_config() -> Result<Config, ConfigError> {
    Config::builder().build()
}

pub fn get_listening_port(config: &Config) -> u16 {
    let port = config
        .get_int(KEY_PORT)
        .map_err(|e| miette!("Failed to get port: {e}"))
        .map(|p| p as u16);
    port.unwrap_or(DEFAULT_PORT)
}
