use iced::{Application, Command, Element, Settings, Theme};
use std::sync::Arc;
use tokio::sync::Mutex;

mod bluetooth;
mod driver_db;
mod ui;
mod error;
mod service;

use crate::bluetooth::manager::BluetoothManager;
use crate::ui::MainView;

fn main() -> iced::Result {
    let manager = Arc::new(Mutex::new(BluetoothManager::new()));
    MainView::run(Settings::with_flags(State { manager }))
}

struct State {
    manager: Arc<Mutex<BluetoothManager>>,
}

impl Application for MainView {
    type Executor = iced::executor::Default;
    type Message = ui::Message;
    type Theme = Theme;
    type Flags = State;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self {
            devices: Vec::new(),
            progress: 0.0,
            manager: flags.manager,
            service: service::DriverService::new(),
        }, Command::none())
    }

    fn title(&self) -> String {
        "Universal Bluetooth Driver".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            ui::Message::ScanDevices => {
                let manager = self.manager.clone();
                Command::perform(async move {
                    let mut mgr = manager.lock().await;
                    mgr.scan_devices().await
                }, ui::Message::DevicesScanned)
            }
            ui::Message::DevicesScanned(result) => {
                if let Ok(devices) = result {
                    self.devices = devices;
                }
                Command::none()
            }
            ui::Message::InstallDrivers => {
                let service = self.service.clone();
                let devices = self.devices.clone();
                Command::perform(async move {
                    service.install_all(devices).await
                }, ui::Message::InstallationProgress)
            }
            ui::Message::InstallationProgress(progress) => {
                self.progress = progress;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        ui::views::main_screen(&self.devices, self.progress)
    }
}