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

    // Test 1: Basic initialization with OFFLINE mode ONLY (like NightRadio's example)
    println!("Test 1: Initializing SunVox with OFFLINE mode ONLY");
    println!("  Flags: SV_INIT_FLAG_OFFLINE (no other flags!)");
    println!("  This matches NightRadio's Juce plugin example from 2021");

    let flags = SV_INIT_FLAG_OFFLINE;

    let sample_rate = 44100;
    let channels = 2;

    // Try Test 1a: NULL config (standard approach)
    println!("\n  Test 1a: NULL config");
    let result = unsafe {
        sv_init(0 as *const i8, sample_rate, channels, flags)
    };

    if result == 0 {
        println!("  ✅ SUCCESS: sv_init() returned 0");
    } else {
        println!("  ❌ FAILURE: sv_init() returned {} (0x{:X})", result, result);
        println!("\n  Error code 0x{:X} indicates:", result);
        if result == 131331 || result == 0x20103 {
            println!("  - Audio hardware access failure");
            println!("  - CoreAudio initialization blocked");
            println!("  - Likely sandbox/permissions issue");
        }
        println!("\nExiting - cannot continue without successful initialization");
        return;
    }

    // Test 2: Get sample rate
    println!("\nTest 2: Getting sample rate");
    let sr = unsafe { sv_get_sample_rate() };
    println!("  Sample rate: {} Hz", sr);

    // Test 3: Open a slot
    println!("\nTest 3: Opening SunVox slot 0");
    let slot = 0;
    let result = unsafe { sv_open_slot(slot) };
    if result == 0 {
        println!("  ✅ Slot opened successfully");
    } else {
        println!("  ❌ Failed to open slot: {}", result);
    }

    // Test 4: Load a project
    println!("\nTest 4: Loading SunVox project");
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

    // Test 5: Start playback
    println!("\nTest 5: Starting playback");
    let result = unsafe { sv_play_from_beginning(slot) };
    if result == 0 {
        println!("  ✅ Playback started");
    } else {
        println!("  ❌ Failed to start playback: {}", result);
    }

    // Test 6: Generate some audio
    println!("\nTest 6: Generating audio (5 buffers)");
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

    // Test 7: Stop playback
    println!("\nTest 7: Stopping playback");
    let result = unsafe { sv_stop(slot) };
    println!("  Result: {}", result);

    // Test 8: Cleanup
    println!("\nTest 8: Cleanup");
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
