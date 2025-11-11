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

### 🚫 Phase 2: BLOCKED BY SUNVOX ARM64 LIBRARY BUG
SunVox integration implementation is complete, but blocked by library bug:
- ✅ FFI bindings created (`src/sunvox_ffi.rs`)
- ✅ Library linking configured (`build.rs`)
- ✅ Plugin initialization code written
- ✅ Audio generation code implemented
- ✅ Graceful error handling with fallback (test tone)
- ❌ **BLOCKER**: SunVox Library v2.1.3 fails on macOS ARM64 with error 0x20103

**Critical Issue**: `sv_init()` fails on macOS ARM64 across ALL flag combinations and environments (tested 6 configs, all failed with CoreAudio initialization error 0x20103)

**Status Date**: November 11, 2025
**Action Required**: Contact SunVox developer (see `SUNVOX_ARM64_BUG_REPORT.md`)

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
- **`plan.md`** - Complete two-phase development plan with checklists ⭐ START HERE
- **`README.md`** - Project overview and installation instructions
- **`local_instructions.md`** - Comprehensive local development guide
- **`CLAUDE.md`** - This file (AI assistant context)
- **`TESTING.md`** - Complete test history (Tests 1-6) ⭐ CRITICAL
- **`SUNVOX_ARM64_BUG_REPORT.md`** - Bug report for SunVox developer ⭐ ACTION ITEM
- **`NEXT_STEPS.md`** - Decision tree and alternatives ⭐ ROADMAP
- **`SUNVOX_INIT_INVESTIGATION.md`** - Technical investigation details
- **`JUCE_FORUM_ANALYSIS.md`** - Developer confirmation analysis

### SunVox Library (Already Included)
- **`sunvox_lib/sunvox_lib/headers/sunvox.h`** - C API header
- **`sunvox_lib/sunvox_lib/linux/lib_x86_64/sunvox.so`** - Linux library
- **`sunvox_lib/sunvox_lib/docs/readme.txt`** - Documentation
- **`sunvox_lib/sunvox_lib/examples/`** - Example code

## Quick Reference

### Current Blocker Summary (November 11, 2025)
**Problem**: SunVox Library v2.1.3 ARM64 has CoreAudio initialization bug
**Error**: 0x20103 (CoreAudio failure) on macOS ARM64
**Tests Run**: 6 different flag combinations, all failed
**Environments**: Both sandboxed (plugin) and non-sandboxed (terminal)
**Reproducibility**: 100% - fails every time on ARM64
**Action Needed**: Contact SunVox developer with bug report

### Building
```bash
./bundle.sh                          # Build and create CLAP bundle
cargo build --release                # Manual build
cargo check                          # Fast syntax check
cargo run --bin sunvox_standalone_test --release  # Test SunVox init (will fail on ARM64)
```

### Installing (for testing)
```bash
mkdir -p ~/.clap
cp -r target/release/sunvox_clap.clap ~/.clap/
# Plugin loads but SunVox init fails - generates test tone instead
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

## Testing Environments

### ⚠️ CRITICAL: Local vs Sandboxed Testing

This project requires testing in **two distinct environments** with different capabilities and limitations:

#### 🖥️ Local Testing Environment (User's Mac)

**What it is**:
- Native macOS terminal/shell environment
- Direct access to system audio hardware (CoreAudio)
- No sandbox restrictions
- Full file system access

**What can be tested**:
- ✅ Standalone SunVox initialization (`cargo run --bin sunvox_standalone_test`)
- ✅ Direct audio hardware access
- ✅ Real-world DAW plugin loading (Bitwig, Reaper, etc.)
- ✅ File loading and resource access
- ✅ Full system capabilities

**Required for**:
- Testing SunVox `sv_init()` outside plugin sandbox
- Validating audio hardware requirements
- Real DAW integration testing
- End-to-end user experience validation

**How AI Assistant helps**:
- Provide commands for user to run
- Interpret results from user-provided logs/output
- Cannot directly execute these tests

---

#### 🐳 Sandboxed/Container Testing Environment (CI/Docker)

**What it is**:
- Linux container environment (kernel 4.4.0, x86_64)
- **NO audio hardware access** (no ALSA, PulseAudio, JACK, CoreAudio)
- Restricted permissions and capabilities
- Automated testing environment

**What can be tested**:
- ✅ Build system (`cargo build --release`)
- ✅ Compilation and linking
- ✅ FFI bindings correctness
- ✅ Code syntax and structure
- ✅ Unit tests that don't require audio hardware
- ⚠️ Plugin loading (but audio init will fail)

**Limitations**:
- ❌ SunVox `sv_init()` will ALWAYS fail (no audio hardware)
- ❌ Cannot test real audio generation
- ❌ Cannot validate DAW integration
- ❌ Not representative of user environment

**Good for**:
- Rapid development iteration
- Build verification
- Code structure validation
- FFI binding correctness

---

#### Test Classification

When planning tests, always classify them:

**🖥️ LOCAL ONLY** - Requires user's Mac:
- Standalone SunVox initialization test
- Real DAW plugin loading
- Audio generation validation
- Hardware requirement validation

**🐳 SANDBOXED OK** - Can run in CI/container:
- Build and compilation
- Unit tests (with graceful audio init failure handling)
- FFI binding syntax checks
- Code linting and formatting

**⚠️ HYBRID** - Partial testing in both:
- Plugin structure (builds in sandbox, loads in local)
- Error handling (can verify graceful failure in sandbox)
- Fallback mechanisms (test tone generation)

---

#### Communication Protocol

When AI Assistant plans tests:
- **Always specify** which environment the test requires
- **Mark LOCAL ONLY tests** with ⚠️ LOCAL TESTING REQUIRED
- **Explain to user** what they need to run locally
- **Interpret results** from user-provided logs/output
- **Don't attempt** local-only tests in sandboxed environment

When User provides test results:
- **Include environment details** (macOS version, DAW name, etc.)
- **Copy full output** or relevant log snippets
- **Note any errors or warnings** from system console
- **Specify success/failure** clearly

---

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

### 🚨 CRITICAL BLOCKER: SunVox ARM64 Library Bug (COMPLETED TESTING)

**Test 6 Status**: ✅ **COMPLETE** - All testing finished November 11, 2025

**Results**: ❌ ALL 6 flag combinations FAILED on macOS ARM64
- Test 1: `flags = 0` (official example) → FAILED (0x20103)
- Test 2: `SV_INIT_FLAG_OFFLINE` → FAILED (0x20103)
- Test 3: `SV_INIT_FLAG_USER_AUDIO_CALLBACK` → FAILED (0x20103)
- Test 4: `USER_AUDIO_CALLBACK | OFFLINE` → FAILED (0x20103)
- Test 5: Full plugin flags → FAILED (0x20103)
- Test 6: Additional combinations → FAILED (0x20103)

**Conclusion**: SunVox Library v2.1.3 ARM64 has a CoreAudio initialization bug
**Environment Tested**: Non-sandboxed terminal application (full system access)
**Error**: Consistent 0x20103 across all tests

**See Complete Results**:
- `TESTING.md` - Test 6 section with full details
- `SUNVOX_ARM64_BUG_REPORT.md` - Comprehensive bug report

### ⚠️ IMMEDIATE ACTION REQUIRED

**Next Step**: Contact SunVox developer (NightRadio)

1. **Send bug report** to https://warmplace.ru/forum/ or developer email
   - Use `SUNVOX_ARM64_BUG_REPORT.md` as message content
   - Subject: "SunVox Library v2.1.3 - CoreAudio Failure on macOS ARM64"

2. **Follow timeline** in `NEXT_STEPS.md`:
   - Week 1-2: Wait for developer response
   - Week 3: Test Rosetta workaround if no response
   - Week 4: Make strategic decision (wait vs. alternative engine)

3. **Alternative options** documented in `NEXT_STEPS.md`:
   - Option A: Rosetta 2 translation (x86_64 library)
   - Option B: Out-of-process architecture
   - Option C: Pre-rendering approach
   - Option D: Alternative synthesis engine
   - Option E: Wait for library fix (recommended)

---

### Short Term (After Test 6)

**If Test 6 Succeeds**:
1. Test in Reaper or other DAW with less restrictive sandbox
2. Investigate workarounds (pre-rendering, out-of-process)
3. Contact SunVox developer with specific findings
4. Document hardware/sandbox requirements

**If Test 6 Fails**:
1. Verify audio hardware is functional on macOS
2. Test different flag combinations
3. Contact SunVox developer about macOS ARM64 compatibility
4. Evaluate alternative approaches

---

### Medium Term (If SunVox Works)

**Phase 2 Completion**:
1. Implement chosen workaround (if needed)
2. Add comprehensive error handling
3. Test multiple plugin instances
4. Verify no memory leaks or crashes
5. Document working configurations

**Cross-Platform**:
1. Test on Linux with real audio hardware (if available)
2. Add platform-specific initialization if needed
3. Document per-platform requirements

---

### Long Term (Post Phase 2)

**Feature Development**:
1. Add parameters (map to SunVox module controllers)
2. MIDI input support
3. Project file loading UI
4. GUI for project selection
5. Preset management
6. Multiple SunVox slots

**Production Readiness**:
1. Comprehensive testing across DAWs
2. Performance optimization
3. User documentation
4. Distribution/packaging

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
