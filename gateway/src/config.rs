use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use toml;

#[derive(Serialize, Deserialize)]
pub struct Config {
    // the host that the gateway listens on
    pub(crate) host: String,
    // the port that the gateway listens on
    //#[serde(default = "3001")]
    pub(crate) listen_on: u32,

    // MySQL DB configuration
    pub(crate) db_config: DBConfig,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DBConfig {
    pub(crate) url: String,
    pub(crate) port: u32,
    pub(crate) user: String,
    pub(crate) password: String,
    pub(crate) database: String,
}

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = {
        common::utils::read_config_from_file("config.toml").unwrap()
    };
}