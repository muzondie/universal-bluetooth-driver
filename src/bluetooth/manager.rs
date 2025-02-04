use super::{BluetoothDevice, DeviceType, DriverStatus};
use windows::Devices::Bluetooth::{
    BluetoothDevice as WinDevice,
    BluetoothLEDevice,
    BluetoothConnectionStatus,
};

pub struct BluetoothManager {
    watcher: Option<windows::Devices::Bluetooth::BluetoothLEAdvertisementWatcher>,
}

impl BluetoothManager {
    pub fn new() -> Self {
        Self { watcher: None }
    }

    pub async fn scan_devices(&mut self) -> Result<Vec<BluetoothDevice>, crate::error::Error> {
        let mut devices = Vec::new();
        let selector = WinDevice::GetDeviceSelector()?;
        let devices_found = WinDevice::FindAllAsync(selector)?.await?;

        for device in devices_found {
            let id = device.DeviceId()?.to_string();
            let name = device.Name()?.to_string();
            let device_type = match device.ClassOfDevice()?.MajorClass()? {
                0x04 => DeviceType::Headset,
                0x05 => DeviceType::Keyboard,
                0x04 => DeviceType::Speaker,
                _ => DeviceType::Other,
            };

            devices.push(BluetoothDevice {
                id,
                name,
                device_type,
                driver_status: DriverStatus::Missing,
            });
        }

        Ok(devices)
    }
}