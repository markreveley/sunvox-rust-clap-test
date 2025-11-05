# Local Development Instructions - SunVox CLAP Plugin

## Overview

This project is building a CLAP (CLever Audio Plugin) that integrates the SunVox modular synthesizer library. It's written in Rust using the nih-plug framework.

**Current Status**: Phase 1 Complete ✅ - Basic CLAP plugin structure is working

## Project Structure

```
sunvox-rust-clap-test/
├── src/
│   └── lib.rs              # Main plugin implementation
├── sunvox_lib/             # SunVox C library (multiple platforms)
│   └── sunvox_lib/
│       ├── headers/sunvox.h           # C API header
│       ├── linux/lib_x86_64/sunvox.so # Linux library
│       ├── windows/lib_x86_64/sunvox.dll
│       └── docs/readme.txt
├── Cargo.toml              # Rust dependencies
├── bundle.sh               # Build script
├── plan.md                 # Complete development plan (2 phases)
├── README.md               # Project overview
└── local_instructions.md   # This file
```

## Prerequisites

### Required Tools
- **Rust** (latest stable): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Build tools**: gcc, make, pkg-config
  - Ubuntu/Debian: `sudo apt install build-essential pkg-config`
  - Fedora: `sudo dnf install gcc make pkg-config`

### Testing Requirements
- A CLAP-compatible DAW (Digital Audio Workstation):
  - **Bitwig Studio** (recommended, best CLAP support)
  - **Reaper** (enable CLAP in preferences)
  - **Qtractor** (Linux)
  - Or use a CLAP validator tool

## Building the Plugin

### Quick Build
```bash
# Build and bundle in one command
./bundle.sh
```

### Manual Build
```bash
# Build release version (optimized)
cargo build --release

# Create CLAP bundle structure
mkdir -p target/release/sunvox_clap.clap
cp target/release/libsunvox_clap.so target/release/sunvox_clap.clap/sunvox_clap.so
```

### Build Output
- Compiled library: `target/release/libsunvox_clap.so` (Linux)
- CLAP bundle: `target/release/sunvox_clap.clap/`
- Size: ~1.1 MB (stripped, optimized)

## Installing the Plugin

### For Testing (User Install)
```bash
# Copy to user plugin directory
mkdir -p ~/.clap
cp -r target/release/sunvox_clap.clap ~/.clap/

# Verify installation
ls -l ~/.clap/sunvox_clap.clap/
```

### System-Wide Install (Optional)
```bash
sudo mkdir -p /usr/lib/clap
sudo cp -r target/release/sunvox_clap.clap /usr/lib/clap/
```

## Testing the Plugin

### In a DAW

1. **Install the plugin** (see above)
2. **Launch your DAW** (e.g., Bitwig Studio, Reaper)
3. **Rescan plugins** if needed
4. **Look for "SunVox CLAP"** in the plugin browser
   - Category: Instrument / Synthesizer
   - Vendor: SunVox CLAP Plugin
5. **Load the plugin** on a track
6. **Play audio through it** - it should pass through unchanged (Phase 1)

### Expected Behavior (Phase 1)
- ✅ Plugin appears in DAW plugin list
- ✅ Loads without errors
- ✅ Audio passes through cleanly (no processing yet)
- ✅ No crashes or glitches
- ✅ Can be loaded multiple times
- ❌ No sound generation yet (Phase 2)

### Troubleshooting

**Plugin doesn't appear in DAW:**
- Check installation path: `ls ~/.clap/`
- Rescan plugins in DAW
- Check DAW CLAP support is enabled
- Verify bundle structure: `ls ~/.clap/sunvox_clap.clap/sunvox_clap.so`

**Build errors:**
- Ensure Rust is up to date: `rustup update`
- Clean and rebuild: `cargo clean && cargo build --release`
- Check for network issues (nih-plug is fetched from git)

**Runtime crashes:**
- Check DAW logs for error messages
- Try in a different CLAP-compatible host
- Rebuild with debug symbols: `cargo build` (without --release)

## Development Workflow

### Making Code Changes

1. **Edit source files** (primarily `src/lib.rs`)
2. **Build**: `cargo build --release`
3. **Bundle**: `./bundle.sh` or manually copy to `.clap/` directory
4. **Test**: Restart DAW (or rescan) and test changes

### Fast Iteration
```bash
# Watch for changes and auto-rebuild (install cargo-watch first)
cargo install cargo-watch
cargo watch -x 'build --release'
```

### Debugging
```bash
# Build with debug symbols
cargo build

# Check for issues without full build
cargo check

# Run tests (when added)
cargo test

# View plugin symbols
nm -D target/release/sunvox_clap.clap/sunvox_clap.so | grep clap_entry
```

## Current Implementation (Phase 1)

### What's Implemented
- ✅ Basic CLAP plugin structure
- ✅ Proper nih-plug integration
- ✅ Stereo audio I/O (2 in, 2 out)
- ✅ Plugin metadata (name, vendor, ID, features)
- ✅ Passthrough audio processing
- ✅ CLAP entry point export

### Key Files to Understand

**`src/lib.rs`** - Main plugin code:
- `SunVoxPlugin` struct - Plugin state
- `SunVoxPluginParams` - Parameters (empty for now)
- `Plugin` trait implementation - Core plugin behavior
- `ClapPlugin` trait implementation - CLAP-specific metadata
- `process()` function - Audio callback (currently passthrough)

**`Cargo.toml`** - Dependencies:
- `nih_plug` - Plugin framework (from git)
- `crate-type = ["cdylib"]` - Builds dynamic library

## Next Steps: Phase 2 (SunVox Integration)

Phase 2 will add actual audio synthesis using the SunVox library. See `plan.md` for full details.

### Phase 2 Overview
1. **Create FFI bindings** for SunVox C API
2. **Link SunVox library** to the plugin
3. **Initialize SunVox** in offline mode (manual audio callback)
4. **Generate audio** in the `process()` function
5. **Test** sound output and stability

### Key SunVox Files (Already in Repo)
- Header: `sunvox_lib/sunvox_lib/headers/sunvox.h`
- Library: `sunvox_lib/sunvox_lib/linux/lib_x86_64/sunvox.so`
- Docs: `sunvox_lib/sunvox_lib/docs/readme.txt`
- Examples: `sunvox_lib/sunvox_lib/examples/`

### Starting Phase 2

When ready to implement Phase 2:

1. **Read the plan**: `plan.md` has detailed Phase 2 steps
2. **Study SunVox API**: Check `sunvox_lib/sunvox_lib/headers/sunvox.h`
3. **Choose binding approach**:
   - Option A: Use `bindgen` to auto-generate Rust bindings
   - Option B: Manually write FFI declarations for core functions
4. **Start with minimal integration**: Just initialize and deinitialize SunVox
5. **Gradually add audio generation**

### Phase 2 Key Functions Needed
```rust
// From sunvox.h - these need Rust FFI bindings:
sv_init()           // Initialize SunVox
sv_deinit()         // Cleanup
sv_open_slot()      // Open a slot
sv_close_slot()     // Close slot
sv_audio_callback() // Get audio (offline mode)
sv_play()           // Start playback
sv_stop()           // Stop playback
```

## Useful Commands Reference

```bash
# Building
cargo build                      # Debug build
cargo build --release            # Release build (optimized)
./bundle.sh                      # Build + bundle

# Development
cargo check                      # Fast syntax check
cargo clippy                     # Linting
cargo fmt                        # Format code

# Cleaning
cargo clean                      # Remove build artifacts
rm -rf ~/.clap/sunvox_clap.clap  # Remove installed plugin

# Plugin info
file target/release/libsunvox_clap.so           # Check file type
nm -D target/release/sunvox_clap.clap/*.so       # View symbols
ldd target/release/sunvox_clap.clap/*.so         # Check dependencies
```

## Resources

### Documentation
- **This project's plan**: `plan.md` - Complete 2-phase roadmap
- **nih-plug docs**: https://github.com/robbert-vdh/nih-plug
- **nih-plug examples**: https://github.com/robbert-vdh/nih-plug/tree/master/plugins
- **CLAP spec**: https://github.com/free-audio/clap
- **SunVox library**: https://warmplace.ru/soft/sunvox/sunvox_lib.php

### Example Plugins
Look at nih-plug's example plugins for reference:
- `examples/gain` - Simple gain plugin (good starting point)
- `examples/sine` - Generates audio (similar to what we need)
- `examples/poly_mod_synth` - Full synthesizer example

### Getting Help
- nih-plug Discord/discussions
- CLAP Discord: https://discord.gg/cleveraudioplugin
- SunVox forum: https://warmplace.ru/forum/

## Notes for AI Assistants

When working on this project with Claude or similar:

1. **Always build before testing**: Run `./bundle.sh` after code changes
2. **Phase 1 is complete**: Focus is on Phase 2 (SunVox integration)
3. **Refer to plan.md**: It has the complete roadmap with checklists
4. **SunVox library is bundled**: No need to download, it's in `sunvox_lib/`
5. **Target platform**: Currently Linux x86_64, but multiplatform possible
6. **Threading model**: Will use `SV_INIT_FLAG_ONE_THREAD` for simplicity
7. **Audio format**: float32 stereo at host sample rate

### Common Tasks for AI
- "Implement Phase 2 step 1" - Start SunVox integration
- "Add a gain parameter" - Practice with nih-plug parameters
- "Debug why plugin crashes on load" - Troubleshooting
- "Add MIDI support" - Future enhancement
- "Create bindgen bindings for SunVox" - FFI setup

## License Notes

This project combines:
- **nih-plug** - ISC License
- **SunVox library** - Check `sunvox_lib/sunvox_lib/docs/license/LICENSE.txt`
- Your code - TODO: Choose compatible license

Ensure license compatibility before distribution.

## Questions?

- Check `README.md` for project overview
- Check `plan.md` for detailed implementation plan
- Check SunVox documentation in `sunvox_lib/sunvox_lib/docs/`
- Examine `src/lib.rs` for current implementation
