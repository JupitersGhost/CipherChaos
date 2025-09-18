# CipherChaos

<div align="center">
  <img src="icon.png" alt="CipherChaos" width="200" height="200" style="border-radius: 50%;">
  
  **CipherChaos - Enhanced Entropy Generation & Post-Quantum Cryptography**
</div>

---

CipherChaos is an advanced entropy generation system combining ESP32-S3 hardware with host-side processing to create high-quality random keys. Features post-quantum cryptography integration, real-time entropy visualization, and personality-driven interaction through "CipherChaos" and "Cipher-chan" characters.

This system has been tested on Windows 11 with ESP32-S3 development boards and supports both classical and post-quantum key wrapping.

## Features

### Core Functionality
- **Multi-Source Entropy**: Keystroke timing, mouse movement, ESP32 TRNG, USB jitter
- **Classical Cryptography**: High-quality AES256 key generation with comprehensive entropy auditing
- **Real-time Visualization**: Live entropy monitoring with animated displays
- **Enhanced Auditing**: NIST-inspired entropy quality assessment by "Echo-tan"
- **Hardware Integration**: ESP32-S3 with WS2812 RGB LEDs and enhanced firmware
- **Network Support**: CobraMesh/Headscale integration for distributed entropy
- **Experimental PQC**: Kyber512 KEM and Falcon512 signatures (work in progress)

### ESP32-S3 Features (Cipher-chan Enhanced Firmware v2.1)
- **TRNG Streaming**: High-frequency true random number generation
- **WiFi Entropy**: Environmental RF noise collection
- **USB Jitter**: Timing-based entropy from USB communications
- **RGB Control**: Dynamic LED feedback with 16M+ colors
- **Personality System**: cipher-tan's cryptographic commentary
- **Status Monitoring**: Real-time hardware telemetry

### GUI Features
- **Modern Interface**: PySide6-based with CipherChaos aesthetic
- **Tray Integration**: Background operation with system tray
- **Configurable Settings**: Brightness, timing windows, entropy sources
- **Key Logging**: JSON-based key storage with metadata
- **Network Status**: Mesh connectivity and peer monitoring

## Setup

### Prerequisites
```bash
# Python 3.8+ required
pip install -r requirements.txt

# For experimental PQC support (optional):
# 1. Install Rust toolchain from https://rustup.rs/
# 2. Install maturin: pip install maturin
# 3. Build bindings: maturin develop
# Note: PQC features are experimental and not fully functional
```

### Hardware Setup
1. **ESP32-S3 Board**: Any ESP32-S3 with accessible GPIO pins
2. **LED Configuration**: 
   - Primary: WS2812 addressable LED on GPIO48 (configurable)
   - Fallback: Individual RGB LEDs on GPIO47, 21, 14
3. **USB Connection**: Serial communication at 115200 baud

### Software Installation
1. **Clone the repository**:
   ```bash
   git clone https://github.com/JupitersGhost/CipherChaos.git
   cd CipherChaos
   ```

2. **Create and activate virtual environment** (strongly recommended):
   ```bash
   # Create virtual environment
   python -m venv cipherchaos
   
   # Activate virtual environment
   # Windows:
   cipherchaos\Scripts\activate
   
   # Linux/macOS:
   source cipherchaos/bin/activate
   ```

3. **Install Python dependencies**:
   ```bash
   pip install -r requirements.txt
   ```

4. **Build PQC bindings** (optional - experimental feature):
   ```bash
   # Only needed if you want to experiment with PQC features
   # Install Rust toolchain first: https://rustup.rs/
   
   # Install maturin (Python build tool for Rust)
   pip install maturin
   
   # Build and install the experimental PQC bindings
   maturin develop
   
   # Note: This enables the PQC checkbox in GUI, but functionality is limited
   ```

5. **Flash ESP32-S3 firmware**:
   ```bash
   # Copy main.py to your ESP32-S3 board as main.py
   # Ensure MicroPython is installed on the board
   # Recommended tool: Thonny IDE for easy file transfer
   ```

6. **Run CipherChaos**:
   ```bash
   # Make sure virtual environment is activated first
   python cipherchaoslive.py
   ```

## ESP32-S3 Firmware Setup

### Flashing the Firmware
1. Install MicroPython on your ESP32-S3
2. Copy `main.py` to the board as the main script (I recommend Thonny)
3. The firmware will auto-start and begin personality initialization

### Firmware Commands
The ESP32 firmware supports various serial commands:

- `RGB:r,g,b` - Set LED color (0-255 each)
- `BRI:0.5` - Set brightness (0.01-1.0)
- `VER?` - Get firmware version
- `STAT?` - Get detailed status JSON
- `TRNG:START,10` - Start TRNG streaming at 10Hz
- `TRNG:STOP` - Stop TRNG streaming
- `TEST?` - Run comprehensive diagnostics
- `RESET` - Restart the device

## Configuration

### GUI Configuration
- **Window Duration**: 0.2-30.0 seconds for entropy collection
- **LED Brightness**: 1-100% brightness control
- **Entropy Sources**: Toggle keyboard, mouse, host RNG, ESP32 TRNG
- **PQC Settings**: Enable Kyber/Falcon key wrapping
- **Log File**: Custom path for key storage

### ESP32 Configuration
The firmware stores configuration in `cipher_enhanced_cfg.json`:
```json
{
  "led_pin": 48,
  "brightness": 1.0,
  "personality_level": 0.3,
  "debug_mode": false,
  "led_type": "ws2812"
}
```

### Hardware Pin Configuration
- **Default LED Pin**: GPIO48 (WS2812)
- **Fallback RGB**: GPIO47 (R), GPIO21 (G), GPIO14 (B)
- **Serial**: USB CDC (115200 baud)

## Post-Quantum Cryptography

**⚠️ EXPERIMENTAL FEATURE - WORK IN PROGRESS ⚠️**

The PQC implementation is currently under development and should be considered experimental. While the basic framework exists, the functionality is incomplete and not recommended for production use.

### Current Implementation Status
- **Framework**: Basic PQC manager and key wrapping structure implemented
- **Algorithms**: Kyber512 KEM and Falcon512 signatures (via Rust bindings)
- **Status**: Bindings compile but full integration is incomplete
- **GUI Integration**: PQC checkbox present but functionality limited
- **Key Storage**: PQC-wrapped key saving partially implemented

### Building PQC Support (Optional/Experimental)
```bash
# Install Rust toolchain first:
# https://rustup.rs/

# Install maturin (Python build tool for Rust)
pip install maturin

# Build the experimental PQC bindings
maturin develop

# Note: Even with successful compilation, PQC features are not fully functional
```

### What Works vs. What Doesn't
**Currently Working:**
- Rust bindings compilation
- Basic PQC manager initialization
- Classical entropy generation and key forging

**Not Yet Working:**
- Complete PQC key wrapping integration
- Reliable PQC key unwrapping/verification
- Full post-quantum key lifecycle management
- Production-ready PQC entropy assessment

### Future Development
The PQC implementation will be completed in future releases. Currently, CipherChaos operates primarily in classical cryptography mode with high-quality entropy generation suitable for standard AES256 keys.

## Network Integration

### Network Support
CipherChaos includes network monitoring for mesh environments:
- **Headscale Detection**: Automatic Tailscale/Headscale connectivity detection
- **Mesh Peers**: Real-time peer count monitoring
- **Uplink Status**: Network connectivity visualization

### Network Commands
Send commands to remote CipherChaos instances through the mesh (future feature).

## Entropy Quality & Auditing

### Echo-tan Entropy Auditor
The system includes comprehensive entropy assessment:
- **Frequency Test**: Bit distribution analysis
- **Runs Test**: Consecutive bit pattern detection
- **Chi-Square Test**: Statistical randomness verification
- **Shannon Entropy**: Information density measurement
- **Compression Test**: Predictability assessment
- **NIST-Inspired Tests**: Block frequency and longest run analysis

### PQC Readiness
Entropy is assessed for post-quantum cryptographic suitability:
- **Minimum Score**: 70% overall quality
- **Entropy Rate**: ≥6.5 bits per byte
- **All Tests**: Must pass basic statistical requirements

## Character Personalities

### CipherChaos (Host-side)
The main GUI features dynamic quotes from Cipher, a chaotic frenzy fueled anime themed character with a passion for encrypting.
- **Startup Quips**: "Entropy buffet's open—who's hungry for bits?"
- **Key Generation**: "Another key minted—smell that? That's post-quantum spice."
- **Network**: "Packets scrambled, mesh tangled—chaos relay primed!"

### Cipher-tan (ESP32 Firmware)
The ESP32 firmware features additional quotes from Cipher:
- **Boot Messages**: "^^^ RNG Queen reporting for duty! Let's chaos it up!"
- **RGB Updates**: "*** Pretty colors! My LED is definitely more stylish!"
- **Key Forging**: "[*] Key forged in the fires of chaos!"

## System Requirements

### Host System
- **OS**: Windows 10/11, Linux, macOS
- **Python**: 3.8 or higher
- **RAM**: 4GB minimum, 8GB recommended
- **Storage**: 1GB for logs and keys
- **USB**: Available port for ESP32 connection

### ESP32-S3 Requirements
- **Board**: ESP32-S3 with USB CDC support
- **Flash**: 4MB minimum (8MB recommended)
- **RAM**: 512KB SRAM
- **GPIO**: At least 1 pin for LED output
- **MicroPython**: Latest stable version

## Tested Hardware

- **ESP32-S3-DevKitC-1**: Primary development board
- **ESP32-S3-WROOM-1**: Tested and supported
- **Custom Boards**: Any ESP32-S3 with accessible GPIOs
- **LEDs**: WS2812/WS2812B addressable LEDs (recommended)

## File Structure

```
CipherChaos/
├── cipherchaoslive.py      # Main GUI application
├── main.py                 # ESP32-S3 firmware (MicroPython)
├── Cargo.toml             # Rust PQC bindings configuration
├── src/
│   └── lib.rs             # Rust PQC bindings source
├── requirements.txt        # Python dependencies
├── icon.png               # Application icon
└── README.md              # This file
```

## Development

### Building from Source
```bash
# Clone repository
git clone https://github.com/JupitersGhost/cipherchaos.git
cd CipherChaos

# Create virtual environment
python -m venv cipherchaos-env

# Activate virtual environment
# Windows:
cipherchaos-env\Scripts\activate
# Linux/macOS:
source cipherchaos-env/bin/activate

# Install development dependencies
pip install -r requirements.txt

# Build PQC bindings (optional)
pip install maturin
maturin develop

# Run in development mode
python cipherchaoslive.py
```

### Contributing
- **Code Style**: Follow PEP 8 for Python, rustfmt for Rust
- **Testing**: Test with real ESP32-S3 hardware
- **Documentation**: Update README for new features
- **Security**: All cryptographic changes require review

## License

MIT License

This project uses:
- **PySide6**: Qt for Python (LGPL/Commercial)
- **pqcrypto**: Post-quantum cryptography implementations (Public Domain)
- **MicroPython**: Python 3 implementation for microcontrollers (MIT)

## Security Notice

**This is a proof-of-concept system.** While it implements proper cryptographic practices and post-quantum algorithms, it should not be used for production security applications without thorough security review and testing.

## Character & Art

Cipher-tan is an original character created for this project, representing the chaotic and precise nature of cryptographic entropy generation, while also being fun. The character embodies both the unpredictability of true randomness and the mathematical precision required for cryptographic applications.

## Contributing

Issues and pull requests welcome! Please test hardware changes with real ESP32-S3 boards and verify entropy quality meets cryptographic standards.
