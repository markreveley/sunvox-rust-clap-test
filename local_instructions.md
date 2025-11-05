# Local Development Instructions - SunVox CLAP Plugin

## Overview

This project is building a CLAP (CLever Audio Plugin) that integrates the SunVox modular synthesizer library. It's written in Rust using the nih-plug framework.

**Current Status**:
- ✅ Phase 1 Complete - Basic CLAP plugin structure working
- 🔄 Phase 2 In Progress - SunVox integration (Steps 2.1 & 2.2 complete)
  - ✅ Step 2.1: FFI bindings created
  - ✅ Step 2.2: Library linking configured
  - ⏭️  Step 2.3: Next - Initialize SunVox in plugin

## Project Structure

```
sunvox-rust-clap-test/
├── src/
│   ├── lib.rs              # Main plugin implementation
│   └── sunvox_ffi.rs       # SunVox FFI bindings (NEW in Phase 2)
├── sunvox_lib/             # SunVox C library (multiple platforms)
│   └── sunvox_lib/
│       ├── headers/sunvox.h           # C API header
│       ├── linux/lib_x86_64/
│       │   ├── sunvox.so             # Linux library
│       │   └── libsunvox.so          # Symlink for linker (NEW)
│       ├── windows/lib_x86_64/sunvox.dll
│       └── docs/readme.txt
├── Cargo.toml              # Rust dependencies
├── build.rs                # Library linking configuration (NEW in Phase 2)
├── bundle.sh               # Build script
├── plan.md                 # Complete development plan (2 phases)
├── README.md               # Project overview
├── local_instructions.md   # This file
└── claude.md               # AI assistant context
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

### Before Committing - CRITICAL Testing Requirements

**ALWAYS run these tests before committing code:**

1. **Unit Tests**:
   ```bash
   cargo test --lib -- --nocapture
   ```

2. **Build Verification**:
   ```bash
   cargo build --release  # Must succeed
   cargo clippy           # Check warnings
   ./bundle.sh            # Verify bundle creation
   ```

3. **E2E Tests** (end-to-end):
   - Install plugin to DAW plugin directory
   - Load plugin in Bitwig/Reaper
   - Verify plugin appears and loads without errors
   - Test core functionality works

4. **User Tests** (manual validation):
   - Create multiple plugin instances
   - Test load/unload cycles
   - Check for crashes or glitches
   - Monitor CPU usage
   - Verify parameter changes work (when applicable)

**Pre-Commit Checklist:**
- [ ] `cargo test` passes
- [ ] `cargo build --release` succeeds
- [ ] `cargo clippy` has no warnings
- [ ] Plugin loads in DAW
- [ ] Core functionality verified
- [ ] No crashes or memory leaks

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

# Run unit tests (FFI bindings test)
cargo test --lib -- --nocapture

# View plugin symbols
nm -D target/release/sunvox_clap.clap/sunvox_clap.so | grep clap_entry

# Check SunVox library linking
ldd target/release/libsunvox_clap.so | grep sunvox
```

## Current Implementation

### Phase 1: Complete ✅
- ✅ Basic CLAP plugin structure
- ✅ Proper nih-plug integration
- ✅ Stereo audio I/O (2 in, 2 out)
- ✅ Plugin metadata (name, vendor, ID, features)
- ✅ Passthrough audio processing
- ✅ CLAP entry point export

### Phase 2: In Progress (Steps 2.1 & 2.2 Complete) 🔄

**✅ Step 2.1: FFI Bindings (COMPLETE)**
- Created `src/sunvox_ffi.rs` with Rust declarations for SunVox C API
- Functions: `sv_init`, `sv_deinit`, `sv_audio_callback`, `sv_open_slot`, `sv_close_slot`, `sv_load`, `sv_play`, `sv_stop`, `sv_volume`, `sv_send_event`, `sv_get_ticks`, `sv_get_sample_rate`, etc.
- Constants: `SV_INIT_FLAG_*`, `NOTECMD_*`
- Comprehensive documentation for each function
- Unit test: `test_sunvox_ffi_bindings` - verifies FFI calls work

**✅ Step 2.2: Library Linking (COMPLETE)**
- Created `build.rs` for compile-time linking
- Configured rpath for runtime library discovery
- Created `libsunvox.so` symlink (linker expects lib prefix)
- Platform: Linux x86_64 (extensible to Windows/macOS)

**⏭️ Step 2.3: SunVox Initialization (NEXT)**
- Add SunVox state to `SunVoxPlugin` struct
- Initialize SunVox in `Plugin::initialize()` method
- Use offline mode with float32 audio
- Implement proper cleanup on shutdown

### Testing FFI Bindings

Run the unit test to verify SunVox FFI bindings work:

```bash
cargo test --lib -- --nocapture
```

**Expected output:**
```
=== Testing SunVox FFI Bindings ===
Test 1: Initializing SunVox...
  ✓ SunVox initialized successfully
Test 2: Checking sample rate...
  ✓ SunVox initialized with sample rate: 44100 Hz
Test 3: Testing tick counters...
  ✓ Ticks per second: 50000
  ✓ Current tick: <number>
Test 4: Opening slot 0...
  ✓ Slot 0 opened successfully
Test 5: Closing slot 0...
  ✓ Slot 0 closed successfully
Test 6: Deinitializing SunVox...
  ✓ SunVox deinitialized successfully

=== All FFI binding tests passed! ===
test sunvox_ffi::tests::test_sunvox_ffi_bindings ... ok
```

**Note**: In containerized environments without audio hardware, `sv_init` may fail with error code 0x20103. This is expected and the test will pass anyway (FFI bindings are still verified).

### Key Files to Understand

**`src/lib.rs`** - Main plugin code:
- `SunVoxPlugin` struct - Plugin state
- `SunVoxPluginParams` - Parameters (empty for now)
- `Plugin` trait implementation - Core plugin behavior
- `ClapPlugin` trait implementation - CLAP-specific metadata
- `process()` function - Audio callback (currently passthrough)

**`src/sunvox_ffi.rs`** - SunVox FFI bindings (NEW):
- External C function declarations with `#[link(name = "sunvox")]`
- Constants for initialization and note commands
- Comprehensive documentation for each function
- Unit tests for FFI verification

**`build.rs`** - Build configuration (NEW):
- Links SunVox library at compile time
- Sets rpath for runtime library discovery
- Platform-specific (currently Linux, extensible)

**`Cargo.toml`** - Dependencies:
- `nih_plug` - Plugin framework (from git)
- `crate-type = ["cdylib"]` - Builds dynamic library

## Phase 2 Progress & Next Steps

### Completed Steps
1. ✅ **2.1 FFI Bindings** - Full SunVox C API accessible from Rust
2. ✅ **2.2 Library Linking** - SunVox library links automatically

### Next Steps
3. ⏭️ **2.3 Initialize SunVox** - Integrate into plugin structure
4. 🔜 **2.4 Audio Generation** - Call `sv_audio_callback()` in `process()`
5. 🔜 **2.5 Error Handling** - Robust error handling and safety
6. 🔜 **2.6 Testing** - Full validation in DAW with audio output

See `plan.md` for detailed step-by-step instructions for each remaining phase.

### Key Resources
- **Plan**: `plan.md` - Complete roadmap with detailed steps
- **SunVox Header**: `sunvox_lib/sunvox_lib/headers/sunvox.h` - C API reference
- **SunVox Docs**: `sunvox_lib/sunvox_lib/docs/readme.txt` - Library documentation
- **Claude Context**: `claude.md` - AI assistant context and common tasks

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

1. **Current Status**: Phase 2 Steps 2.1 & 2.2 complete (FFI bindings + linking)
2. **Next Task**: Step 2.3 - Initialize SunVox in plugin structure
3. **CRITICAL - Test before committing**: ALWAYS run unit tests, build tests, E2E tests, and user tests before committing (see "Before Committing" section above)
4. **Always build before testing**: Run `./bundle.sh` after code changes
5. **Refer to plan.md**: It has the complete roadmap with checklists
6. **SunVox library is bundled**: No need to download, it's in `sunvox_lib/`
7. **Target platform**: macOS (arm64), Linux (x86_64), Windows (x86_64)
8. **Threading model**: Use `SV_INIT_FLAG_ONE_THREAD` for simplicity
9. **Audio format**: float32 stereo at host sample rate
10. **FFI bindings ready**: `src/sunvox_ffi.rs` has all necessary functions

### Testing Requirements for AI
**Before any commit or claiming work complete:**
- Run `cargo test --lib -- --nocapture` - Unit tests must pass
- Run `cargo build --release` - Build must succeed
- Run `cargo clippy` - No warnings allowed
- Run `./bundle.sh` - Bundle creation must work
- Load plugin in DAW - Must load without errors
- Test functionality - Core features must work
- Report test results in completion message

### Common Tasks for AI
- "Implement Phase 2 step 2.3" - Next: Initialize SunVox in plugin (REMEMBER TO TEST)
- "Run tests" - `cargo test --lib -- --nocapture`
- "Add a gain parameter" - Practice with nih-plug parameters (TEST BEFORE COMMITTING)
- "Debug why plugin crashes on load" - Troubleshooting
- "Add MIDI support" - Future enhancement (after Phase 2, TEST THOROUGHLY)

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
