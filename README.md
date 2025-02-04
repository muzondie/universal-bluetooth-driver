# Universal Bluetooth Driver  

A Rust-based Windows application that simplifies Bluetooth device setup. It scans your device, identifies missing drivers, and installs them automatically, no technical skills required.  

## Download  
1. Go to the [Releases](https://github.com/muzondie/universal-bluetooth-driver/releases) tab.  
2. Download the latest `.zip` file.  
3. Unzip the file and run `UniversalBluetoothDriver.exe`.  

## Usage  
1. **Run the app** after unzipping (allow permissions if prompted).  
2. Connect your Bluetooth device to the computer.  
3. The app will detect the device and install drivers automatically.  
4. A status bar shows progress; restart your device if needed.  

## Features  
- Works with all Bluetooth devices (headsets, keyboards, mice, speakers, gamepads, etc).  
- Auto-detects new devices and missing drivers.  
- Installs drivers silently without user input.  
- Lightweight GUI with progress tracking.  
- Supports Windows 10 and 11 (64-bit).  
- Open-source code for transparency.  
- Regular driver database updates.  
- Blocks unsafe or untrusted drivers.  
- Low system resource usage.  

## Build from Source  
1. Install [Rust](https://www.rust-lang.org/tools/install).  
2. Clone the repository:  
   ```bash  
   git clone https://github.com/muzondie/universal-bluetooth-driver.git  
   ```  
3. Build the project:  
   ```bash  
   cd universal-bluetooth-driver  
   cargo build --release  
   ```  
4. Find the executable in `target/release/`.  

## Contributing  
Contributions are currently closed due to limited maintenance capacity.

## License  
MIT License. Free to use, modify, and distribute. See [LICENSE](LICENSE) for details.