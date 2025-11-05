use nih_plug::prelude::*;
use std::sync::Arc;

/// A minimal CLAP plugin that passes audio through unchanged.
/// This serves as the foundation for integrating SunVox in Phase 2.
struct SunVoxPlugin {
    params: Arc<SunVoxPluginParams>,
}

#[derive(Params)]
struct SunVoxPluginParams {}

impl Default for SunVoxPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(SunVoxPluginParams {}),
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

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // For now, just pass audio through unchanged
        // In Phase 2, we'll integrate SunVox audio generation here
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
