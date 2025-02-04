use reqwest::Client;
use ring::digest::{digest, SHA256};
use std::path::PathBuf;

pub struct DriverDatabase {
    client: Client,
    base_url: String,
}

impl DriverDatabase {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.driverdb.com/api".into(),
        }
    }

    pub async fn get_driver(&self, device_id: &str) -> Result<PathBuf, crate::error::Error> {
        let response = self.client
            .get(&format!("{}/drivers/{}", self.base_url, device_id))
            .send()
            .await?;

        let bytes = response.bytes().await?;
        let hash = digest(&SHA256, &bytes);
        
        let path = std::env::temp_dir().join(format!("{}.inf", device_id));
        tokio::fs::write(&path, bytes).await?;
        
        Ok(path)
    }
}