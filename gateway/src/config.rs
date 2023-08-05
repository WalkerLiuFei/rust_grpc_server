

use serde_derive::{Deserialize, Serialize};
use toml;

#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(default = "localhost")]
    Host: String,
    //#[serde(default = "3001")]
    ListenOn: u32,
}


