# SunVox Library Bug Report: CoreAudio Initialization Failure on macOS ARM64

**Date**: November 11, 2025
**Reporter**: Mark Reveley
**Library Version**: SunVox Library v2.1.3 (October 19, 2025)
**Platform**: macOS (Darwin 24.4.0), ARM64 (Apple Silicon)
**Severity**: **CRITICAL** - Blocks all usage on macOS ARM64

---

## Summary

The SunVox library (v2.1.3) fails to initialize on macOS ARM64 (Apple Silicon) with error code `0x20103` (CoreAudio initialization failure). This occurs with **all flag combinations** including the official example configuration, in both sandboxed (plugin) and non-sandboxed (standalone app) environments.

---

## Environment Details

### System Information
- **Operating System**: macOS (Darwin 24.4.0)
- **Architecture**: ARM64 (aarch64) - Apple Silicon
- **Audio Hardware**: Functional (system audio works normally)
- **SunVox Library**:
  - File: `sunvox_lib/macos/lib_arm64/sunvox.dylib`
  - Version: 2.1.3 (October 19, 2025)
  - Size: 1,011,136 bytes
  - Architecture: Mach-O 64-bit ARM64

### Library Dependencies
```
sunvox.dylib (compatibility version 0.0.0, current version 0.0.0)
/usr/lib/libSystem.B.dylib
/usr/lib/libc++.1.dylib
/System/Library/Frameworks/Cocoa.framework
/System/Library/Frameworks/Carbon.framework
/System/Library/Frameworks/AudioUnit.framework
/System/Library/Frameworks/CoreAudio.framework
/System/Library/Frameworks/Foundation.framework
/usr/lib/libobjc.A.dylib
```

---

## Problem Description

### Error Code
```
sv_init() returns: 131331 (0x20103)
Meaning: CoreAudio initialization failure
```

### Reproducibility
- ✅ **100% reproducible** across all flag combinations
- ✅ **Consistent error code** in all cases
- ✅ **Occurs in both sandboxed and non-sandboxed environments**

---

## Test Results

### Test Environment 1: Standalone Application (Non-Sandboxed)

**Context**: Terminal application with full system access (no sandbox restrictions)

**Test Code**: Rust FFI calling `sv_init()` with various flag combinations

**Results**: ALL 6 configurations FAILED with identical error

#### Configuration 1: No Flags (Official Example)
```c
sv_init(NULL, 44100, 2, 0)  // Matches test1.c from examples/c/
```
**Result**: ❌ Error 0x20103

#### Configuration 2: OFFLINE Flag (Juce Plugin Example)
```c
sv_init(NULL, 44100, 2, SV_INIT_FLAG_OFFLINE)  // Matches 2021 Juce forum post
```
**Result**: ❌ Error 0x20103

#### Configuration 3: USER_AUDIO_CALLBACK Flag
```c
sv_init(NULL, 44100, 2, SV_INIT_FLAG_USER_AUDIO_CALLBACK)
```
**Result**: ❌ Error 0x20103

#### Configuration 4: USER_AUDIO_CALLBACK + OFFLINE
```c
sv_init(NULL, 44100, 2, SV_INIT_FLAG_USER_AUDIO_CALLBACK | SV_INIT_FLAG_OFFLINE)
```
**Result**: ❌ Error 0x20103

#### Configuration 5: Multiple Flags
```c
sv_init(NULL, 44100, 2,
    SV_INIT_FLAG_NO_DEBUG_OUTPUT | SV_INIT_FLAG_USER_AUDIO_CALLBACK |
    SV_INIT_FLAG_AUDIO_FLOAT32 | SV_INIT_FLAG_ONE_THREAD | SV_INIT_FLAG_OFFLINE)
```
**Result**: ❌ Error 0x20103

#### Configuration 6: Full Plugin Flags
```c
sv_init(NULL, 44100, 2,
    SV_INIT_FLAG_NO_DEBUG_OUTPUT | SV_INIT_FLAG_OFFLINE |
    SV_INIT_FLAG_AUDIO_FLOAT32 | SV_INIT_FLAG_ONE_THREAD)
```
**Result**: ❌ Error 0x20103

---

### Test Environment 2: CLAP Plugin (Sandboxed)

**Context**: Loading as CLAP plugin in Bitwig Studio

**Configuration**:
```c
sv_init(NULL, 44100, 2,
    SV_INIT_FLAG_NO_DEBUG_OUTPUT | SV_INIT_FLAG_OFFLINE |
    SV_INIT_FLAG_AUDIO_FLOAT32 | SV_INIT_FLAG_ONE_THREAD)
```

**Result**: ❌ Error 0x20103 (same as standalone)

**Debug Log Output**:
```
[1762457935] === SunVox Plugin Initialize START ===
[1762457935] Sample rate: 44100
[1762457935] Calling sv_init with flags: 27
[1762457935] ERROR: sv_init failed with code: 131331 (0x20103)
```

---

## Library Behavior Observed

When `sv_init()` fails, the library outputs:
```
SOUND: sundog_sound_deinit() begin
SOUND: sundog_sound_deinit() end
Max memory used: 88699
```

This indicates:
- ✅ Library loads successfully
- ✅ Partial initialization occurs (memory allocation: 88KB)
- ❌ CoreAudio initialization fails
- ✅ Cleanup/deinitialization works correctly

---

## Additional Evidence

### Official Examples Not ARM64-Ready

The official C examples in `sunvox_lib/examples/c/` have:

**File**: `MAKE_MACOS`
```bash
cp ../../macos/lib_x86_64/sunvox.dylib ./  # <-- Uses x86_64, not lib_arm64
make CFLAGS="-m64" LDFLAGS="-m64" LIBS="-lstdc++"
```

**Observation**: Build scripts only reference `lib_x86_64`, suggesting ARM64 support may be incomplete or untested.

### Working Configurations (Other Platforms)

For comparison, the same code works correctly on:
- ✅ Linux x86_64 with audio hardware (reported by NightRadio in 2021)
- ✅ macOS x86_64 (Intel) - assumed working based on examples
- ❓ Windows - not tested

---

## Expected vs. Actual Behavior

### Expected
According to SunVox documentation and forum posts:
- `SV_INIT_FLAG_OFFLINE` should allow initialization without audio device
- `SV_INIT_FLAG_USER_AUDIO_CALLBACK` should bypass system audio
- Plugins should be able to use SunVox in offline mode
- The developer confirmed this works in a 2021 Juce forum post

### Actual
- ALL initialization attempts fail with CoreAudio error
- Error occurs even with flags meant to bypass audio devices
- Error occurs in both plugin (sandboxed) and standalone (unrestricted) environments
- Official example configuration (`flags = 0`) also fails

---

## Impact

### Critical Issues
1. 🚫 **Impossible to create audio plugins** using SunVox on macOS ARM64
2. 🚫 **Impossible to create standalone apps** using SunVox on macOS ARM64
3. 🚫 **No workaround available** - this is a library-level bug
4. 🚫 **Affects latest library version** (v2.1.3) - not an old version issue

### User Impact
- Apple Silicon Mac users cannot use SunVox Library at all
- Plugin developers cannot target macOS ARM64
- Growing proportion of macOS users are on ARM64 hardware

---

## Technical Analysis

### Root Cause (Hypothesis)
The SunVox library's CoreAudio initialization code appears to:
1. Attempt to initialize CoreAudio hardware **regardless of flags**
2. Not properly check ARM64 architecture requirements
3. Fail with error 0x20103 when CoreAudio setup fails
4. Not fall back to offline-only mode even when requested

### Possible Issues
- ARM64-specific CoreAudio API usage error
- Missing ARM64 architecture checks
- Incorrect framework linking for ARM64
- Audio Unit instantiation incompatible with ARM64
- Hardcoded x86_64 assumptions in audio setup code

---

## Reproduction Steps

### Prerequisites
- macOS system with Apple Silicon (ARM64)
- SunVox Library v2.1.3 (from official download)
- C/C++ or Rust compiler

### Minimal Reproduction Code

```c
#include <stdio.h>
#include <dlfcn.h>

#define SUNVOX_MAIN
#include "sunvox.h"

int main() {
    if (sv_load_dll() != 0) {
        printf("Failed to load SunVox library\n");
        return 1;
    }

    // Try official example configuration
    int result = sv_init(NULL, 44100, 2, 0);

    if (result != 0) {
        printf("sv_init() failed with error: %d (0x%X)\n", result, result);
        sv_unload_dll();
        return 1;
    }

    printf("SUCCESS!\n");

    sv_deinit();
    sv_unload_dll();
    return 0;
}
```

**Expected**: `SUCCESS!`
**Actual**: `sv_init() failed with error: 131331 (0x20103)`

### Build Commands
```bash
# Copy ARM64 library
cp sunvox_lib/macos/lib_arm64/sunvox.dylib ./

# Compile
clang -o test test.c -I sunvox_lib/headers -L. -lsunvox -lstdc++

# Run
./test
```

---

## Request for Developer

### Questions
1. **Is ARM64 support fully implemented and tested?**
   - Official examples only reference x86_64
   - No ARM64 build scripts provided

2. **Is error 0x20103 a known issue on ARM64?**
   - This error is consistent across all configurations

3. **Should `SV_INIT_FLAG_USER_AUDIO_CALLBACK` bypass CoreAudio entirely?**
   - Currently it still triggers CoreAudio initialization and fails

4. **Are there any ARM64-specific initialization requirements?**
   - Different API calls needed?
   - Additional flags required?
   - Special configuration strings?

### Requested Actions
1. **Investigate CoreAudio initialization on ARM64**
   - Test on actual Apple Silicon hardware
   - Add ARM64 build scripts to examples
   - Document ARM64-specific requirements if any

2. **Fix or document the issue**
   - If fixable: Release updated library with ARM64 fix
   - If limitation: Document that ARM64 is not supported
   - If workaround exists: Update documentation with details

3. **Update examples for ARM64**
   - Add `MAKE_MACOS_ARM64` script
   - Test examples on Apple Silicon
   - Document any platform-specific differences

---

## Workarounds Attempted

All of the following were attempted **without success**:

❌ Different flag combinations (tried 6+ variations)
❌ Config string parameters
❌ Different sample rates (44100, 48000)
❌ Non-sandboxed environment (terminal app)
❌ Manual audio callback provision
❌ Offline mode flag
❌ Using official example configuration

**Conclusion**: No workaround exists - this requires a library fix.

---

## Files Available for Review

I can provide:
- Complete test source code (Rust FFI bindings)
- Binary standalone test application
- Debug logs from multiple test runs
- System diagnostics and library dependency info
- CLAP plugin source code demonstrating the issue

---

## Contact Information

**Reporter**: Mark Reveley
**Project**: CLAP audio plugin using SunVox Library
**Framework**: nih-plug (Rust)
**Repository**: Available on request

---

## Summary for Developer

**TL;DR**: SunVox Library v2.1.3 `sv_init()` fails with error 0x20103 on macOS ARM64 across ALL flag combinations (including official example config), in both sandboxed and non-sandboxed environments. CoreAudio initialization consistently fails. ARM64 support appears incomplete or broken. This blocks all SunVox usage on Apple Silicon Macs.

**Priority**: HIGH - Affects growing user base on ARM64 hardware

**Reproducibility**: 100% - Fails every time on ARM64, likely works on x86_64

**Urgency**: Critical for plugin and app developers targeting modern Macs

---

Thank you for your time investigating this issue. SunVox is an amazing synthesizer and we'd love to use it in plugins for macOS ARM64 users!
