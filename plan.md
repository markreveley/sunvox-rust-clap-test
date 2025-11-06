# SunVox CLAP Plugin Development Plan

## Overview
This project aims to create a CLAP (CLever Audio Plugin) plugin written in Rust that integrates with the SunVox modular synthesizer library. The development is divided into two main phases:

1. **Phase 1**: Create a basic "Hello World" CLAP plugin using nih-plug ✅ **COMPLETE**
2. **Phase 2**: Integrate SunVox library for audio generation 🔄 **IN PROGRESS**

---

## Phase 1: Basic CLAP Plugin with nih-plug ✅ COMPLETE

### Goal
Create the simplest possible working CLAP plugin that can be loaded in a DAW (Digital Audio Workstation) and passes audio through without modification.

### Steps

#### 1.1 Project Setup
- [x] Initialize a new Rust project with `cargo new --lib sunvox-clap`
- [x] Configure `Cargo.toml` with necessary dependencies:
  - `nih_plug` - The main framework for building audio plugins
  - `nih_plug_clap` - CLAP format support
- [x] Set up the crate type as `cdylib` for dynamic library output

#### 1.2 Create Minimal Plugin Structure
- [x] Define a basic plugin struct implementing the `Plugin` trait from nih-plug
- [x] Set up plugin metadata (name, vendor, version, unique ID)
- [x] Implement the `nih_export_clap!()` macro to export the plugin
- [x] Create basic audio processing callback (simple passthrough initially)
- [x] Add minimal parameter setup (even if empty)

#### 1.3 Build Configuration
- [x] Configure build targets for the host platform (Linux initially)
- [x] Set up proper library naming for CLAP plugins (`.clap` extension)
- [x] Create build script if needed for platform-specific configuration

#### 1.4 Testing
- [x] Build the plugin: `cargo build --release`
- [x] Verify the compiled library has correct format
- [x] Test loading the plugin in a CLAP-compatible DAW (e.g., Bitwig Studio, Reaper)
- [x] Verify audio passthrough works correctly
- [x] Check that plugin appears in DAW's plugin list

### Expected Output
A working CLAP plugin file that:
- Loads successfully in a DAW
- Appears in the plugin browser
- Passes audio through without crashes
- Can be instantiated multiple times

### Phase 1 Completion Summary

**Status**: ✅ Complete
**Files Created**:
- `src/lib.rs` - Plugin implementation (SunVoxPlugin struct)
- `Cargo.toml` - Dependencies and build configuration
- `bundle.sh` - Build and bundling script
- `README.md` - Project documentation
- `local_instructions.md` - Local development guide

**Plugin Details**:
- Name: SunVox CLAP
- ID: `com.sunvox.clap-plugin`
- Size: ~1.1 MB (stripped, optimized)
- Audio I/O: Stereo (2 in, 2 out)
- Current behavior: Passthrough (no processing)

**Build Output**: `target/release/sunvox_clap.clap/`

### Key References
- nih-plug GitHub: https://github.com/robbert-vdh/nih-plug
- nih-plug examples: `examples/gain` and `examples/sine` in the repo
- CLAP specification: https://github.com/free-audio/clap

---

## Phase 2: SunVox Library Integration

### Goal
Integrate the SunVox library to enable basic audio generation within the CLAP plugin.

### Prerequisites
- Completed Phase 1 with working CLAP plugin
- SunVox library files available at `sunvox_lib/sunvox_lib/`
- SunVox header file at `sunvox_lib/sunvox_lib/headers/sunvox.h`

### Phase 2 Progress Summary

**Status**: ⚠️ Core implementation complete, but blocked by SunVox initialization issue

**Completed Steps**:
- ✅ Step 2.1: FFI Bindings Setup
- ✅ Step 2.2: Library Linking
- ✅ Step 2.3: SunVox Initialization in Plugin
- ✅ Step 2.4: Basic Audio Integration
- ✅ Investigation: Created diagnostic tools and comprehensive analysis

**Current Blocker**: 🚨
`sv_init()` fails in environments without audio hardware access, even with `SV_INIT_FLAG_OFFLINE`. See detailed analysis in:
- `SUNVOX_INIT_INVESTIGATION.md` - Technical findings
- `JUCE_FORUM_ANALYSIS.md` - Developer confirmation and nuances
- `SUNVOX_BUG_REPORT.md` - Questions for SunVox developer

**Key Discoveries**:
1. ✅ SunVox developer (NightRadio) confirms plugins ARE possible (2021 Juce forum)
2. ⚠️ `SV_INIT_FLAG_OFFLINE` prevents audio *streaming* but NOT audio *initialization*
3. ⚠️ Requires audio hardware to be ACCESSIBLE (even if unused by SunVox)
4. ✅ Works on systems with audio hardware present
5. ❌ Fails in containerized/strictly sandboxed environments

**Next Steps**: Test on real hardware, contact developer, evaluate workarounds

### Steps

#### 2.1 FFI Bindings Setup ✅ COMPLETE
- [x] Create Rust FFI bindings for SunVox C API
  - Option B: Manually write FFI declarations for essential functions (chosen)
- [x] Focus on core functions needed:
  - `sv_init()` - Initialize SunVox
  - `sv_deinit()` - Cleanup SunVox
  - `sv_audio_callback()` - Get audio from SunVox (offline mode)
  - `sv_open_slot()` - Open a SunVox slot
  - `sv_close_slot()` - Close a slot
  - `sv_load()` - Load a SunVox project
  - `sv_play()` / `sv_play_from_beginning()` - Start playback
  - `sv_stop()` - Stop playback
  - Plus additional functions: `sv_get_ticks()`, `sv_get_sample_rate()`, `sv_send_event()`, etc.
- [x] Created `src/sunvox_ffi.rs` with comprehensive FFI bindings
- [x] Added unit tests to verify FFI bindings work correctly

#### 2.2 Library Linking ✅ COMPLETE
- [x] Add `build.rs` script to handle platform-specific linking
- [x] Configure linking to appropriate SunVox library:
  - Linux: `sunvox_lib/sunvox_lib/linux/lib_x86_64/sunvox.so` (implemented)
  - Windows: `sunvox_lib/sunvox_lib/windows/lib_x86_64/sunvox.dll` (planned)
  - macOS: `sunvox_lib/sunvox_lib/macos/lib_x86_64/sunvox.dylib` (planned)
- [x] Set up runtime loading using rpath (added to `build.rs`)
- [x] Created `libsunvox.so` symlink for linker compatibility
- [x] Successfully tested library loading and linking

#### 2.3 SunVox Initialization in Plugin ✅ COMPLETE
- [x] Initialize SunVox in plugin's `initialize()` method
  - Use `SV_INIT_FLAG_OFFLINE` flag for manual audio callback ✓
  - Use `SV_INIT_FLAG_AUDIO_FLOAT32` to match plugin's audio format ✓
  - Use `SV_INIT_FLAG_ONE_THREAD` for simpler threading model ✓
  - Use `SV_INIT_FLAG_NO_DEBUG_OUTPUT` to reduce noise ✓
- [x] Open a SunVox slot for playback (slot 0)
- [x] Store SunVox state in plugin struct (`sunvox_initialized`, `sunvox_slot`, `sample_rate`)
- [x] Implement proper cleanup in plugin's `deactivate()` method
- [x] Added graceful error handling for initialization failures

#### 2.4 Basic Audio Integration ✅ COMPLETE
- [x] Implement simplest possible audio generation:
  - **Option 1**: Load a simple SunVox project file (.sunvox) ✓ (chosen approach)
  - Loads `sunvox_lib/sunvox_lib/resources/song01.sunvox` on initialization
- [x] Call `sv_audio_callback()` in the plugin's `process()` function
- [x] Replace audio buffer with SunVox output (de-interleaved stereo)
- [x] Handle sample rate matching between plugin and SunVox (initialized with host sample rate)
- [x] Start playback automatically with `sv_play_from_beginning()`
- [x] **Result**: Plugin now generates real audio from SunVox! 🎵

#### 2.5 Error Handling & Safety 🔄 IN PROGRESS
- [x] Add proper error handling for SunVox initialization failures (graceful degradation with logging)
- [x] Ensure thread safety (SunVox in single-threaded mode with `SV_INIT_FLAG_ONE_THREAD`)
- [ ] Handle null pointer checks from FFI (currently trusting SunVox API)
- [ ] Add safety documentation for `unsafe` blocks (needs improvement)
- [ ] Gracefully handle missing library files (currently assumes files exist)
- [ ] Add more comprehensive error recovery and reporting

#### 2.6 Testing & Validation 🔄 IN PROGRESS
- [x] Build the integrated plugin (successful)
- [x] Test plugin loading in DAW (tested locally by user)
- [x] Verify SunVox audio is generated (confirmed - audio playing!)
- [ ] Check for audio glitches, clicks, or distortion (needs thorough testing)
- [ ] Monitor CPU usage and performance (needs measurement)
- [ ] Test plugin unload and cleanup (no crashes or memory leaks)
- [ ] Verify multiple instances work independently (needs testing)

### Expected Output
A CLAP plugin that:
- Successfully initializes SunVox library
- Generates audio using SunVox engine
- Produces clean, glitch-free output
- Can be loaded/unloaded safely
- Demonstrates basic integration as proof-of-concept

### Key SunVox API Information

**Initialization Pattern (Offline Mode):**
```c
// Initialize with offline audio callback
sv_init(NULL, 44100, 2, SV_INIT_FLAG_OFFLINE | SV_INIT_FLAG_AUDIO_FLOAT32 | SV_INIT_FLAG_ONE_THREAD);

// Open a slot
int slot = 0;
sv_open_slot(slot);

// Load or create content
sv_load(slot, "path/to/project.sunvox");

// Start playback
sv_play(slot);

// In audio callback:
sv_audio_callback(output_buffer, frames, 0, sv_get_ticks());

// Cleanup:
sv_close_slot(slot);
sv_deinit();
```

**Important Flags:**
- `SV_INIT_FLAG_OFFLINE` - Disables SunVox's internal audio system, we manually call `sv_audio_callback()`
- `SV_INIT_FLAG_AUDIO_FLOAT32` - Output as float samples (matching most plugin APIs)
- `SV_INIT_FLAG_ONE_THREAD` - Simplifies threading (all operations in same thread)

---

## Technical Considerations

### Sample Rate Handling
- Plugin sample rate may differ from SunVox initialization
- Consider resampling if rates don't match
- Or reinitialize SunVox when plugin sample rate changes

### Threading Model
- Use `SV_INIT_FLAG_ONE_THREAD` to avoid complex synchronization
- All SunVox calls should be from audio thread when using this flag
- Alternative: Use separate SunVox thread with lock/unlock mechanism

### Library Distribution
- SunVox library needs to be accessible at runtime
- Options:
  1. Bundle `.so`/`.dll`/`.dylib` with plugin
  2. Use dynamic loading (`dlopen`/`LoadLibrary`)
  3. Expect user to install SunVox library separately

### Platform Differences
- Linux: `.so` shared objects
- Windows: `.dll` dynamic libraries
- macOS: `.dylib` dynamic libraries
- May need platform-specific `build.rs` logic

---

## Future Enhancements (Post Phase 2)

Once basic integration works, consider:
- [ ] Parameter controls (map plugin parameters to SunVox module controllers)
- [ ] MIDI input support (forward MIDI to SunVox)
- [ ] Project loading UI (allow users to load .sunvox files)
- [ ] Multiple SunVox slots for layering
- [ ] Pattern/timeline control
- [ ] Preset management
- [ ] GUI using nih-plug's VIZIA or egui support

---

## Resources & References

### nih-plug
- Repository: https://github.com/robbert-vdh/nih-plug
- Documentation: https://docs.rs/nih_plug/
- Examples: Check `examples/` directory for gain, sine, poly-mod-synth

### SunVox Library
- Main site: https://warmplace.ru/soft/sunvox/sunvox_lib.php
- Header: `sunvox_lib/sunvox_lib/headers/sunvox.h`
- Documentation: `sunvox_lib/sunvox_lib/docs/readme.txt`
- Examples: `sunvox_lib/sunvox_lib/examples/`
- License: `sunvox_lib/sunvox_lib/docs/license/LICENSE.txt`

### CLAP Format
- Specification: https://github.com/free-audio/clap
- Forum: https://cleveraudioplugin.com/

### Rust FFI
- Rustonomicon (Unsafe Rust): https://doc.rust-lang.org/nomicon/
- Bindgen tool: https://rust-lang.github.io/rust-bindgen/

---

## Success Criteria

### Phase 1 Complete When:
- Plugin builds without errors
- Plugin loads in at least one CLAP-compatible DAW
- Audio passes through cleanly
- No crashes during load/unload cycles

### Phase 2 Complete When:
- SunVox library initializes successfully
- Plugin generates audible sound from SunVox
- Audio output is clean and glitch-free
- Plugin can be safely loaded/unloaded multiple times
- Basic proof-of-concept demonstrates SunVox integration viability

---

## Development Environment

### Required Tools
- Rust toolchain (latest stable)
- Cargo
- C compiler (gcc/clang for linking)
- A CLAP-compatible DAW for testing
  - Bitwig Studio (Linux, macOS, Windows)
  - Reaper (with CLAP support)
  - Others: FL Studio, Qtractor

### Build Commands
```bash
# Build debug version
cargo build

# Build release version (optimized)
cargo build --release

# Run tests
cargo test

# Check for errors without building
cargo check
```

### Installation Locations
CLAP plugins typically installed at:
- Linux: `~/.clap/` or `/usr/lib/clap/`
- Windows: `C:\Program Files\Common Files\CLAP\`
- macOS: `~/Library/Audio/Plug-Ins/CLAP/` or `/Library/Audio/Plug-Ins/CLAP/`
