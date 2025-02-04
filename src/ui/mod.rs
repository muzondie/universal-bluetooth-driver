mod views;

use iced::{Column, Text, ProgressBar, Button};
use super::bluetooth;

#[derive(Debug, Clone)]
pub enum Message {
    ScanDevices,
    DevicesScanned(Result<Vec<bluetooth::BluetoothDevice>, crate::error::Error>),
    InstallDrivers,
    InstallationProgress(f32),
}

pub struct MainView {
    devices: Vec<bluetooth::BluetoothDevice>,
    progress: f32,
    manager: std::sync::Arc<tokio::sync::Mutex<bluetooth::manager::BluetoothManager>>,
    service: crate::service::DriverService,
}