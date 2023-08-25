use std::env;

use lazy_static::lazy_static;
use log::warn;
use serde_derive::Deserialize;

use common::utils;

#[derive(Debug, Deserialize)]
pub struct APPConfig {
    pub redis_config: RedisConfig,
    pub host: String,
    pub port: i32,
    pub name: String,
    pub log_level: String,
    pub jaeger_endpoint: Option<String>,
    pub consul_endpoint: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: APPConfig = {
        let cfg_file_path = match env::var("CONFIG_PATH") {
            Ok(config_path) => {
                config_path
            },
            Err(_) =>{
                warn!("CONFIG_PATH not set, use default config.toml");
                String::from("config.toml")
            },
        };
        utils::read_config_from_file::<APPConfig>(cfg_file_path.as_str()).expect("Failed to load the configuration.")
    };
}


#[cfg(test)]
mod tests {
    use lazy_static::initialize;

    use super::*;

    #[test]
    fn test_config() {
        let _ = initialize(&CONFIG);

        // Now you can access the CONFIG lazy-static variable in the test function.
        println!("{:?}", *CONFIG);
    }
}
