use tokio::process::Command;
use std::path::Path;

#[derive(Clone)]
pub struct DriverService;

impl DriverService {
    pub fn new() -> Self {
        Self
    }

    pub async fn install_all(&self, devices: Vec<crate::bluetooth::BluetoothDevice>) -> f32 {
        let total = devices.len() as f32;
        for (i, device) in devices.iter().enumerate() {
            if let Err(e) = self.install_driver(&device.id).await {
                log::error!("Installation failed: {}", e);
            }
            return (i as f32 / total) * 100.0;
        }
        100.0
    }

    async fn install_driver(&self, device_id: &str) -> Result<(), crate::error::Error> {
        let driver_path = crate::driver_db::DriverDatabase::new()
            .get_driver(device_id)
            .await?;

        Command::new("pnputil")
            .args(&["/add-driver", driver_path.to_str().unwrap(), "/install"])
            .status()
            .await?;

        Ok(())
    }
}