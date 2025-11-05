# SunVox CLAP Plugin

A CLAP (CLever Audio Plugin) integrating the SunVox modular synthesizer, written in Rust using the [nih-plug](https://github.com/robbert-vdh/nih-plug) framework.

## Project Status

### ✅ Phase 1: Complete
A basic "Hello World" CLAP plugin has been successfully implemented. The plugin:
- ✅ Compiles successfully using nih-plug framework
- ✅ Exports proper CLAP entry point (`clap_entry`)
- ✅ Passes audio through without modification
- ✅ Is packaged as a proper CLAP bundle (`.clap` directory)
- ✅ Ready to be loaded by CLAP-compatible DAWs

### 🔄 Phase 2: Pending
Integration with SunVox library for audio synthesis (see `plan.md` for details).

## Building the Plugin

### Prerequisites
- Rust toolchain (latest stable)
- Linux build tools (gcc/clang)

### Build Instructions

```bash
# Build and bundle the plugin in one step
./bundle.sh

# Or build manually
cargo build --release
```

The compiled plugin will be located at: `target/release/sunvox_clap.clap/`

## Installing the Plugin

Copy the plugin bundle to one of the standard CLAP plugin directories:

```bash
# User-specific installation (recommended)
mkdir -p ~/.clap
cp -r target/release/sunvox_clap.clap ~/.clap/

# System-wide installation (requires sudo)
sudo mkdir -p /usr/lib/clap
sudo cp -r target/release/sunvox_clap.clap /usr/lib/clap/
```

## Testing the Plugin

The plugin can be loaded in any CLAP-compatible DAW, including:
- **Bitwig Studio** (Linux, macOS, Windows)
- **Reaper** (with CLAP support enabled)
- **Qtractor** (Linux)
- **FL Studio** (Windows)
- And others supporting the CLAP format

### What to Expect (Phase 1)
Currently, the plugin:
- Appears as "SunVox CLAP" in the plugin browser
- Can be instantiated multiple times
- Passes audio through without modification (no processing yet)
- Does not crash or cause audio glitches

This is the foundation for Phase 2, where SunVox audio generation will be integrated.

## Plugin Details

- **Name**: SunVox CLAP
- **Vendor**: SunVox CLAP Plugin
- **Format**: CLAP (CLever Audio Plugin)
- **ID**: `com.sunvox.clap-plugin`
- **Audio I/O**: 2 inputs → 2 outputs (stereo)
- **MIDI Support**: Not yet implemented
- **Features**: Instrument, Synthesizer, Stereo

## Project Structure

```
sunvox-rust-clap-test/
├── src/
│   └── lib.rs           # Plugin implementation
├── sunvox_lib/          # SunVox library files (for Phase 2)
├── Cargo.toml           # Rust dependencies
├── bundle.sh            # Build and bundling script
├── plan.md              # Detailed development plan
└── README.md            # This file
```

## Development Plan

See `plan.md` for the complete two-phase development plan:
- **Phase 1**: Basic CLAP plugin (✅ Complete)
- **Phase 2**: SunVox integration (🔄 Next)

## SunVox Library

The SunVox library is included in the `sunvox_lib/` directory:
- Documentation: `sunvox_lib/sunvox_lib/docs/readme.txt`
- Header file: `sunvox_lib/sunvox_lib/headers/sunvox.h`
- Linux libraries: `sunvox_lib/sunvox_lib/linux/lib_x86_64/sunvox.so`
- License: `sunvox_lib/sunvox_lib/docs/license/LICENSE.txt`

## References

- **nih-plug**: https://github.com/robbert-vdh/nih-plug
- **CLAP**: https://github.com/free-audio/clap
- **SunVox**: https://warmplace.ru/soft/sunvox/
- **SunVox Library**: https://warmplace.ru/soft/sunvox/sunvox_lib.php

## License

TODO: Add appropriate license information considering:
- nih-plug license (ISC)
- SunVox library license
- Your project license

## Next Steps

To proceed with Phase 2 (SunVox integration):
1. Create Rust FFI bindings for SunVox C API
2. Link SunVox library to the plugin
3. Initialize SunVox in offline mode
4. Integrate audio generation in the `process()` callback
5. Test audio output and stability

See `plan.md` for detailed Phase 2 steps.
