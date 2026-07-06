# Decker - Xbox Controller → Okular Control

A custom Rust daemon that reads Xbox controller input and translates it into keyboard shortcuts or system input events to control [Okular](https://okular.kde.org/) and other PDF viewers.

## Overview

Decker allows you to control document viewers using an Xbox/gamepad controller. Perfect for hands-free document navigation while presenting or reading.

### Features

- 🎮 Full Xbox controller support via `gilrs`
- ⚡ Low-latency input injection using `xdotool`
- 🔧 Fully configurable button mappings via TOML
- 📁 XDG-compliant configuration storage
- 🏃 Multi-CPU architecture optimization
- 🔄 Automatic gamepad reconnection on disconnect
- 📊 Comprehensive logging with `env_logger`

## Architecture

```
Xbox Controller
      ↓
Rust input daemon (gilrs)
      ↓
Action mapping layer
      ↓
Input injection (xdotool)
      ↓
Okular/PDF Viewer actions (page nav, zoom, scroll)
```

## Prerequisites

### Linux

```bash
# Install xdotool for input injection
sudo apt install xdotool

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### macOS

```bash
# Install Homebrew dependencies
brew install xdotool

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Installation

```bash
# Clone the repository
git clone https://github.com/itd22/decker.git
cd decker

# Build with CPU optimizations
./build.sh znver3     # For Ryzen 5000+ or EPYC 7003+
./build.sh avx2       # For Intel Skylake+ or Ryzen 1000+
./build.sh generic    # Maximum compatibility
./build.sh all        # Build all variants

# The binary will be in ./builds/
```

## Configuration

### Default Config Location

Configuration is stored in `~/.config/decker/config.toml`

### Default Button Mappings

| Xbox Button | Action | Keyboard |
|-------------|--------|----------|
| D-Pad Up    | Scroll up | Up |
| D-Pad Down  | Scroll down | Down |
| RB (Right Bumper) | Next page | Right |
| LB (Left Bumper) | Previous page | Left |
| A           | Zoom in | Ctrl+Plus |
| B           | Zoom out | Ctrl+Minus |
| Start       | Toggle fullscreen | F11 |

### Custom Configuration

Edit `~/.config/decker/config.toml`:

```toml
controlled_program = "okular"
deadzone = 0.15

[button_mappings.DPadUp]
button = "DPadUp"
action = "scroll_up"
keys = ["Up"]

[button_mappings.RB]
button = "RB"
action = "next_page"
keys = ["Right"]

[button_mappings.A]
button = "A"
action = "zoom_in"
keys = ["ctrl", "plus"]

[button_mappings.LT]
button = "LT"
action = "custom"
keys = ["Page_Down"]
```

## Usage

### Running the Daemon

```bash
# Basic usage
./builds/decker-znver3

# With logging
RUST_LOG=info ./builds/decker-znver3
RUST_LOG=debug ./builds/decker-znver3

# Background operation
./builds/decker-znver3 &
```

### Supported Actions

- `scroll_up` / `scroll_down` - Scroll document
- `next_page` / `previous_page` - Navigate pages
- `zoom_in` / `zoom_out` / `zoom_reset` - Zoom controls
- `toggle_fullscreen` - Toggle fullscreen mode
- Custom key sequences via `keys` array

### Systemd Service (Optional)

Create `~/.config/systemd/user/decker.service`:

```ini
[Unit]
Description=Decker - Xbox Controller Okular Control
After=display-manager.service

[Service]
Type=simple
ExecStart=%h/path/to/decker-znver3
Restart=always
RestartSec=5
Environment="RUST_LOG=info"

[Install]
WantedBy=default.target
```

Then:
```bash
systemctl --user daemon-reload
systemctl --user enable --now decker
journalctl --user -u decker -f  # View logs
```

## Build Options

### CPU Targets

```bash
./build.sh znver3      # AMD Ryzen 5000+, EPYC 7003+
./build.sh znver2      # AMD Ryzen 3000+, EPYC 7002+
./build.sh avx2        # Intel Skylake+, AMD Ryzen 1000+
./build.sh v3          # x86-64-v3 (generic newer SIMD)
./build.sh generic     # x86-64 baseline (all systems)
```

### Build Configuration

The `build.sh` script automatically:

- Enables LTO (Link-Time Optimization)
- Sets optimal codegen units
- Strips debug symbols
- Applies CPU-specific optimizations

### Environment Variables

```bash
RUSTFLAGS="-C opt-level=3" ./build.sh znver3
CARGO_BUILD_JOBS=8 ./build.sh all
```

## Troubleshooting

### No Gamepad Detected

```bash
# Check if gamepad is recognized
cat /proc/bus/input/devices

# Test gamepad with evtest
sudo apt install evtest
sudo evtest
```

### Input Injection Not Working

```bash
# Verify xdotool installation
which xdotool
xdotool version

# Check if xdotool can inject keys
xdotool key Right  # Should trigger Right arrow
```

### High CPU Usage

- Reduce polling frequency in code (adjust sleep duration)
- Increase deadzone threshold in config
- Check for stuck buttons or analog drift

### Permission Issues

The daemon needs access to `/dev/input/` devices. Either:

```bash
# Run with sudo (not recommended)
sudo ./builds/decker-znver3

# Or add user to input group (recommended)
sudo usermod -a -G input $USER
# Log out and back in, then:
./builds/decker-znver3
```

## Project Structure

```
decker/
├── src/
│   ├── main.rs        # Entry point and event loop
│   ├── config.rs      # Configuration management
│   ├── controller.rs  # Xbox controller input handling
│   ├── action.rs      # Action mapping and processing
│   └── input.rs       # Input injection (xdotool)
├── Cargo.toml         # Dependencies and metadata
├── build.sh           # Multi-target build script
└── README.md          # This file
```

## Dependencies

- `gilrs` - Xbox/gamepad input
- `xdotool` - Keyboard event injection
- `tokio` - Async runtime
- `serde` / `serde_json` - Serialization
- `toml` - Config file format
- `dirs` - XDG directory support
- `log` / `env_logger` - Logging

## Performance

Decker is optimized for low latency:

- Event-driven polling with 10ms sleep
- Configurable deadzone to prevent jitter
- Direct xdotool injection (no D-Bus overhead)
- LTO and CPU-specific optimizations

Typical latency: **<50ms** from button press to event injection

## Future Enhancements

- [ ] Alternative input backends (evdev, uinput)
- [ ] D-Bus integration for KDE/GNOME
- [ ] Analog stick support for precise control
- [ ] Trigger pressure sensitivity
- [ ] Per-application button profiles
- [ ] GUI configuration tool
- [ ] Hot-reload configuration
- [ ] macOS and Windows support

## License

MIT / Open Source

## Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## Support

For issues, questions, or feature requests:
- Open an [GitHub Issue](https://github.com/itd22/decker/issues)
- Check existing issues for solutions

---

**Note:** Decker is currently optimized for Linux. macOS and Windows support requires additional input backend implementations.
