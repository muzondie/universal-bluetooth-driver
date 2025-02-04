use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Bluetooth device discovery failed")]
    DeviceDiscovery(#[from] windows::core::Error),
    #[error("Driver download failed")]
    DriverDownload(#[from] reqwest::Error),
    #[error("Filesystem error")]
    Io(#[from] std::io::Error),
    #[error("Installation service error")]
    ServiceError(String),
}