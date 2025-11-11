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

### 🚫 Phase 2: BLOCKED - SunVox ARM64 Library Bug
SunVox integration **code is complete**, but blocked by library bug:
- ✅ FFI bindings implemented (`src/sunvox_ffi.rs`)
- ✅ Library linking configured (`build.rs`)
- ✅ Plugin initialization code written
- ✅ Audio generation implemented with fallback (test tone)
- ❌ **BLOCKER**: SunVox Library v2.1.3 fails on macOS ARM64

**Issue**: `sv_init()` fails with error 0x20103 (CoreAudio initialization) on all ARM64 Macs, tested across 6 flag combinations and multiple environments. This is a library-level bug requiring developer fix.

**Status**: November 11, 2025 - Comprehensive testing complete, bug report prepared

**See**: `SUNVOX_ARM64_BUG_REPORT.md`, `TESTING.md`, `NEXT_STEPS.md` for details

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

### What to Expect (Current State)
Currently, the plugin:
- ✅ Appears as "SunVox CLAP" in the plugin browser
- ✅ Can be instantiated multiple times
- ✅ Does not crash or cause audio glitches
- ⚠️ **On macOS ARM64**: SunVox init fails → generates test tone instead
- ⚠️ **On other platforms**: May work if SunVox supports them

**Note**: SunVox integration code exists but cannot run due to ARM64 library bug (error 0x20103). Plugin gracefully falls back to test tone generation.

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
- **Phase 2**: SunVox integration (🚫 Blocked by ARM64 library bug)

### Documentation Overview

**⭐ Start Here for New Contributors**:
1. **`plan.md`** - Full development roadmap and current status
2. **`TESTING.md`** - All test results (Tests 1-6)
3. **`NEXT_STEPS.md`** - Action plan and alternatives

**Technical Details**:
- **`SUNVOX_ARM64_BUG_REPORT.md`** - Bug report for SunVox developer
- **`SUNVOX_INIT_INVESTIGATION.md`** - Technical investigation
- **`CLAUDE.md`** - AI assistant context and quick reference
- **`local_instructions.md`** - Local development setup

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

### Current Status (November 11, 2025)

Phase 2 implementation is **complete** but **blocked by SunVox ARM64 library bug**:
- ✅ Steps 1-4: All implementation done (FFI, linking, init, audio)
- ✅ Step 5: Comprehensive testing completed (6 configurations tested)
- ❌ **Result**: SunVox v2.1.3 ARM64 fails with error 0x20103 on all tests

### Immediate Actions Required

1. **Contact SunVox developer** with bug report (`SUNVOX_ARM64_BUG_REPORT.md`)
   - Forum: https://warmplace.ru/forum/
   - Subject: "SunVox Library v2.1.3 - CoreAudio Failure on macOS ARM64"

2. **Follow timeline** in `NEXT_STEPS.md`:
   - Week 1-2: Wait for developer response
   - Week 3: Test Rosetta workaround if needed
   - Week 4: Make strategic decision

3. **Alternative options** available (see `NEXT_STEPS.md`):
   - Rosetta 2 translation (x86_64 library)
   - Out-of-process architecture
   - Alternative synthesis engine
   - Wait for library fix (recommended)

See `NEXT_STEPS.md` for complete decision tree and action plan.
