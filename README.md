# CipherChaos

<div align="center">
  <img src="icon.png" alt="CipherChaos" width="200" height="200" style="border-radius: 50%;">
  
  **CipherChaos - Enhanced Entropy Generation & Post-Quantum Cryptography**
</div>

---

CipherChaos is an advanced entropy generation system combining ESP32-S3 hardware with host-side processing to create high-quality random keys. Features post-quantum cryptography integration, real-time entropy visualization, and personality-driven interaction through a quirky chaotic character, "Cipher-tan".

This system has been tested on Windows 11 with ESP32-S3 development boards and supports both classical and post-quantum key wrapping.

## Features

### Core Functionality
- **Multi-Source Entropy**: Keystroke timing, mouse movement, ESP32 TRNG, USB jitter
- **Classical Cryptography**: High-quality AES256 key generation with comprehensive entropy auditing
- **Real-time Visualization**: Live entropy monitoring with animated displays
- **Enhanced Auditing**: NIST-inspired entropy quality assessment by "Echo-tan"
- **Hardware Integration**: ESP32-S3 with WS2812 RGB LEDs and enhanced firmware
- **Network Support**: Headscale integration for distributed entropy (work in progress)
- **Post-Quantum Cryptography**: Kyber512 KEM and Falcon512 signatures with hybrid key wrapping

### ESP32-S3 Features (Cipher-chan Enhanced Firmware v2.1)
- **TRNG Streaming**: High-frequency true random number generation
- **WiFi Entropy**: Environmental RF noise collection --WIP--
- **USB Jitter**: Timing-based entropy from USB communications
- **RGB Control**: Dynamic LED feedback with 16M+ colors
- **Personality System**: Cipher-tan's cryptographic commentary
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

# For optional PQC support:
# 1. Install Rust toolchain from https://rustup.rs/
# 2. Install maturin: pip install maturin==1.6.0
# 3. Build bindings: maturin develop --release
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

4. **Build PQC bindings** (optional):
   ```bash
   # Install Rust toolchain first: https://rustup.rs/
   
   # Install maturin (Python build tool for Rust)
   pip install maturin
   
   # Build and install the PQC bindings
   maturin develop
   
   # This enables full PQC functionality in the GUI
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

CipherChaos includes optional post-quantum cryptography integration using NIST-standardized algorithms. 
The implementation provides hybrid classical-quantum protection through key wrapping and digital signatures. 
If PQC bindings are not built, the system will still function with classical AES256 key generation.

### Algorithms Supported
- **Kyber512 KEM**: Key encapsulation mechanism for hybrid key wrapping
- **Falcon512**: Digital signatures for key authentication
- **Hybrid Approach**: Classical AES256 keys wrapped with post-quantum protection

### Core Features
- Real-time entropy quality assessment for PQC readiness (≥65% audit score, ≥6.0 bits/byte)
- Automatic PQC key wrapping when entropy quality meets thresholds (if PQC is built)
- Individual algorithm selection (Kyber512 KEM / Falcon512 signatures)
- Secure key storage with separate public/private key files
- Live status monitoring showing current key protection level

### Building PQC Support
```bash
# Install Rust toolchain first:
# https://rustup.rs/

# Install maturin (Python build tool for Rust)
pip install maturin==1.6.0

# Build the PQC bindings
maturin develop --release

# Verify installation
python -c "import pqcrypto_bindings; print('PQC bindings available')"
```

### Dependencies

The PQC implementation uses these **Rust crate dependencies** (already configured in `Cargo.toml`):
- `pqcrypto-kyber = "0.4.0"` - Kyber512 KEM implementation
- `pqcrypto-falcon = "0.4.0"` - Falcon512 signature implementation  
- `pqcrypto-traits = "0.3.5"` - Common traits for PQC algorithms
- `pyo3 = { version = "0.20.3", features = ["extension-module"] }`

**Build tool dependency:**
- `maturin = "1.6.0"`

### Usage
1. **Enable PQC**: Check "Enable PQC Key Wrapping" in the Chaos Control panel
2. **Algorithm Selection**: Choose Kyber512 KEM and/or Falcon512 signatures
3. **Entropy Generation**: Start chaos generation - PQC activates automatically when entropy quality is sufficient
4. **Key Storage**: PQC-wrapped keys are automatically saved to the `keys/` directory

### Current Status
- Kyber512 key encapsulation and hybrid wrapping
- Falcon512 digital signatures for key authentication
- Real-time entropy auditing with PQC readiness assessment
- GUI integration with live status updates
- Automatic key storage and logging

**Security Considerations:**
- Implementation uses NIST-standardized algorithms
- Entropy thresholds are configurable but set to practical defaults
- Keys are stored separately from wrapped data for security
- Classical fallback maintains compatibility when PQC is not enabled

**Limitations:**
- PQC bindings compilation required for functionality
- Key unwrapping/verification utilities not included in GUI
- Production deployment should undergo independent security review

### Current Implementation

**Algorithms Supported:**
- **Kyber512 KEM**: Key encapsulation mechanism for hybrid key wrapping
- **Falcon512**: Digital signatures for key authentication
- **Hybrid Approach**: Classical AES256 keys wrapped with post-quantum protection

**Core Features:**
- Real-time entropy quality assessment for PQC readiness (≥65% audit score, ≥6.0 bits/byte)
- Automatic PQC key wrapping when entropy quality meets thresholds
- Individual algorithm selection (Kyber512 KEM / Falcon512 signatures)
- Secure key storage with separate public/private key files
- Live status monitoring showing current key protection level

### Building PQC Support

```bash
# Install Rust toolchain first:
# https://rustup.rs/

# Install maturin (Python build tool for Rust)
pip install maturin

# Build the PQC bindings
maturin develop

# Verify installation
python -c "import pqcrypto_bindings; print('PQC bindings available')"
```

### Dependencies

The PQC implementation uses these **Rust crate dependencies** (already configured in `Cargo.toml`):
- `pqcrypto-kyber = "0.4.0"` - Kyber512 KEM implementation
- `pqcrypto-falcon = "0.4.0"` - Falcon512 signature implementation  
- `pqcrypto-traits = "0.3.5"` - Common traits for PQC algorithms
- `pyo3 = { version = "0.20", features = ["extension-module"] }` - Python bindings

**Build tool dependency:**
- `maturin = "1"` - Rust-to-Python build system

### Usage

1. **Enable PQC**: Check "Enable PQC Key Wrapping" in the Chaos Control panel
2. **Algorithm Selection**: Choose Kyber512 KEM and/or Falcon512 signatures
3. **Entropy Generation**: Start chaos generation - PQC activates automatically when entropy quality is sufficient
4. **Key Storage**: PQC-wrapped keys are automatically saved to the `keys/` directory

### Current Status

**Fully Functional:**
- Kyber512 key encapsulation and hybrid wrapping
- Falcon512 digital signatures for key authentication
- Real-time entropy auditing with PQC readiness assessment
- GUI integration with live status updates
- Automatic key storage and logging

**Security Considerations:**
- Implementation uses NIST-standardized algorithms
- Entropy thresholds are configurable but set to practical defaults
- Keys are stored separately from wrapped data for security
- Classical fallback maintains compatibility when PQC is disabled

**Limitations:**
- PQC bindings compilation required for functionality
- Key unwrapping/verification utilities not included in GUI
- Production deployment should undergo independent security review

The PQC implementation provides forward security against quantum computing threats while maintaining backward compatibility with classical cryptographic workflows.

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

**Apache License 2.0**

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

### Third-Party Acknowledgments
This project incorporates or links to the following open-source technologies:
- **PySide6**: Qt for Python (LGPL v3 / Commercial)
- **pqcrypto**: Post-quantum cryptography implementations (Public Domain / MIT)
- **MicroPython**: Python 3 implementation for microcontrollers (MIT)

## Security Notice

**This is a proof-of-concept system.** While it implements proper cryptographic practices and post-quantum algorithms, it should not be used for production security applications without thorough security review and testing.

## Character & Art

Cipher-tan is an original character created for this project, representing the chaotic and precise nature of cryptographic entropy generation, while also being fun. The character embodies both the unpredictability of true randomness and the mathematical precision required for cryptographic applications.

## Contributing

Issues and pull requests welcome! Please test hardware changes with real ESP32-S3 boards and verify entropy quality meets cryptographic standards.
