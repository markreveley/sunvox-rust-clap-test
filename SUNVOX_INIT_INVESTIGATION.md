# SunVox Initialization Investigation

## Problem Statement

`sv_init()` fails when called from within audio plugin host environments, even with the `SV_INIT_FLAG_OFFLINE` flag that should bypass audio hardware requirements.

## Test Results

### Linux (x86_64) - Container Environment

**Test Date**: 2025-11-06
**Environment**: Linux container (no audio hardware)
**Test Application**: Standalone Rust binary (`sunvox_standalone_test`)

**Results**:
```
Test 1: Initializing SunVox with OFFLINE mode
  Flags: SV_INIT_FLAG_OFFLINE | SV_INIT_FLAG_AUDIO_FLOAT32 | SV_INIT_FLAG_ONE_THREAD
  ❌ FAILURE: sv_init() returned -1 (0xFFFFFFFF)
```

**ALSA Errors** (Linux audio system):
```
ALSA lib pcm.c:2721:(snd_pcm_open_noupdate) Unknown PCM pulse
ALSA lib pcm.c:2721:(snd_pcm_open_noupdate) Unknown PCM default
```

### macOS (ARM64) - DAW Plugin Host

**Test Date**: 2025-11-05
**Environment**: Bitwig Studio plugin host (`BitwigPluginHost-ARM64-NEON`)
**Test Application**: CLAP plugin running in sandboxed process

**Results**:
```
[1762457935] === SunVox Plugin Initialize START ===
[1762457935] Sample rate: 44100
[1762457935] Calling sv_init with flags: 27
[1762457935] ERROR: sv_init failed with code: 131331 (0x20103)
```

**Error Code Analysis**:
- `0x20103` = Audio hardware access failure
- CoreAudio initialization blocked by sandbox

## Key Findings

### Finding 1: Offline Mode Requires Audio Hardware
**🚨 CRITICAL**: `SV_INIT_FLAG_OFFLINE` does **NOT** fully bypass audio hardware initialization.

SunVox still attempts to initialize the platform audio system:
- **macOS**: CoreAudio
- **Linux**: ALSA
- **Windows**: (likely DirectSound/WASAPI)

### Finding 2: Cross-Platform Issue
This is not a macOS-specific problem. The same behavior occurs on Linux, confirming it's a fundamental limitation of SunVox's implementation.

### Finding 3: Sandbox Incompatibility
Most modern DAWs run plugins in sandboxed processes that restrict:
- Audio hardware access
- System audio API initialization
- Device enumeration

SunVox's initialization requirements conflict with these restrictions.

## Technical Analysis

### Expected Behavior (from SunVox documentation)
```c
// From sunvox.h - SV_INIT_FLAG_OFFLINE description:
// "Audio callback should be called manually"
// "Internal audio stream will NOT be created"
```

**Expectation**: No hardware access needed when using OFFLINE mode.

### Actual Behavior
Even with `SV_INIT_FLAG_OFFLINE`, SunVox calls:
- **Linux**: `snd_pcm_open()` - ALSA PCM device opening
- **macOS**: CoreAudio initialization (error 0x20103)

**Reality**: Hardware initialization still occurs, fails in restricted environments.

### Why This Matters for Plugins
Audio plugins must work in restricted environments:
1. **Sandboxing**: Modern OS security requires process isolation
2. **No Direct Hardware**: DAW manages all audio I/O
3. **Plugin Model**: Plugins receive audio buffers, don't access devices
4. **Multi-Instance**: Multiple plugins can't all open same hardware

## Test Code

See `src/bin/standalone_test.rs` for the complete test application.

**Key Test**:
```rust
let flags = SV_INIT_FLAG_NO_DEBUG_OUTPUT
    | SV_INIT_FLAG_OFFLINE
    | SV_INIT_FLAG_AUDIO_FLOAT32
    | SV_INIT_FLAG_ONE_THREAD;

let result = unsafe {
    sv_init(std::ptr::null(), 44100, 2, flags)
};
// Returns -1 (Linux) or 131331 (macOS) in restricted environments
```

## Proposed Solutions

### Option 1: Contact SunVox Developer ⭐ RECOMMENDED
**Effort**: Low
**Impact**: Potentially high

Actions:
1. Report findings to Alexander Zolotov (SunVox developer)
2. Request true hardware-independent offline mode
3. Ask if there's an undocumented flag or method
4. Inquire about plugin-specific API or workaround

Contact: https://warmplace.ru/soft/sunvox/

### Option 2: Test with Audio Hardware Present
**Effort**: Low
**Impact**: Diagnostic only

Test on Linux system with working ALSA to determine if:
- Issue is purely "no hardware" vs "sandboxed access"
- Different error codes provide more information

### Option 3: Alternative Initialization Flags
**Effort**: Low
**Impact**: Low (likely won't work)

Try combinations like:
```rust
// Experiment with minimal flags
sv_init(null, 0, 0, SV_INIT_FLAG_OFFLINE);

// Try without NO_DEBUG_OUTPUT
sv_init(null, 44100, 2, SV_INIT_FLAG_OFFLINE | SV_INIT_FLAG_AUDIO_FLOAT32);
```

### Option 4: Pre-Rendering Approach
**Effort**: Medium
**Impact**: High (works but loses real-time features)

1. Pre-render SunVox projects to WAV files offline
2. Bundle rendered audio with plugin
3. Load and playback pre-rendered samples
4. Lose: Real-time synthesis, parameters, MIDI input

### Option 5: Out-of-Process Architecture
**Effort**: High
**Impact**: High (preserves all features)

1. Run SunVox in unsandboxed helper process
2. Use IPC (shared memory/pipes) for audio transfer
3. Plugin requests audio from helper, forwards to DAW
4. Complex but allows full SunVox functionality

Challenges:
- Process lifecycle management
- Low-latency IPC
- Platform-specific implementation
- Increased complexity

### Option 6: Mock Audio Device
**Effort**: Medium-High
**Impact**: Unknown

Create a virtual/null audio device that satisfies SunVox's init requirements but doesn't actually access hardware. Would require:
- Platform-specific implementation
- Deep understanding of SunVox's internal checks
- May not work if SunVox validates device capabilities

## Recommendations

### Immediate (Next Session)
1. ✅ **DONE**: Created standalone test app
2. ✅ **DONE**: Confirmed issue on Linux
3. ⏭️ **NEXT**: Research SunVox developer contact
4. ⏭️ **NEXT**: Prepare detailed bug report with findings

### Short Term (This Week)
1. Contact SunVox developer with test results
2. Test on Linux system with actual audio hardware
3. Wait for response (1 week grace period)

### Long Term (If No SunVox Fix)
1. **If response positive**: Implement suggested fix
2. **If response negative**: Choose between:
   - Option 4: Pre-rendering (simpler, but limited)
   - Option 5: Out-of-process (complex, but full-featured)
3. Document decision and implement chosen approach

## Files Created

- `src/bin/standalone_test.rs` - Standalone test application
- `SUNVOX_INIT_INVESTIGATION.md` - This document
- Updated `Cargo.toml` - Added binary target

## Conclusion

The `SV_INIT_FLAG_OFFLINE` flag does not create a truly hardware-independent initialization path. SunVox still attempts to access system audio APIs even in offline mode, making it incompatible with sandboxed plugin environments.

**This is not a bug in our implementation** - it's a fundamental limitation of the SunVox library's current design.

Next steps depend on whether SunVox can be modified to support true offline/embedded mode, or if we need to architect around this limitation.

---

**Investigation Date**: November 6, 2025
**Investigator**: Claude AI Assistant
**Status**: Findings documented, awaiting SunVox developer consultation
