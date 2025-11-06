use nih_plug::prelude::*;
use std::sync::Arc;
use std::ffi::CString;

// SunVox FFI bindings
mod sunvox_ffi;
use sunvox_ffi::*;

/// A CLAP plugin integrating SunVox modular synthesizer.
/// Phase 2: Now initializes SunVox for audio generation.
struct SunVoxPlugin {
    params: Arc<SunVoxPluginParams>,

    // SunVox state (Phase 2)
    sunvox_initialized: bool,
    sunvox_slot: i32,
    sample_rate: f32,
}

#[derive(Params)]
struct SunVoxPluginParams {}

impl Default for SunVoxPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(SunVoxPluginParams {}),
            sunvox_initialized: false,
            sunvox_slot: 0,
            sample_rate: 44100.0,
        }
    }
}

impl Plugin for SunVoxPlugin {
    const NAME: &'static str = "SunVox CLAP";
    const VENDOR: &'static str = "SunVox CLAP Plugin";
    const URL: &'static str = "https://warmplace.ru/soft/sunvox/";
    const EMAIL: &'static str = "";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // Unique identifier for this plugin
    // Generated from https://www.guidgenerator.com/
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            aux_input_ports: &[],
            aux_output_ports: &[],
            names: PortNames::const_default(),
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Store the sample rate from the host
        self.sample_rate = buffer_config.sample_rate;

        // Initialize SunVox in offline mode with float32 audio
        unsafe {
            let flags = SV_INIT_FLAG_NO_DEBUG_OUTPUT
                | SV_INIT_FLAG_OFFLINE
                | SV_INIT_FLAG_AUDIO_FLOAT32
                | SV_INIT_FLAG_ONE_THREAD;

            let result = sv_init(
                std::ptr::null(),
                buffer_config.sample_rate as i32,
                2, // stereo
                flags,
            );

            if result != 0 {
                nih_log!("⚠ SunVox initialization failed with code: {} (0x{:x})", result, result);
                nih_log!("⚠ This may be expected in some environments");
                nih_log!("⚠ Plugin will continue but audio generation will be disabled");
                self.sunvox_initialized = false;
                return true; // Still return true so plugin loads
            }

            nih_log!("✓ SunVox initialized successfully at {} Hz", buffer_config.sample_rate);

            // Open slot 0 for playback
            let result = sv_open_slot(self.sunvox_slot);
            if result != 0 {
                nih_log!("⚠ Failed to open SunVox slot: {}", result);
                sv_deinit();
                self.sunvox_initialized = false;
                return true; // Still return true so plugin loads
            }

            nih_log!("✓ SunVox slot {} opened", self.sunvox_slot);

            // Load an example SunVox project for testing
            // Using a simple test song from the SunVox library resources
            let project_path = CString::new("sunvox_lib/sunvox_lib/resources/song01.sunvox")
                .expect("CString creation failed");

            let result = sv_load(self.sunvox_slot, project_path.as_ptr());
            if result != 0 {
                nih_log!("⚠ Failed to load SunVox project: {}", result);
                nih_log!("⚠ Audio generation will be disabled");
                sv_close_slot(self.sunvox_slot);
                sv_deinit();
                self.sunvox_initialized = false;
                return true; // Still return true so plugin loads
            }

            nih_log!("✓ SunVox project loaded successfully");

            // Start playback
            let result = sv_play_from_beginning(self.sunvox_slot);
            if result != 0 {
                nih_log!("⚠ Failed to start SunVox playback: {}", result);
            } else {
                nih_log!("✓ SunVox playback started");
            }

            self.sunvox_initialized = true;
        }

        true
    }

    fn deactivate(&mut self) {
        // Clean up SunVox when plugin is deactivated
        if self.sunvox_initialized {
            unsafe {
                nih_log!("Cleaning up SunVox...");
                sv_close_slot(self.sunvox_slot);
                sv_deinit();
                nih_log!("✓ SunVox cleaned up");
            }
            self.sunvox_initialized = false;
        }
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Skip audio generation if SunVox is not initialized
        if !self.sunvox_initialized {
            return ProcessStatus::Normal;
        }

        // Generate audio from SunVox
        unsafe {
            let num_frames = buffer.samples() as i32;

            // Create interleaved buffer for SunVox (LRLRLR...)
            let mut sunvox_buffer = vec![0.0f32; (num_frames * 2) as usize];

            // Call SunVox to generate audio
            // Parameters: buffer, frames, latency (0 for simplicity), current tick
            let result = sv_audio_callback(
                sunvox_buffer.as_mut_ptr() as *mut std::os::raw::c_void,
                num_frames,
                0, // latency
                sv_get_ticks(),
            );

            // If SunVox generated audio (result == 1), copy to output
            if result == 1 {
                // Get output channels and de-interleave SunVox audio
                let channels = buffer.as_slice();
                for (channel_idx, channel) in channels.iter_mut().enumerate() {
                    for (sample_idx, sample) in channel.iter_mut().enumerate() {
                        // SunVox buffer is interleaved: LRLRLR...
                        // channel 0 = left, channel 1 = right
                        *sample = sunvox_buffer[sample_idx * 2 + channel_idx];
                    }
                }
            }
            // If result == 0, SunVox returned silence (buffer already zeroed)
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for SunVoxPlugin {
    const CLAP_ID: &'static str = "com.sunvox.clap-plugin";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A CLAP plugin integrating SunVox modular synthesizer");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument,
        ClapFeature::Synthesizer,
        ClapFeature::Stereo,
    ];
}

nih_export_clap!(SunVoxPlugin);
