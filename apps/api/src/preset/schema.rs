/// Vital preset parameter mapping and validation.
///
/// Gemini outputs normalized 0.0–1.0 values using simplified parameter names.
/// This module maps those to real Vital synthesizer keys and value ranges.
///
/// The real Init.vital template is loaded from `init_template.json` (174KB).
/// Parameters live under a `"settings"` key in the preset JSON.

/// The real Init.vital preset JSON, embedded at compile time.
pub const INIT_TEMPLATE_JSON: &str = include_str!("init_template.json");

// ── Parameter Mapping ─────────────────────────────────────────────────────────
//
// Each entry maps:
//   gemini_name  → what Gemini outputs (simplified, normalized 0–1)
//   vital_key    → real key inside the "settings" object
//   convert      → function to convert 0.0–1.0 → real Vital range

pub struct ParamMapping {
    pub gemini_name: &'static str,
    pub vital_key: &'static str,
    pub convert: fn(f64) -> f64,
}

// ── Conversion functions ──────────────────────────────────────────────────────

/// Pass through: 0–1 stays 0–1.
fn identity(v: f64) -> f64 {
    v
}

/// Bipolar: 0–1 → -1.0 to 1.0
fn bipolar(v: f64) -> f64 {
    (v * 2.0) - 1.0
}

/// Semitones: 0–1 → -48 to 48
fn semitones_48(v: f64) -> f64 {
    (v * 96.0) - 48.0
}

/// Unison voices: 0–1 → 1 to 16 (rounded)
fn unison_voices(v: f64) -> f64 {
    (v * 15.0 + 1.0).round()
}

/// Unison detune: 0–1 → 0.0 to 10.0
fn unison_detune(v: f64) -> f64 {
    v * 10.0
}

/// Filter cutoff (MIDI note): 0–1 → 8.0 to 136.0
fn midi_cutoff(v: f64) -> f64 {
    v * 128.0 + 8.0
}

/// Filter blend: 0–1 → 0.0 to 2.0 (0=LP, 1=BP, 2=HP)
fn filter_blend(v: f64) -> f64 {
    v * 2.0
}

/// Envelope time: Vital uses ~0.0–1.0 internally for envelope segments,
/// but values up to ~4.0 are valid for very long envelopes.
/// Map 0–1 → 0.0 to 4.0.
fn env_time(v: f64) -> f64 {
    v * 4.0
}

/// Envelope power (curve shape): 0–1 → -4.0 to 4.0
/// 0 = linear, negative = log, positive = exponential
fn env_power(v: f64) -> f64 {
    (v * 8.0) - 4.0
}

/// Delay frequency: 0–1 → -6.0 to 6.0 (tempo-synced divisions)
fn delay_freq(v: f64) -> f64 {
    (v * 12.0) - 6.0
}

/// LFO frequency: 0–1 → -8.0 to 8.0 (log scale Hz)
fn lfo_freq(v: f64) -> f64 {
    (v * 16.0) - 8.0
}

/// Osc level: Vital uses 0.0–1.0 where 0.707 = -3dB default.
/// Keep as identity since Gemini 0–1 maps well here.
fn osc_level(v: f64) -> f64 {
    v
}

// ── The Mapping Table ─────────────────────────────────────────────────────────

pub const PARAM_MAPPINGS: &[ParamMapping] = &[
    // ── Oscillator 1 ──────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "osc_1_wave_frame",    vital_key: "osc_1_wave_frame",    convert: identity },
    ParamMapping { gemini_name: "osc_1_level",         vital_key: "osc_1_level",         convert: osc_level },
    ParamMapping { gemini_name: "osc_1_pan",           vital_key: "osc_1_pan",           convert: bipolar },
    ParamMapping { gemini_name: "osc_1_tune",          vital_key: "osc_1_tune",          convert: bipolar },
    ParamMapping { gemini_name: "osc_1_transpose",     vital_key: "osc_1_transpose",     convert: semitones_48 },
    ParamMapping { gemini_name: "osc_1_unison_voices", vital_key: "osc_1_unison_voices", convert: unison_voices },
    ParamMapping { gemini_name: "osc_1_unison_detune", vital_key: "osc_1_unison_detune", convert: unison_detune },
    ParamMapping { gemini_name: "osc_1_phase",         vital_key: "osc_1_phase",         convert: identity },

    // ── Oscillator 2 ──────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "osc_2_wave_frame",    vital_key: "osc_2_wave_frame",    convert: identity },
    ParamMapping { gemini_name: "osc_2_level",         vital_key: "osc_2_level",         convert: osc_level },
    ParamMapping { gemini_name: "osc_2_pan",           vital_key: "osc_2_pan",           convert: bipolar },
    ParamMapping { gemini_name: "osc_2_tune",          vital_key: "osc_2_tune",          convert: bipolar },
    ParamMapping { gemini_name: "osc_2_transpose",     vital_key: "osc_2_transpose",     convert: semitones_48 },
    ParamMapping { gemini_name: "osc_2_unison_voices", vital_key: "osc_2_unison_voices", convert: unison_voices },
    ParamMapping { gemini_name: "osc_2_unison_detune", vital_key: "osc_2_unison_detune", convert: unison_detune },
    ParamMapping { gemini_name: "osc_2_phase",         vital_key: "osc_2_phase",         convert: identity },

    // ── Filter 1 ──────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "filter_1_cutoff",     vital_key: "filter_1_cutoff",     convert: midi_cutoff },
    ParamMapping { gemini_name: "filter_1_resonance",  vital_key: "filter_1_resonance",  convert: identity },
    ParamMapping { gemini_name: "filter_1_drive",      vital_key: "filter_1_drive",      convert: identity },
    ParamMapping { gemini_name: "filter_1_blend",      vital_key: "filter_1_blend",      convert: filter_blend },

    // ── Envelope 1 (amplitude) ────────────────────────────────────────────────
    ParamMapping { gemini_name: "env_1_attack",        vital_key: "env_1_attack",        convert: env_time },
    ParamMapping { gemini_name: "env_1_decay",         vital_key: "env_1_decay",         convert: env_time },
    ParamMapping { gemini_name: "env_1_sustain",       vital_key: "env_1_sustain",       convert: identity },
    ParamMapping { gemini_name: "env_1_release",       vital_key: "env_1_release",       convert: env_time },

    // ── Envelope 2 (modulation) ───────────────────────────────────────────────
    ParamMapping { gemini_name: "env_2_attack",        vital_key: "env_2_attack",        convert: env_time },
    ParamMapping { gemini_name: "env_2_decay",         vital_key: "env_2_decay",         convert: env_time },
    ParamMapping { gemini_name: "env_2_sustain",       vital_key: "env_2_sustain",       convert: identity },
    ParamMapping { gemini_name: "env_2_release",       vital_key: "env_2_release",       convert: env_time },

    // ── LFO 1 ─────────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "lfo_1_frequency",     vital_key: "lfo_1_frequency",     convert: lfo_freq },
    ParamMapping { gemini_name: "lfo_1_phase",         vital_key: "lfo_1_phase",         convert: identity },
    ParamMapping { gemini_name: "lfo_1_fade_time",     vital_key: "lfo_1_fade_time",     convert: identity },
    ParamMapping { gemini_name: "lfo_1_delay_time",    vital_key: "lfo_1_delay_time",    convert: identity },

    // ── Reverb ────────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "reverb_dry_wet",      vital_key: "reverb_dry_wet",      convert: identity },
    ParamMapping { gemini_name: "reverb_decay_time",   vital_key: "reverb_decay_time",   convert: identity },
    ParamMapping { gemini_name: "reverb_size",         vital_key: "reverb_size",         convert: identity },

    // ── Delay ─────────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "delay_dry_wet",       vital_key: "delay_dry_wet",       convert: identity },
    ParamMapping { gemini_name: "delay_feedback",      vital_key: "delay_feedback",      convert: identity },
    ParamMapping { gemini_name: "delay_frequency",     vital_key: "delay_frequency",     convert: delay_freq },

    // ── Chorus ────────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "chorus_dry_wet",      vital_key: "chorus_dry_wet",      convert: identity },
    ParamMapping { gemini_name: "chorus_feedback",     vital_key: "chorus_feedback",     convert: identity },

    // ── Distortion ────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "distortion_drive",    vital_key: "distortion_drive",    convert: identity },
    ParamMapping { gemini_name: "distortion_mix",      vital_key: "distortion_mix",      convert: identity },
];

/// Look up a parameter mapping by Gemini output name.
pub fn find_mapping(gemini_name: &str) -> Option<&'static ParamMapping> {
    PARAM_MAPPINGS
        .iter()
        .find(|m| m.gemini_name == gemini_name)
}

// ── Effect auto-on rules ──────────────────────────────────────────────────────
//
// When Gemini sets a nonzero wet mix for an effect, we auto-enable it.
// Format: (vital_wet_key, vital_on_key)

pub const EFFECT_AUTO_ON: &[(&str, &str)] = &[
    ("reverb_dry_wet",    "reverb_on"),
    ("delay_dry_wet",     "delay_on"),
    ("chorus_dry_wet",    "chorus_on"),
    ("distortion_drive",  "distortion_on"),
];
