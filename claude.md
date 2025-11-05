# Claude Code Context - SunVox CLAP Plugin

## Project Overview

This is a CLAP audio plugin project integrating the SunVox modular synthesizer library, written in Rust using the nih-plug framework.

**Goal**: Create a functional CLAP plugin that can generate audio using SunVox's powerful synthesis engine, usable in any CLAP-compatible DAW (Bitwig Studio, Reaper, etc.).

## Current Status

### ✅ Phase 1: Complete
A working "hello world" CLAP plugin has been implemented:
- Compiles and bundles successfully
- Loads in CLAP-compatible DAWs
- Passes audio through (no processing yet)
- Proper plugin metadata and structure in place

### 🔄 Phase 2: Next - SunVox Integration
Need to integrate the SunVox library for audio synthesis:
- Create FFI bindings for SunVox C API
- Link SunVox library to plugin
- Initialize SunVox in offline mode
- Generate audio in the plugin's process callback

## Key Files

### Source Code
- **`src/lib.rs`** - Main plugin implementation
  - `SunVoxPlugin` struct - Plugin state
  - `Plugin` trait impl - Core nih-plug interface
  - `ClapPlugin` trait impl - CLAP-specific metadata
  - `process()` function - Audio callback (currently passthrough)

### Build & Configuration
- **`Cargo.toml`** - Dependencies (nih-plug from git, cdylib crate type)
- **`bundle.sh`** - Build script that creates `.clap` bundle
- **`.gitignore`** - Excludes target/, Cargo.lock

### Documentation
- **`plan.md`** - Complete two-phase development plan with checklists
- **`README.md`** - Project overview and installation instructions
- **`local_instructions.md`** - Comprehensive local development guide
- **`claude.md`** - This file (AI assistant context)

### SunVox Library (Already Included)
- **`sunvox_lib/sunvox_lib/headers/sunvox.h`** - C API header
- **`sunvox_lib/sunvox_lib/linux/lib_x86_64/sunvox.so`** - Linux library
- **`sunvox_lib/sunvox_lib/docs/readme.txt`** - Documentation
- **`sunvox_lib/sunvox_lib/examples/`** - Example code

## Quick Reference

### Building
```bash
./bundle.sh                          # Build and create CLAP bundle
cargo build --release                # Manual build
cargo check                          # Fast syntax check
```

### Installing (for testing)
```bash
mkdir -p ~/.clap
cp -r target/release/sunvox_clap.clap ~/.clap/
```

### Project Structure
```
sunvox-rust-clap-test/
├── src/lib.rs              # Plugin implementation
├── Cargo.toml              # Rust config
├── bundle.sh               # Build script
├── plan.md                 # Development roadmap
├── README.md               # User documentation
├── local_instructions.md   # Development guide
└── sunvox_lib/             # SunVox C library
    └── sunvox_lib/
        ├── headers/sunvox.h
        ├── linux/lib_x86_64/sunvox.so
        └── docs/
```

## Common Tasks

### "Implement Phase 2"
See `plan.md` Phase 2 section for detailed steps. High-level approach:

1. **Create FFI bindings** (`src/sunvox_ffi.rs`):
   - Option A: Use bindgen to auto-generate from `sunvox.h`
   - Option B: Manually declare essential functions
   - Focus on: `sv_init`, `sv_deinit`, `sv_audio_callback`, `sv_open_slot`, `sv_play`

2. **Add build.rs** for linking:
   - Link to `sunvox_lib/sunvox_lib/linux/lib_x86_64/sunvox.so`
   - Handle runtime library loading

3. **Initialize SunVox** in plugin:
   - Add initialization in `Plugin::initialize()`
   - Use offline mode flags: `SV_INIT_FLAG_OFFLINE | SV_INIT_FLAG_AUDIO_FLOAT32 | SV_INIT_FLAG_ONE_THREAD`
   - Store SunVox state in `SunVoxPlugin` struct

4. **Generate audio** in `process()`:
   - Call `sv_audio_callback()` to get SunVox audio
   - Write to plugin's output buffer

5. **Test thoroughly**:
   - Build, install, load in DAW
   - Verify audio generation works
   - Check for crashes, glitches, memory leaks

### "Add a parameter"
Example: Add a gain control
- Add field to `SunVoxPluginParams` with `#[id = "gain"]` attribute
- Use `FloatParam::new()` with range and default
- Apply in `process()`: multiply buffer samples by parameter value
- See nih-plug's `examples/gain` for reference

### "Debug why plugin crashes"
- Build with debug info: `cargo build` (not --release)
- Check DAW logs/console output
- Add `nih_log!()` calls in Rust code
- Verify library loading: `ldd target/release/sunvox_clap.clap/*.so`
- Test in simpler host or CLAP validator

### "Add MIDI support"
- Change `const MIDI_INPUT: MidiConfig = MidiConfig::Basic`
- Process MIDI in `process()` using `context.next_event()`
- Forward MIDI notes to SunVox (will need Phase 2 complete)

## Important Considerations

### SunVox Integration (Phase 2)
- **Threading**: Use `SV_INIT_FLAG_ONE_THREAD` - all calls on audio thread
- **Sample Rate**: Initialize SunVox with plugin's sample rate from `initialize()`
- **Offline Mode**: Essential - use `SV_INIT_FLAG_OFFLINE` to control audio callback manually
- **Float Audio**: Use `SV_INIT_FLAG_AUDIO_FLOAT32` to match plugin format
- **Cleanup**: Call `sv_deinit()` in plugin drop/deactivate

### Safety & FFI
- All SunVox calls will be `unsafe` FFI
- Document why each unsafe block is safe
- Check for null pointers from C API
- Handle errors gracefully (don't panic in audio thread)

### Performance
- SunVox operations in `process()` must be real-time safe
- No allocations, locks, or system calls in audio thread
- Keep buffer sizes reasonable

## Development Guidelines

### Code Style
- Follow Rust conventions
- Use `cargo fmt` for formatting
- Run `cargo clippy` for lints
- Document public APIs and unsafe code

### Testing

**CRITICAL: Always run tests before committing!**

Run all applicable tests before committing changes:

1. **Unit Tests** (when available):
   ```bash
   cargo test
   ```

2. **Build Tests**:
   ```bash
   cargo build --release  # Must succeed
   cargo clippy           # Check for lints/warnings
   ./bundle.sh            # Verify bundle creation works
   ```

3. **E2E Tests** (end-to-end):
   - Install plugin to test location
   - Load in DAW (Bitwig/Reaper)
   - Verify plugin appears in browser
   - Check plugin loads without errors
   - Test audio processing works correctly

4. **User Tests** (manual validation):
   - Create multiple plugin instances
   - Verify load/unload doesn't crash
   - Monitor CPU usage
   - Check for audio glitches or artifacts
   - Test parameter changes (when applicable)
   - Verify plugin survives project save/reload

**Testing Checklist Before Commit:**
- [ ] `cargo test` passes (or N/A if no tests yet)
- [ ] `cargo build --release` succeeds
- [ ] `cargo clippy` shows no warnings
- [ ] `./bundle.sh` creates valid bundle
- [ ] Plugin loads in DAW without errors
- [ ] Core functionality works as expected
- [ ] No crashes or memory leaks observed

### Git Workflow
- Develop on branch: `claude/sunvox-d-investigation-011CUqJ5F3Ku7UyyrhQdrkzd`
- **Run all applicable tests before committing** (see Testing section above)
- Commit logical units of work
- Write descriptive commit messages
- Push when phase or feature complete
- Never commit broken or untested code

## Resources

### nih-plug
- Repository: https://github.com/robbert-vdh/nih-plug
- Documentation: https://docs.rs/nih_plug/
- Examples: https://github.com/robbert-vdh/nih-plug/tree/master/plugins
  - `gain` - Simple effect (good starting point)
  - `sine` - Audio generation (relevant for Phase 2)
  - `poly_mod_synth` - Full synthesizer

### SunVox
- Library docs: https://warmplace.ru/soft/sunvox/sunvox_lib.php
- Header file: `sunvox_lib/sunvox_lib/headers/sunvox.h` (in repo)
- Examples: `sunvox_lib/sunvox_lib/examples/` (in repo)
- Main site: https://warmplace.ru/soft/sunvox/

### CLAP
- Specification: https://github.com/free-audio/clap
- Forum: https://cleveraudioplugin.com/

### Rust FFI
- Rustonomicon: https://doc.rust-lang.org/nomicon/ffi.html
- Bindgen: https://rust-lang.github.io/rust-bindgen/

## Technical Details

### Plugin Metadata
- **Name**: SunVox CLAP
- **Vendor**: SunVox CLAP Plugin
- **ID**: `com.sunvox.clap-plugin`
- **Version**: 0.1.0
- **Audio I/O**: Stereo (2 in, 2 out)
- **Features**: Instrument, Synthesizer, Stereo

### Build Configuration
- **Crate Type**: cdylib (dynamic library)
- **Rust Edition**: 2021
- **Dependencies**: nih-plug (from git)
- **Profiles**: release (optimized, stripped), profiling (debug+optimized)

### Current Architecture (Phase 1)
```rust
struct SunVoxPlugin {
    params: Arc<SunVoxPluginParams>,
}

impl Plugin for SunVoxPlugin {
    fn process(&mut self, buffer: &mut Buffer, ...) -> ProcessStatus {
        // Currently: passthrough (no processing)
        // Phase 2: Call SunVox audio generation here
        ProcessStatus::Normal
    }
}

impl ClapPlugin for SunVoxPlugin {
    const CLAP_ID: &'static str = "com.sunvox.clap-plugin";
    // ... metadata
}

nih_export_clap!(SunVoxPlugin);
```

### Future Architecture (Phase 2)
```rust
struct SunVoxPlugin {
    params: Arc<SunVoxPluginParams>,
    // Add:
    sunvox_slot: i32,
    sample_rate: f32,
}

impl Plugin for SunVoxPlugin {
    fn initialize(&mut self, ...) -> bool {
        // Initialize SunVox with offline mode
        // Open slot, load project or create modules
        true
    }

    fn process(&mut self, buffer: &mut Buffer, ...) -> ProcessStatus {
        // Call sv_audio_callback() to get SunVox audio
        // Write to buffer
        ProcessStatus::Normal
    }
}
```

## Troubleshooting

### Build Issues
- **nih-plug not found**: Check network, git is accessible
- **Linker errors**: Install build-essential (Debian/Ubuntu)
- **Edition 2024**: Change to 2021 in Cargo.toml if needed

### Runtime Issues
- **Plugin doesn't appear**: Check installation path, rescan DAW
- **Crashes on load**: Check DAW logs, rebuild with debug
- **No audio**: Phase 1 is passthrough only (expected)
- **Library not found** (Phase 2): Check LD_LIBRARY_PATH or bundle library

### DAW-Specific
- **Bitwig**: Preferences > Locations > VST/CLAP paths
- **Reaper**: Preferences > Plug-ins > CLAP, rescan
- Check DAW supports CLAP format

## Next Steps for Development

### Immediate (Phase 2 Start)
1. Read Phase 2 steps in `plan.md` carefully
2. Study `sunvox_lib/sunvox_lib/headers/sunvox.h` - understand API
3. Look at SunVox examples in `sunvox_lib/sunvox_lib/examples/`
4. Decide on binding approach (bindgen vs manual)
5. Create `src/sunvox_ffi.rs` with essential function declarations

### Short Term (Phase 2 Core)
1. Add `build.rs` to link SunVox library
2. Initialize SunVox in `Plugin::initialize()`
3. Call `sv_audio_callback()` in `process()`
4. Test audio generation works

### Medium Term (Phase 2 Polish)
1. Add error handling
2. Test multiple instances
3. Handle cleanup properly
4. Verify no memory leaks or crashes

### Long Term (Post Phase 2)
1. Add parameters (map to SunVox module controllers)
2. MIDI input support
3. Project file loading
4. GUI for project selection
5. Preset management

## Communication with User

When working with the user:
- **Ask for clarification** on ambiguous requirements
- **Show progress** on multi-step tasks
- **Explain decisions** when choosing between options
- **Test thoroughly before marking complete** - ALWAYS run unit tests, e2e tests, and user tests before committing or claiming work is done
- **Provide examples** of how to test changes
- **Document** any assumptions or trade-offs made
- **Report test results** - Include what tests were run and their outcomes in completion messages

## Success Metrics

### Phase 1 ✅ (Complete)
- [x] Plugin builds without errors
- [x] Loads in CLAP-compatible DAW
- [x] Audio passes through cleanly
- [x] No crashes on load/unload

### Phase 2 (In Progress)
- [ ] SunVox initializes successfully
- [ ] Plugin generates audible sound from SunVox
- [ ] Audio output is clean (no glitches)
- [ ] Multiple instances work independently
- [ ] Safe cleanup on plugin unload

---

**Last Updated**: Phase 1 Complete
**Next Milestone**: Phase 2 - SunVox Integration
**Priority**: Implement FFI bindings and basic audio generation
