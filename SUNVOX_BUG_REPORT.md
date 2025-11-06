# Bug Report for SunVox Developer

## Contact Information
**Developer**: Alexander Zolotov (NightRadio)
**Email**: nightradio@gmail.com
**Website**: https://warmplace.ru/soft/sunvox/

---

## Email Template

**Subject**: SunVox Library: Question about SV_INIT_FLAG_OFFLINE in sandboxed plugin environments

---

Dear Alexander,

I am developing a CLAP audio plugin using the SunVox library, following your guidance from the 2021 Juce forum thread where you confirmed this is possible. I have questions about `SV_INIT_FLAG_OFFLINE` behavior in strict sandbox environments.

### Issue Summary

When initializing SunVox with `SV_INIT_FLAG_OFFLINE`, the library still attempts to access system audio hardware (CoreAudio on macOS, ALSA on Linux). This causes initialization to fail in sandboxed environments like audio plugin hosts, where direct hardware access is restricted.

### Technical Details

**Platform tested**:
- macOS (ARM64) - Bitwig Studio plugin host
- Linux (x86_64) - Container environment

**Initialization code**:
```c
int flags = SV_INIT_FLAG_NO_DEBUG_OUTPUT
    | SV_INIT_FLAG_OFFLINE
    | SV_INIT_FLAG_AUDIO_FLOAT32
    | SV_INIT_FLAG_ONE_THREAD;

int result = sv_init(NULL, 44100, 2, flags);
```

**Error results**:
- macOS: Returns error code `131331` (0x20103) - CoreAudio access failure
- Linux: Returns `-1` with ALSA errors attempting to open PCM devices

**Observed behavior**:
Even with `SV_INIT_FLAG_OFFLINE` set, SunVox attempts to initialize the platform audio system, which fails when:
1. No audio hardware is present
2. Process lacks audio hardware permissions (sandbox restrictions)
3. Running in plugin host environments where DAW manages all audio I/O

### Expected Behavior

Based on the documentation for `SV_INIT_FLAG_OFFLINE`:
> "Audio callback should be called manually"
> "Internal audio stream will NOT be created"

I expected that offline mode would:
- Skip all audio hardware initialization
- Not attempt to open or enumerate audio devices
- Allow initialization in any process, regardless of audio permissions
- Work purely in-memory with manual `sv_audio_callback()` calls

### Use Case

Audio plugins (VST, AU, CLAP, LV2) run in sandboxed host processes that:
- Do not have direct audio hardware access
- Receive pre-allocated audio buffers from the DAW
- Cannot initialize system audio APIs
- Must work in restricted security contexts

This is a standard requirement for all modern plugin formats across all platforms.

### Test Results

I created a standalone test application to isolate the issue. Full details in attached investigation document.

**Key findings**:
1. Issue occurs on both macOS and Linux
2. Happens in both standalone apps (no hardware) and plugin hosts (sandboxed)
3. Same behavior regardless of other flag combinations
4. Plugin infrastructure works perfectly - only `sv_init()` fails

### Reference: Your 2021 Juce Forum Post

In October 2021, you confirmed that SunVox works in plugins:
> "It is definitely possible :)"

You provided example code:
```c
sv_init( 0, sample_rate, 2, SV_INIT_FLAG_OFFLINE );
```

I followed this guidance, but encountered issues in containerized/sandboxed environments.

### Questions

1. **Does SV_INIT_FLAG_OFFLINE require audio hardware to exist?**
   - Even if the hardware isn't actively used by SunVox?
   - Will it work if hardware exists but access is blocked by sandbox?

2. **Have you tested SunVox plugins in production DAWs with strict sandboxes?**
   - Bitwig Studio
   - Ableton Live
   - Logic Pro
   - Do these work, or only less restricted environments?

3. **Is there a way to initialize SunVox with zero audio device access?**
   - Truly hardware-independent initialization?
   - Config string option like `audiodriver=none`?
   - For containerized/embedded/sandboxed environments?

4. **Are there requirements for OFFLINE mode that weren't documented?**
   - Must audio subsystem initialization succeed?
   - Are there platform-specific differences?

### Potential Solution

A truly hardware-independent initialization mode would enable:
- Audio plugin integration (VST/AU/CLAP/LV2)
- Embedded systems without audio hardware
- Server-side audio rendering
- Automated testing in CI/CD environments
- Any sandboxed/restricted process context

This would expand SunVox library's use cases significantly while maintaining all existing functionality for standard applications.

### Workarounds Considered

If no fix is possible, I'm considering:
1. **Pre-rendering**: Render SunVox projects offline, bundle audio files (loses interactivity)
2. **Out-of-process**: Run SunVox in separate helper process, IPC audio data (complex architecture)
3. **Mock audio device**: Create virtual device to satisfy init requirements (platform-specific, uncertain)

However, a fix in SunVox library would be far superior and benefit all developers.

### Repository & Test Code

I can provide:
- Complete test application demonstrating the issue
- Detailed investigation document with findings
- Example plugin code showing integration attempt
- Test results on multiple platforms

Available at: [provide GitHub repo link if desired]

### Thank You

Thank you for creating and maintaining SunVox - it's an incredible synthesizer and the library integration would be amazing if we can solve this initialization issue. I appreciate any guidance you can provide.

Best regards,

[Your Name]
[Your Contact Info]

---

## Attachments to Include

1. **SUNVOX_INIT_INVESTIGATION.md** - Detailed technical findings
2. **src/bin/standalone_test.rs** - Test application source code
3. **Test output logs** - Showing errors on both platforms

## Additional Resources

**SunVox Library Page**: https://warmplace.ru/soft/sunvox/sunvox_lib.php
**Forum**: https://warmplace.ru/forum/ (consider posting here as well)
**GitHub Issues**: Check if there's an official GitHub repo for bug tracking

## Notes

- Be polite and constructive - Alexander maintains SunVox as a passion project
- Emphasize the potential for expanding SunVox's reach into plugin development
- Offer to test any proposed fixes or workarounds
- Be patient - he may be busy with other projects
- Consider posting on the WarmPlace forum as well for community input

---

**Prepared**: November 6, 2025
**Status**: Ready to send
