// Standalone test application to verify SunVox library works outside plugin sandbox
// This tests whether the macOS sandbox restriction is the root cause of sv_init() failure

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};
use std::thread;
use std::time::Duration;

// SunVox FFI declarations (copied from sunvox_ffi.rs for standalone binary)
#[link(name = "sunvox")]
extern "C" {
    pub fn sv_init(config: *const c_char, freq: c_int, channels: c_int, flags: u32) -> c_int;
    pub fn sv_deinit() -> c_int;
    pub fn sv_audio_callback(buf: *mut c_void, frames: c_int, latency: c_int, out_time: u32) -> c_int;
    pub fn sv_open_slot(slot: c_int) -> c_int;
    pub fn sv_close_slot(slot: c_int) -> c_int;
    pub fn sv_load(slot: c_int, name: *const c_char) -> c_int;
    pub fn sv_play_from_beginning(slot: c_int) -> c_int;
    pub fn sv_stop(slot: c_int) -> c_int;
    pub fn sv_get_ticks() -> u32;
    pub fn sv_get_sample_rate() -> c_int;
}

// SunVox initialization flags
const SV_INIT_FLAG_NO_DEBUG_OUTPUT: u32 = 1 << 0;
const SV_INIT_FLAG_USER_AUDIO_CALLBACK: u32 = 1 << 1;
const SV_INIT_FLAG_AUDIO_INT16: u32 = 1 << 2;
const SV_INIT_FLAG_AUDIO_FLOAT32: u32 = 1 << 3;
const SV_INIT_FLAG_ONE_THREAD: u32 = 1 << 4;
const SV_INIT_FLAG_OFFLINE: u32 = 1 << 8;

fn main() {
    println!("==============================================");
    println!("SunVox Standalone Test - Sandbox Investigation");
    println!("==============================================\n");

    println!("Purpose: Test if SunVox works outside DAW plugin sandbox");
    println!("Platform: {}", std::env::consts::OS);
    println!("Arch: {}\n", std::env::consts::ARCH);

    let sample_rate = 44100;
    let channels = 2;
    let mut success = false;

    // Test 1: NO FLAGS (like the official C example test1.c)
    println!("Test 1: Initializing SunVox with NO FLAGS");
    println!("  Flags: 0 (none)");
    println!("  This matches the official SunVox C example test1.c");

    let flags_test1 = 0;

    let mut result = unsafe {
        sv_init(0 as *const i8, sample_rate, channels, flags_test1)
    };

    if result == 0 {
        println!("  ✅ SUCCESS!");
        success = true;
    } else {
        println!("  ❌ FAILURE: sv_init() returned {} (0x{:X})", result, result);
        if result == 131331 || result == 0x20103 {
            println!("     (CoreAudio initialization blocked)");
        }

        // Clean up partial initialization
        unsafe { sv_deinit(); }
    }

    // Test 2: With OFFLINE mode (like NightRadio's Juce plugin example)
    if !success {
        println!("\nTest 2: OFFLINE mode only");
        println!("  Flags: SV_INIT_FLAG_OFFLINE");
        println!("  This matches NightRadio's Juce plugin example from 2021");

        let flags_test2 = SV_INIT_FLAG_OFFLINE;

        result = unsafe {
            sv_init(0 as *const i8, sample_rate, channels, flags_test2)
        };

        if result == 0 {
            println!("  ✅ SUCCESS!");
            success = true;
        } else {
            println!("  ❌ FAILURE: sv_init() returned {} (0x{:X})", result, result);
            unsafe { sv_deinit(); }
        }
    }

    // Test 3: With USER_AUDIO_CALLBACK flag only
    if !success {
        println!("\nTest 3: USER_AUDIO_CALLBACK only");
        println!("  Flags: SV_INIT_FLAG_USER_AUDIO_CALLBACK");
        println!("  User provides audio callback, no system audio device");

        let flags_test3 = SV_INIT_FLAG_USER_AUDIO_CALLBACK;

        result = unsafe {
            sv_init(0 as *const i8, sample_rate, channels, flags_test3)
        };

        if result == 0 {
            println!("  ✅ SUCCESS!");
            success = true;
        } else {
            println!("  ❌ FAILURE: sv_init() returned {} (0x{:X})", result, result);
            unsafe { sv_deinit(); }
        }
    }

    // Test 4: With USER_AUDIO_CALLBACK + OFFLINE
    if !success {
        println!("\nTest 4: USER_AUDIO_CALLBACK + OFFLINE");
        println!("  Flags: SV_INIT_FLAG_USER_AUDIO_CALLBACK | SV_INIT_FLAG_OFFLINE");

        let flags_test4 = SV_INIT_FLAG_USER_AUDIO_CALLBACK | SV_INIT_FLAG_OFFLINE;

        result = unsafe {
            sv_init(0 as *const i8, sample_rate, channels, flags_test4)
        };

        if result == 0 {
            println!("  ✅ SUCCESS!");
            success = true;
        } else {
            println!("  ❌ FAILURE: sv_init() returned {} (0x{:X})", result, result);
            unsafe { sv_deinit(); }
        }
    }

    // Test 5: With full plugin flags (like our plugin uses)
    if !success {
        println!("\nTest 5: Full plugin flags");
        println!("  Flags: NO_DEBUG_OUTPUT | USER_AUDIO_CALLBACK | AUDIO_FLOAT32 | ONE_THREAD | OFFLINE");

        let flags_test5 = SV_INIT_FLAG_NO_DEBUG_OUTPUT
            | SV_INIT_FLAG_USER_AUDIO_CALLBACK
            | SV_INIT_FLAG_AUDIO_FLOAT32
            | SV_INIT_FLAG_ONE_THREAD
            | SV_INIT_FLAG_OFFLINE;

        result = unsafe {
            sv_init(0 as *const i8, sample_rate, channels, flags_test5)
        };

        if result == 0 {
            println!("  ✅ SUCCESS!");
            success = true;
        } else {
            println!("  ❌ FAILURE: sv_init() returned {} (0x{:X})", result, result);
            unsafe { sv_deinit(); }
        }
    }

    if !success {
        println!("\n==============================================");
        println!("❌ ALL TESTS FAILED");
        println!("==============================================");
        println!("SunVox cannot initialize on this system with any flag combination.");
        println!("\nPossible reasons:");
        println!("1. macOS ARM64 compatibility issue with SunVox library");
        println!("2. System audio drivers not accessible");
        println!("3. Library version incompatibility");
        println!("\nRecommended action: Contact SunVox developer (NightRadio)");
        return;
    }

    println!("\n==============================================");
    println!("✅ INITIALIZATION SUCCESSFUL!");
    println!("==============================================\n");

    // Test 3: Get sample rate
    println!("\nTest 3: Getting sample rate");
    let sr = unsafe { sv_get_sample_rate() };
    println!("  Sample rate: {} Hz", sr);

    // Test 4: Open a slot
    println!("\nTest 4: Opening SunVox slot 0");
    let slot = 0;
    let result = unsafe { sv_open_slot(slot) };
    if result == 0 {
        println!("  ✅ Slot opened successfully");
    } else {
        println!("  ❌ Failed to open slot: {}", result);
    }

    // Test 5: Load a project
    println!("\nTest 5: Loading SunVox project");
    let project_path = "sunvox_lib/sunvox_lib/resources/song01.sunvox";
    println!("  Project: {}", project_path);

    let path_cstring = CString::new(project_path).expect("CString creation failed");
    let result = unsafe { sv_load(slot, path_cstring.as_ptr()) };

    if result == 0 {
        println!("  ✅ Project loaded successfully");
    } else {
        println!("  ❌ Failed to load project: {}", result);
        println!("  Note: File may not exist at this path");
    }

    // Test 6: Start playback
    println!("\nTest 6: Starting playback");
    let result = unsafe { sv_play_from_beginning(slot) };
    if result == 0 {
        println!("  ✅ Playback started");
    } else {
        println!("  ❌ Failed to start playback: {}", result);
    }

    // Test 7: Generate some audio
    println!("\nTest 7: Generating audio (5 buffers)");
    let buffer_size = 512;
    let mut buffer = vec![0.0f32; buffer_size * 2]; // Stereo

    for i in 0..5 {
        let result = unsafe {
            sv_audio_callback(
                buffer.as_mut_ptr() as *mut c_void,
                buffer_size as i32,
                0,
                sv_get_ticks()
            )
        };

        if result == 1 {
            // Calculate RMS to see if we have audio
            let rms: f32 = buffer.iter()
                .map(|&x| x * x)
                .sum::<f32>()
                .sqrt() / buffer.len() as f32;

            println!("  Buffer {}: ✅ Generated (RMS: {:.6})", i + 1, rms);
        } else {
            println!("  Buffer {}: ❌ Failed (returned {})", i + 1, result);
        }

        thread::sleep(Duration::from_millis(10));
    }

    // Test 8: Stop playback
    println!("\nTest 8: Stopping playback");
    let result = unsafe { sv_stop(slot) };
    println!("  Result: {}", result);

    // Test 9: Cleanup
    println!("\nTest 9: Cleanup");
    unsafe {
        sv_close_slot(slot);
        println!("  ✅ Slot closed");

        sv_deinit();
        println!("  ✅ SunVox deinitialized");
    }

    println!("\n==============================================");
    println!("CONCLUSION:");
    println!("==============================================");
    println!("If this test succeeded outside the DAW but fails");
    println!("inside the plugin, then the DAW sandbox is the");
    println!("root cause of the initialization failure.");
    println!("\nNext steps:");
    println!("1. Compare these results with plugin host logs");
    println!("2. Contact SunVox developer with findings");
    println!("3. Consider workarounds (pre-rendering, IPC, etc.)");
    println!("==============================================\n");
}
