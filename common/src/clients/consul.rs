use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use tracing::info;

pub struct ConsulClient {
    client: Client,
    base_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceInfo {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "Port")]
    pub port: u32,
    
}

impl Into<String> for ServiceInfo {
    fn into(self) -> String{
        serde_json::to_string(&self).unwrap_or("".to_string())
    }
}

impl ConsulClient{
    pub fn new(base_url: String)->Self{
        return ConsulClient{
            client: Client::new(),
            base_url,
        }
    }
    pub async fn register(&self, service_info: &ServiceInfo) -> Result<(), reqwest::Error>{
        let url = format!("{}/v1/agent/service/register", self.base_url);
        let info_str = serde_json::to_string(&service_info).unwrap_or_else(|_| "".to_string());

        let res = self.client.put(&url).body(info_str).send().await?;
        info!("register service {} to consul, response: {:?}", service_info.name, res);
        Ok(())
    }
}



