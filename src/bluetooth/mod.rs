pub mod manager;
pub mod device;

#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub driver_status: DriverStatus,
}

#[derive(Debug, Clone)]
pub enum DeviceType {
    Headset,
    Keyboard,
    Speaker,
    Other,
}

#[derive(Debug, Clone)]
pub enum DriverStatus {
    Installed,
    Missing,
    Outdated,
}