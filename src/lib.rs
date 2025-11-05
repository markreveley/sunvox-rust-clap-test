use nih_plug::prelude::*;
use std::sync::Arc;

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
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Phase 2.3 complete: SunVox is now initialized
        // Phase 2.4 next: Call sv_audio_callback() here to generate audio

        // For now, still just passing audio through
        // Step 2.4 will add: sv_audio_callback(buffer, frames, latency, time)

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
