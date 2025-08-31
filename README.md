# BLE Heart Beat Monitor

A desktop application that connects to Bluetooth Low Energy (BLE) heart rate monitors and provides real-time heart rate data for use in streaming applications like OBS Studio.

![Heart Rate Monitor Interface](https://img.shields.io/badge/Platform-Windows-blue)
![Built with Tauri](https://img.shields.io/badge/Built%20with-Tauri-orange)
![Frontend](https://img.shields.io/badge/Frontend-Nuxt.js-green)

## ✨ Features

- **Real-time BLE Connection**: Connect to any standard BLE heart rate monitor device
- **Live Heart Rate Display**: View current heart rate and battery level in a clean interface
- **HTTP API Server**: Built-in web server providing heart rate data at `http://localhost:25872`
- **OBS Integration Ready**: Perfect for streamers who want to display heart rate during gameplay or fitness streams
- **Auto-disconnect Detection**: Automatically detects when devices disconnect (data expires after 30 seconds)
- **Battery Level Monitoring**: Shows the battery level of your heart rate monitor
- **Cross-device Compatibility**: Works with any BLE heart rate monitor following the standard Heart Rate Service specification

## ⚠️ Limitations

- **Windows Only**: This application currently only works on Windows due to Tauri's webview
- **BLE Device Compatibility**: Only works with BLE heart rate monitors that implement the standard Heart Rate Service (0x180D)

## 🚀 Quick Start

### Prerequisites

- Windows 10/11
- Node.js (v18 or higher)
- pnpm package manager
- Rust toolchain (for building from source)

### Installation

#### Option 1: Download Pre-built Binary

1. Go to the [Releases](../../releases) page
2. Download the latest `.exe` file for Windows
3. Run the installer and follow the setup wizard

#### Option 2: Build from Source

```powershell
# Clone the repository
git clone https://github.com/Teages/ble-heart-beat.git
cd ble-heart-beat

# Install dependencies
pnpm install

# Build the application
pnpm tauri build
```

The built executable will be available in `src-tauri/target/release/`.

## 📺 Using with OBS Studio

This application is perfect for streamers who want to display their heart rate during gaming, fitness, or other activities. Here's how to set it up:

### Method 1: Browser Source (Recommended)

1. Launch the BLE Heart Beat Monitor application
2. Connect your BLE heart rate monitor using the "Connect Device" button
3. In OBS Studio, add a **Browser Source**
4. Set the URL to: `http://localhost:25872`
5. Set your custom style
6. The heart rate will automatically update in real-time

### Method 2: Custom Overlay

You can create custom overlays by accessing the JSON API endpoint:

- **Endpoint**: `http://localhost:25872/api/heart`
- **Response**: `{"heart_beat": 75}` (or `null` if disconnected)

Use this endpoint with custom HTML/CSS overlays or third-party tools.

### Example CSS for Custom Styling

```css
body,
p {
  margin: 0;
}

#container {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 24px;
  border-radius: 9999px;
  background: #000000a0;
}
#container::before {
  content: ' ';
  width: 32px;
  height: 32px;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='24' height='24' viewBox='0 0 24 24' fill='%23fb2c36' stroke-linecap='round' stroke-linejoin='round' aria-hidden='true'%3E%3Cpath d='M2 9.5a5.5 5.5 0 0 1 9.591-3.676.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5l-5.492 5.313a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5'/%3E%3C/svg%3E");
  background-size: 32px 32px;
  background-position: center;
  background-repeat: no-repeat;
}

#count {
  flex: 1;
  color: #fff;
  font-size: 32px;
  line-height: 46px;
  font-family: sans-serif;
  text-align: center;
}

.connected #count::after {
  content: 'BPM';
  font-size: 20px;
  line-height: 46px;
  padding-left: 4px;
}

.disconnected#container::before {
  opacity: 0.25;
}
.disconnected #count,
.error #count {
  color: #000;
}

```

## 🔧 Usage Instructions

### Connecting Your Heart Rate Monitor

1. **Launch the Application**: Run the BLE Heart Beat Monitor
2. **Enable Bluetooth**: Make sure your heart rate monitor is in pairing mode
3. **Click "Connect Device"**: The app will scan for nearby BLE heart rate devices
4. **Select Your Device**: Choose your heart rate monitor from the list
5. **Start Monitoring**: Once connected, heart rate data will appear in real-time

### Supported Heart Rate Monitors

This application works with any BLE heart rate monitor that implements the standard Bluetooth Heart Rate Service, including:

- Polar H10, H9, H7
- Wahoo TICKR series
- Garmin HRM-Pro, HRM-Run
- Suunto Smart Sensor
- MyZone MZ-3
- And many others following the BLE HR standard

### Troubleshooting

**Connection Issues:**

- Ensure your heart rate monitor is in pairing mode
- Check that Bluetooth is enabled on your computer
- Try restarting the application
- Make sure no other applications are connected to the heart rate monitor

**Data Not Showing in OBS:**

- Verify the application is running
- Check that the heart rate monitor is connected (green signal icon should be visible)
- Ensure the Browser Source URL is exactly `http://localhost:25872`
- Try refreshing the browser source in OBS

**Browser Compatibility:**

- Use Chrome, Edge, or other Chromium-based browsers
- Enable experimental web features if needed
- Some browsers may require HTTPS for Web Bluetooth (not needed for localhost)

## 🛠️ Development

### Development Setup

```powershell
# Install dependencies
pnpm install

# Start development server
pnpm dev:tauri
```

### Project Structure

```
ble-heart-beat/
├── app/                          # Nuxt.js frontend
│   ├── components/               # Vue components
│   ├── composables/              # Vue composables for BLE and reporting
│   └── pages/                    # Application pages
├── src-tauri/                    # Tauri backend (Rust)
│   ├── src/
│   │   ├── lib.rs               # Main application logic & HTTP server
│   │   └── main.rs              # Entry point
│   └── tauri.conf.json          # Tauri configuration
└── package.json                 # Node.js dependencies
```

### Key Technologies

- **Frontend**: Nuxt.js, Vue 3, TailwindCSS, Nuxt UI
- **Backend**: Rust, Tauri, Hyper (HTTP server)
- **BLE Communication**: Web Bluetooth API
- **Build System**: Vite, Tauri CLI

## 📋 API Reference

### HTTP Endpoints

| Endpoint     | Method | Description                          | Response                         |
| ------------ | ------ | ------------------------------------ | -------------------------------- |
| `/`          | GET    | Web interface for heart rate display | HTML page                        |
| `/api/heart` | GET    | JSON heart rate data                 | `{"heart_beat": number \| null}` |

### Tauri Commands

- `update_heart_beat(heart_rate: number)`: Updates the heart rate value from the frontend
- `greet(name: string)`: Test command for basic functionality

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/new-feature`
3. Commit your changes: `git commit -am 'Add new feature'`
4. Push to the branch: `git push origin feature/new-feature`
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Tauri](https://tauri.app/) for cross-platform desktop development
- Uses [Nuxt.js](https://nuxt.com/) for the modern web interface
- Implements the Bluetooth Heart Rate Service specification
- Thanks to the open-source community for the amazing tools and libraries

---

**Happy Streaming! 💓**

For support, feature requests, or bug reports, please [open an issue](../../issues) on GitHub.
