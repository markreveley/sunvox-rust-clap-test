# Next Steps: SunVox ARM64 Bug - Action Plan

**Date**: November 11, 2025
**Status**: SunVox Library v2.1.3 fails on macOS ARM64 - Library-level bug confirmed

---

## Immediate Actions

### 1. Contact SunVox Developer ⚠️ HIGH PRIORITY

**Who**: NightRadio (SunVox developer)
**Where**:
- 🌐 Forum: https://warmplace.ru/forum/
- 📧 Email: Contact form at https://warmplace.ru/soft/sunvox/
- 💬 Issue tracker (if available)

**What to send**:
- ✅ Attach `SUNVOX_ARM64_BUG_REPORT.md` (comprehensive bug report created)
- ✅ Reference your environment (macOS ARM64, v2.1.3)
- ✅ Mention you tested ALL flag combinations including official examples
- ✅ Emphasize error code 0x20103 (CoreAudio initialization failure)
- ✅ State this affects plugin development on Apple Silicon

**Expected response time**: Unknown (community/developer dependent)

---

### 2. Check for Updates or Known Issues

**Before contacting**, verify:

```bash
# Check SunVox Library download page for newer version
# https://warmplace.ru/soft/sunvox/sunvox_lib.php

# Check forum for existing ARM64 issues
# Search: "ARM64", "Apple Silicon", "M1", "M2", "CoreAudio", "0x20103"
```

**Currently**: You have v2.1.3 (October 19, 2025) - latest available

---

## Alternative Approaches

While waiting for library fix, consider these options:

### Option A: Rosetta 2 Translation (Immediate Workaround)

**Concept**: Use x86_64 library with Rosetta 2 translation

**Pros**:
- ✅ May work immediately
- ✅ No code changes needed
- ✅ Uses official Intel library

**Cons**:
- ❌ Performance penalty (translation overhead)
- ❌ Universal binary complications
- ❌ Still requires testing
- ❌ Not a long-term solution

**How to test**:
```bash
# Use x86_64 library instead of ARM64
cp sunvox_lib/sunvox_lib/macos/lib_x86_64/sunvox.dylib target/release/

# Build for x86_64 target
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# Test
arch -x86_64 ./target/x86_64-apple-darwin/release/sunvox_standalone_test
```

**Risk**: May still fail if DAW is ARM64-native

---

### Option B: Out-of-Process SunVox (Advanced Workaround)

**Concept**: Run SunVox in a separate process, communicate via IPC

**Architecture**:
```
Plugin (ARM64) <--IPC--> Helper Process (x86_64 + Rosetta)
     |                           |
     |                      SunVox Library
     |                           |
   [Audio] <-- Pipe/Socket -- [Generated Audio]
```

**Pros**:
- ✅ Might work with x86_64 library
- ✅ Plugin stays ARM64-native
- ✅ Isolates SunVox issues

**Cons**:
- ❌ Complex implementation
- ❌ Latency concerns (IPC overhead)
- ❌ Resource intensive (two processes)
- ❌ May still fail on ARM64

**Effort**: High (2-3 days development)

---

### Option C: Pre-Rendering Approach

**Concept**: Render SunVox audio offline, play back samples in plugin

**Pros**:
- ✅ No runtime SunVox dependency
- ✅ No latency issues
- ✅ Works everywhere

**Cons**:
- ❌ No real-time parameter changes
- ❌ Not truly generative
- ❌ Large sample library needed
- ❌ Not really using SunVox's strengths

**Use case**: Very limited, not recommended

---

### Option D: Alternative Synthesis Engine

**Concept**: Use different synthesis library that works on ARM64

**Possible alternatives**:
- **DaisySP** - Open-source DSP library (C++)
- **Surge XT** - Open-source synthesizer (library mode)
- **Vital** - Open-source wavetable synth (may have library mode)
- **Custom Rust synthesis** - Roll your own with `fundsp` or `dasp`

**Pros**:
- ✅ Known ARM64 compatibility
- ✅ Active development
- ✅ May have better plugin integration

**Cons**:
- ❌ Not SunVox (different sound/features)
- ❌ Requires redesign
- ❌ Learning curve for new library

**Effort**: High (1-2 weeks for new architecture)

---

### Option E: Wait for SunVox Fix (Recommended Short-Term)

**Concept**: Pause development, wait for library update

**Pros**:
- ✅ Zero effort
- ✅ Proper solution
- ✅ No workarounds needed
- ✅ Future-proof

**Cons**:
- ❌ Unknown timeline
- ❌ No guarantee of fix
- ❌ Project blocked

**Recommended if**:
- Developer responds quickly
- Fix is imminent
- No urgent deadline

---

## Decision Matrix

| Option | Effort | Success Likelihood | Performance | Long-term Viability |
|--------|--------|-------------------|-------------|---------------------|
| **A: Rosetta** | Low | Medium | Fair | Poor |
| **B: Out-of-Process** | High | Medium | Fair | Poor |
| **C: Pre-Render** | Medium | High | Good | Poor |
| **D: Alternative** | High | High | Excellent | Excellent |
| **E: Wait** | None | Unknown | N/A | Excellent |

---

## Recommended Path Forward

### Phase 1: Contact & Wait (1-2 weeks)
1. ✅ Send bug report to SunVox developer
2. ⏳ Wait for initial response
3. 🔍 Monitor forums for others with same issue
4. 🧪 Test any suggestions from developer

**Timeline**: Give developer 2 weeks to respond

---

### Phase 2: Quick Workaround Test (if no response)
If no response after 2 weeks:

1. **Test Rosetta approach** (Option A)
   - Try x86_64 library with Rosetta
   - If works: Ship with x86_64 + note about Rosetta
   - If fails: Proceed to Phase 3

**Timeline**: 1-2 days testing

---

### Phase 3: Strategic Decision (if Rosetta fails)
Choose between:

**Path 3A: Wait Longer**
- If SunVox is essential
- If developer acknowledged issue
- If fix is promised

**Path 3B: Alternative Engine**
- If timeline is critical
- If SunVox features not essential
- If control over synthesis important

**Timeline**: Evaluate after Phase 2 results

---

## Communication Templates

### Template: Initial Bug Report

**Subject**: SunVox Library v2.1.3 - CoreAudio Failure on macOS ARM64

```
Hello,

I'm developing a CLAP audio plugin using the SunVox Library and have encountered a critical bug on macOS ARM64 (Apple Silicon).

ISSUE: sv_init() consistently fails with error 0x20103 (CoreAudio initialization failure) on all ARM64 Macs, regardless of flag configuration.

I've tested:
- All flag combinations (including official example: flags=0)
- Both sandboxed (plugin) and non-sandboxed (standalone) environments
- Latest library version (v2.1.3, October 19, 2025)
- All tests fail with identical error code

ENVIRONMENT:
- macOS Darwin 24.4.0
- Architecture: ARM64 (Apple Silicon)
- Library: sunvox_lib/macos/lib_arm64/sunvox.dylib

I've prepared a comprehensive bug report with detailed test results, reproduction steps, and technical analysis. Please see attached SUNVOX_ARM64_BUG_REPORT.md.

Is ARM64 support fully implemented? Are there known issues or workarounds?

Thank you for your time!

Mark Reveley
```

---

### Template: Follow-up (1 week later)

**Subject**: Re: SunVox Library v2.1.3 - CoreAudio Failure on macOS ARM64

```
Hello,

Following up on my bug report from [DATE].

Has there been any progress investigating the ARM64 initialization issue?

I'm blocked on plugin development and need to decide whether to:
1. Wait for a library fix
2. Explore workarounds (Rosetta/x86_64)
3. Use an alternative synthesis engine

Any information on timeline or status would be greatly appreciated.

Thanks,
Mark
```

---

## Monitoring & Tracking

### What to track:
- [ ] Bug report sent date: ___________
- [ ] Initial response received: ___________
- [ ] Issue acknowledged: Yes / No
- [ ] Fix promised: Yes / No / Unknown
- [ ] Estimated fix timeline: ___________
- [ ] Workaround suggested: ___________
- [ ] Library update released: ___________

### Weekly check:
- [ ] Week 1: Check for response
- [ ] Week 2: Send follow-up if no response
- [ ] Week 3: Test Rosetta workaround if still blocked
- [ ] Week 4: Make strategic decision (wait vs. alternative)

---

## Technical Notes for Future

### If Rosetta Workaround Works

**Build configuration needed**:
```rust
// In build.rs, use x86_64 library even on ARM64
#[cfg(target_os = "macos")]
{
    // Always use x86_64 library (will run via Rosetta on ARM64)
    let lib_path = base_path.join("macos").join("lib_x86_64");
    // ...
}
```

**Documentation needed**:
- ⚠️ Note in README about Rosetta requirement
- ⚠️ Performance implications
- ⚠️ Temporary workaround status

---

### If Alternative Engine Chosen

**Evaluation criteria**:
1. ARM64 native support
2. Real-time synthesis capability
3. Plugin-friendly API
4. Rust bindings available or FFI-able
5. Active maintenance
6. License compatibility
7. Feature set (modular, MIDI, effects)

**Top candidates**:
- DaisySP (embedded DSP, very reliable)
- Custom Rust (fundsp + nih-plug, full control)
- Surge XT (if library mode available)

---

## Success Criteria

**Minimum Viable Product**:
- ✅ Plugin loads in DAW
- ✅ Generates audio (from any synthesis source)
- ✅ Works on macOS ARM64
- ✅ Acceptable latency
- ✅ Stable (no crashes)

**Ideal Solution**:
- ✅ All above, PLUS:
- ✅ Uses SunVox library
- ✅ Native ARM64 (no Rosetta)
- ✅ Full SunVox feature access
- ✅ Low CPU usage

---

## Resources

### SunVox Links
- Library home: https://warmplace.ru/soft/sunvox/sunvox_lib.php
- Main site: https://warmplace.ru/soft/sunvox/
- Forum: https://warmplace.ru/forum/

### Alternative Synthesis Libraries
- DaisySP: https://github.com/electro-smith/DaisySP
- fundsp: https://github.com/SamiPerttu/fundsp
- Surge XT: https://surge-synthesizer.github.io/
- Vital: https://vital.audio/

### Rust Audio Resources
- nih-plug: https://github.com/robbert-vdh/nih-plug
- rust-audio: https://github.com/rust-audio
- Audio Plugin Development Discord: (invite in nih-plug README)

---

## Summary

**Current status**: Blocked by SunVox ARM64 bug
**Next action**: Contact developer with bug report
**Fallback plan**: Rosetta workaround or alternative engine
**Timeline**: 2-4 weeks to resolution or decision

**Remember**: This is a library bug, not an implementation issue. Your code is correct. The plugin infrastructure works. Once SunVox initializes (or an alternative is chosen), development can proceed quickly.

---

**Last updated**: November 11, 2025
**Decision deadline**: Set based on project timeline
