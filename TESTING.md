---
title: SunVox CLAP Plugin - Testing Results
purpose: |
  This document tracks all testing performed on the SunVox CLAP plugin across
  different environments and configurations. Each test includes the exact
  environment details, configuration used, and observed results.

  Use this document to:
  - Understand what has been tested and what hasn't
  - Compare results across different environments
  - Identify patterns in failures/successes
  - Avoid repeating failed approaches
  - Track progress toward working plugin

  When adding test results:
  - Include complete environment details (OS, hardware, virtualization)
  - Document exact command/configuration used
  - Include full error messages or success indicators
  - Note date and who performed the test
  - Link to relevant code commits if applicable
---

# SunVox CLAP Plugin - Testing Results

## Test Result Legend

- ✅ **Success**: Test passed completely
- ⚠️ **Partial**: Test partially succeeded with caveats
- ❌ **Failed**: Test failed
- 🔄 **In Progress**: Test underway or needs completion
- ⏭️ **Blocked**: Test blocked by dependencies
- ⏸️ **Skipped**: Test intentionally skipped

---

## Test 1: Standalone SunVox Init - Multiple Flags (Container)

**Date**: 2025-11-06
**Tester**: Claude AI (automated)
**Status**: ❌ Failed
**Environment**:
- **OS**: Linux (kernel 4.4.0)
- **Architecture**: x86_64
- **Audio Hardware**: None (container environment)
- **Virtualization**: Docker/container
- **Audio Drivers Available**: None (no ALSA, PulseAudio, JACK, OSS)

**Test Command**:
```bash
cargo run --bin sunvox_standalone_test --release
```

**Configuration**:
```rust
let flags = SV_INIT_FLAG_NO_DEBUG_OUTPUT
    | SV_INIT_FLAG_OFFLINE
    | SV_INIT_FLAG_AUDIO_FLOAT32
    | SV_INIT_FLAG_ONE_THREAD;
sv_init(std::ptr::null(), 44100, 2, flags)
```

**Results**:
```
Test 1: Initializing SunVox with OFFLINE mode
  Flags: SV_INIT_FLAG_OFFLINE | SV_INIT_FLAG_AUDIO_FLOAT32 | SV_INIT_FLAG_ONE_THREAD
  ❌ FAILURE: sv_init() returned -1 (0xFFFFFFFF)
```

**Error Messages**:
```
ALSA lib pcm.c:2721:(snd_pcm_open_noupdate) Unknown PCM pulse
ALSA lib pcm.c:2721:(snd_pcm_open_noupdate) Unknown PCM default
```

**Commit**: 02f831a
**Analysis**: SunVox attempts ALSA initialization despite OFFLINE flag. No audio devices present causes hard failure.

---

## Test 2: Standalone SunVox Init - OFFLINE Flag Only (Container)

**Date**: 2025-11-06
**Tester**: Claude AI (automated)
**Status**: ❌ Failed
**Environment**:
- **OS**: Linux (kernel 4.4.0)
- **Architecture**: x86_64
- **Audio Hardware**: None (container environment)
- **Virtualization**: Docker/container
- **Audio Drivers Available**: None

**Test Command**:
```bash
cargo run --bin sunvox_standalone_test --release
```

**Configuration** (matching NightRadio's Juce example):
```rust
let flags = SV_INIT_FLAG_OFFLINE;  // ONLY this flag
sv_init(0 as *const i8, 44100, 2, flags)
```

**Results**:
```
Test 1: Initializing SunVox with OFFLINE mode ONLY
  Flags: SV_INIT_FLAG_OFFLINE (no other flags!)
  This matches NightRadio's Juce plugin example from 2021
  ❌ FAILURE: sv_init() returned -1 (0xFFFFFFFF)
```

**Error Messages**:
```
Desired audio buffer size: 2048 frames
ALSA ERROR: Can't open audio device pulse: No such file or directory
ALSA ERROR: Can't open audio device default: No such file or directory
Switching to jack
JACK: Can't open libjack
Switching to oss
OSS ERROR: Can't open sound device
```

**Commit**: 0c22181
**Analysis**: Even with minimal flags (matching developer's example), SunVox still attempts to initialize multiple audio subsystems (ALSA → JACK → OSS), all fail without hardware.

**Key Insight**: `SV_INIT_FLAG_OFFLINE` prevents audio *streaming* but not audio subsystem *initialization*.

---

## Test 3: Plugin in Bitwig Studio (macOS ARM64)

**Date**: 2025-11-05
**Tester**: User (markreveley)
**Status**: ❌ Failed
**Environment**:
- **OS**: macOS (ARM64)
- **Architecture**: ARM64 (Apple Silicon)
- **Audio Hardware**: Present (Mac audio system)
- **DAW**: Bitwig Studio
- **Plugin Host Process**: BitwigPluginHost-ARM64-NEON
- **Sandbox**: Yes (DAW-imposed)

**Test Method**:
1. Built plugin with `./bundle.sh`
2. Installed to `~/.clap/`
3. Loaded in Bitwig Studio
4. Checked debug log at `/tmp/sunvox_plugin_debug.log`

**Configuration**:
```rust
let flags = SV_INIT_FLAG_NO_DEBUG_OUTPUT
    | SV_INIT_FLAG_OFFLINE
    | SV_INIT_FLAG_AUDIO_FLOAT32
    | SV_INIT_FLAG_ONE_THREAD;
sv_init(std::ptr::null(), buffer_config.sample_rate as i32, 2, flags)
```

**Results** (from debug log):
```
[1762457935] === SunVox Plugin Initialize START ===
[1762457935] Sample rate: 44100
[1762457935] Calling sv_init with flags: 27
[1762457935] ERROR: sv_init failed with code: 131331 (0x20103)
```

**Analysis**:
- Error code `0x20103` = CoreAudio initialization failure
- Audio hardware exists on system but sandbox blocks access
- Plugin host process lacks CoreAudio permissions
- Fallback to test tone (plugin generates sine wave successfully)

**Notable**: Plugin loads and functions, just can't initialize SunVox. Demonstrates graceful error handling works.

---

## Test 4: Plugin Unit Tests (Container)

**Date**: 2025-11-06 (and prior)
**Tester**: Claude AI (automated)
**Status**: ⚠️ Partial Success
**Environment**:
- **OS**: Linux (kernel 4.4.0)
- **Architecture**: x86_64
- **Audio Hardware**: None (container environment)
- **Virtualization**: Docker/container

**Test Command**:
```bash
cargo test --lib -- --nocapture
```

**Results**:
```
test sunvox_ffi::tests::test_sunvox_ffi_bindings ... ok
```

**Configuration**:
```rust
// Test gracefully handles init failure as expected
let result = unsafe { sv_init(std::ptr::null(), 44100, 2, flags) };
if result != 0 {
    println!("  ⚠ SunVox initialization failed (error {})", result);
    println!("  This is expected in containerized environments");
}
```

**Analysis**:
- FFI bindings work correctly (no linking errors)
- `sv_init()` fails as expected in container
- Test passes by gracefully handling the failure
- Validates our Rust FFI declarations are correct

---

## Test 5: Build System (Container)

**Date**: 2025-11-06
**Tester**: Claude AI (automated)
**Status**: ✅ Success
**Environment**:
- **OS**: Linux (kernel 4.4.0)
- **Architecture**: x86_64
- **Rust**: Stable toolchain

**Test Command**:
```bash
cargo build --release
./bundle.sh
```

**Results**:
- ✅ Compiles successfully
- ✅ Links to SunVox library correctly
- ✅ Creates CLAP bundle structure
- ✅ Proper binary output (~1.1 MB)
- ✅ All symbols present

**Analysis**: Build infrastructure and FFI setup are correct. No compilation or linking issues.

---

## Tests Still Needed

### High Priority Tests

#### Test 6: Standalone SunVox Init - Linux with Real Audio Hardware ⏸️
**Status**: Not yet tested
**Hypothesis**: Will succeed because audio devices exist
**Environment Needed**:
- Linux system with working audio (not containerized)
- ALSA/PulseAudio/JACK functional
- Physical or virtual audio devices present

**Expected Result**: `sv_init()` should return 0 (success)

**How to test**:
```bash
# On Linux system with audio:
cargo run --bin sunvox_standalone_test --release
```

---

#### Test 7: Standalone SunVox Init - macOS with Real Audio Hardware ⏸️
**Status**: Not yet tested
**Hypothesis**: Will succeed because CoreAudio accessible
**Environment Needed**:
- macOS system (Intel or Apple Silicon)
- Standard user environment (not sandboxed)
- CoreAudio functional

**Expected Result**: `sv_init()` should return 0 (success)

**How to test**:
```bash
# On macOS system:
cargo run --bin sunvox_standalone_test --release
```

---

#### Test 8: Plugin in Different DAWs (Real Hardware) ⏸️
**Status**: Not yet tested
**Priority**: HIGH
**Environments to test**:

1. **Bitwig Studio** (Linux with audio)
   - System with working audio hardware
   - Not containerized/virtualized
   - May still fail if sandbox blocks CoreAudio/ALSA

2. **Reaper** (Linux/macOS/Windows)
   - Known for less restrictive sandboxing
   - May have better plugin permissions

3. **Ableton Live** (macOS/Windows)
   - Check sandbox behavior

4. **Logic Pro** (macOS)
   - Apple's strict sandbox policies

**How to test**:
```bash
# Build and install
./bundle.sh
cp -r target/release/sunvox_clap.clap ~/.clap/

# Then load in DAW and check:
# - Does plugin appear in browser?
# - Does it load without errors?
# - Check logs for SunVox init success/failure
# - Does audio generate (SunVox) or fallback (test tone)?
```

---

### Medium Priority Tests

#### Test 9: SunVox with Config String ⏸️
**Status**: Not yet tested
**Hypothesis**: Config string might bypass audio init
**Configurations to try**:

```rust
// Test various config strings:
sv_init(c"audiodriver=none".as_ptr(), 44100, 2, SV_INIT_FLAG_OFFLINE);
sv_init(c"audiodriver=null".as_ptr(), 44100, 2, SV_INIT_FLAG_OFFLINE);
sv_init(c"buffer=0|audiodevice=none".as_ptr(), 44100, 2, SV_INIT_FLAG_OFFLINE);
```

**Expected Result**: Unlikely to work, but worth testing.

---

#### Test 10: Dynamic Loading (sv_load_dll) ⏸️
**Status**: Not yet tested
**Hypothesis**: Dynamic loading might behave differently
**Note**: NightRadio's Juce example uses `sv_load_dll()` / `sv_unload_dll()`

**Implementation needed**:
```rust
// Implement dynamic loading instead of static linking
sv_load_dll();
sv_init(...);
// ... use sunvox ...
sv_deinit();
sv_unload_dll();
```

---

### Low Priority Tests

#### Test 11: Different SunVox Library Versions ⏸️
**Status**: Not yet tested
**Current version**: Unknown (from sunvox_lib bundle)
**Action**: Check for updated SunVox library, test if behavior differs

---

#### Test 12: Windows Environment ⏸️
**Status**: Not yet tested
**Platform**: Windows (DirectSound/WASAPI)
**Hypothesis**: May have different audio init behavior

---

## Test Result Patterns

### Consistent Failures
1. ❌ Container environments (no audio hardware)
2. ❌ Strict sandboxes (audio API blocked)
3. ❌ Both minimal and full flag configurations

### Consistent Successes
1. ✅ Build and compilation
2. ✅ FFI bindings and linking
3. ✅ Plugin loads in DAW (even with SunVox init failure)
4. ✅ Graceful fallback (test tone generation)

### Unknowns (Need Testing)
1. ❓ Real Linux system with audio hardware
2. ❓ Real macOS system (non-sandboxed process)
3. ❓ Production DAW environments with audio present
4. ❓ Different DAW sandbox policies
5. ❓ Config string options
6. ❓ Dynamic loading approach

---

## Hypotheses to Validate

### Hypothesis 1: Audio Hardware Requirement
**Statement**: SunVox requires audio hardware to be ACCESSIBLE (even if unused in offline mode)

**Evidence Supporting**:
- ✅ NightRadio confirms plugins work (implying his test env had audio)
- ✅ All our failures are in environments without audio access
- ✅ Error messages show audio subsystem initialization attempts

**Evidence Against**:
- ❌ None yet

**Status**: Strong hypothesis, needs validation on real hardware

---

### Hypothesis 2: Sandbox Tolerance Varies
**Statement**: Some DAWs have looser sandboxes allowing audio API init even if not used

**Evidence Supporting**:
- ⚠️ macOS Bitwig failed (strict sandbox)
- ❓ Other DAWs not yet tested

**Evidence Against**:
- ❌ None yet

**Status**: Weak hypothesis, needs more DAW testing

---

### Hypothesis 3: Developer's Example Works Universally
**Statement**: NightRadio's exact approach (minimal flags) will work on any real system

**Evidence Supporting**:
- ✅ Developer confirmed it works
- ✅ Used in production (implied)

**Evidence Against**:
- ❌ Failed in our container test (but container not representative)
- ❌ Failed in macOS Bitwig sandbox

**Status**: Hypothesis partially disproven, needs refinement

**Refined**: Developer's approach works on systems WHERE AUDIO HARDWARE IS ACCESSIBLE

---

## Recommendations for Next Session

### Critical Path Tests (Must Do)
1. **Test standalone app on Linux with real audio** → Validates hypothesis #1
2. **Test standalone app on macOS (if available)** → Cross-platform validation
3. **Test plugin in Reaper (less restrictive)** → Validates hypothesis #2

### If Critical Tests Pass
4. Contact NightRadio with specific findings
5. Document hardware requirements
6. Continue plugin development with known constraints

### If Critical Tests Fail
4. Re-evaluate workarounds (pre-rendering, out-of-process)
5. Contact NightRadio with failure results
6. Consider alternative approaches

---

## Testing Environment Access

### Available Now
- ✅ Linux container (no audio)
- ✅ Build/compile environment
- ✅ Unit testing

### Need Access To
- ⏸️ Linux system with audio hardware
- ⏸️ macOS system with audio hardware
- ⏸️ Multiple DAW applications for testing
- ⏸️ Windows system (lower priority)

### Can Request User To Test
- 📧 User has macOS with Bitwig (tested, failed)
- 📧 User could test on other DAWs
- 📧 User could test standalone app locally
- 📧 User could try config string variations

---

**Last Updated**: 2025-11-06
**Next Test Session**: TBD (need real hardware access)
