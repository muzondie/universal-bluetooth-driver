use iced::{Element, Length, Row, Space};
use super::Message;

pub fn main_screen(devices: &[crate::bluetooth::BluetoothDevice], progress: f32) -> Element<Message> {
    Column::new()
        .push(Text::new("Detected Bluetooth Devices").size(20))
        .push(Space::new(Length::Shrink, Length::Units(20)))
        .push(device_list(devices))
        .push(ProgressBar::new(0.0..=100.0, progress))
        .push(install_button())
        .into()
}

fn device_list(devices: &[crate::bluetooth::BluetoothDevice]) -> Element<Message> {
    let mut col = Column::new().spacing(10);
    for device in devices {
        col = col.push(device_row(device));
    }
    col.into()
}

fn device_row(device: &crate::bluetooth::BluetoothDevice) -> Element<Message> {
    Row::new()
        .push(Text::new(&device.name).width(Length::Units(150)))
        .push(Text::new(format!("{:?}", device.device_type)).width(Length::Units(100)))
        .into()
}

fn install_button() -> Element<Message> {
    Button::new(Text::new("Install Drivers"))
        .on_press(Message::InstallDrivers)
        .into()
}