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
    /// Minimum valid Vital value after conversion (post-conversion clamp).
    pub min: f64,
    /// Maximum valid Vital value after conversion (post-conversion clamp).
    pub max: f64,
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

/// Discrete int range 0–3 (e.g. filter_style, delay_style, lfo_sync_type)
fn int_range_4(v: f64) -> f64 {
    (v * 3.0).round()
}

/// Discrete int range 0–5 (e.g. filter_model)
fn int_range_6(v: f64) -> f64 {
    (v * 5.0).round()
}

/// Discrete int range 0–7 (e.g. distortion_type)
fn int_range_8(v: f64) -> f64 {
    (v * 7.0).round()
}

/// Discrete int range 0–11 (e.g. osc_distortion_type, spectral_morph_type)
fn int_range_12(v: f64) -> f64 {
    (v * 11.0).round()
}

/// Detune range: 0–1 → 0.0 to 10.0
fn detune_range(v: f64) -> f64 {
    v * 10.0
}

/// Tempo-synced rate: 0–1 → 0.0 to 12.0
fn tempo(v: f64) -> f64 {
    v * 12.0
}

/// Reverb shelf gain: 0–1 → -6.0 to 6.0
fn shelf_gain(v: f64) -> f64 {
    (v * 12.0) - 6.0
}

/// EQ band gain: 0–1 → -15.0 to 15.0
fn eq_gain(v: f64) -> f64 {
    (v * 30.0) - 15.0
}

/// Compressor band gain: 0–1 → -6.0 to 30.0
fn compressor_gain(v: f64) -> f64 {
    (v * 36.0) - 6.0
}

/// Phaser mod depth: 0–1 → 0.0 to 48.0
fn phaser_mod_depth(v: f64) -> f64 {
    v * 48.0
}

/// Polyphony: 0–1 → 1 to 32 (rounded)
fn polyphony(v: f64) -> f64 {
    (v * 31.0 + 1.0).round()
}

/// Chorus voices: 0–1 → 1 to 4 (rounded). Vital crashes with ≥ 5 voices.
fn chorus_voices(v: f64) -> f64 {
    (v * 3.0 + 1.0).round()
}

// ── The Mapping Table ─────────────────────────────────────────────────────────

pub const PARAM_MAPPINGS: &[ParamMapping] = &[
    // ── Oscillator 1 ──────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "osc_1_wave_frame",            vital_key: "osc_1_wave_frame",            convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_1_level",                 vital_key: "osc_1_level",                 convert: osc_level,     min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_1_pan",                   vital_key: "osc_1_pan",                   convert: bipolar,       min: -1.0,  max: 1.0 },
    ParamMapping { gemini_name: "osc_1_tune",                  vital_key: "osc_1_tune",                  convert: bipolar,       min: -1.0,  max: 1.0 },
    ParamMapping { gemini_name: "osc_1_transpose",             vital_key: "osc_1_transpose",             convert: semitones_48,  min: -48.0, max: 48.0 },
    ParamMapping { gemini_name: "osc_1_unison_voices",         vital_key: "osc_1_unison_voices",         convert: unison_voices, min: 1.0,   max: 16.0 },
    ParamMapping { gemini_name: "osc_1_unison_detune",         vital_key: "osc_1_unison_detune",         convert: unison_detune, min: 0.0,   max: 10.0 },
    ParamMapping { gemini_name: "osc_1_unison_blend",          vital_key: "osc_1_unison_blend",          convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_1_phase",                 vital_key: "osc_1_phase",                 convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_1_random_phase",          vital_key: "osc_1_random_phase",          convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_1_distortion_amount",     vital_key: "osc_1_distortion_amount",     convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_1_distortion_phase",      vital_key: "osc_1_distortion_phase",      convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_1_distortion_type",       vital_key: "osc_1_distortion_type",       convert: int_range_12,  min: 0.0,   max: 11.0 },
    ParamMapping { gemini_name: "osc_1_frame_spread",          vital_key: "osc_1_frame_spread",          convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_1_spectral_morph_amount", vital_key: "osc_1_spectral_morph_amount", convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_1_spectral_morph_type",   vital_key: "osc_1_spectral_morph_type",   convert: int_range_12,  min: 0.0,   max: 11.0 },
    ParamMapping { gemini_name: "osc_1_stereo_spread",         vital_key: "osc_1_stereo_spread",         convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_1_detune_range",          vital_key: "osc_1_detune_range",          convert: detune_range,  min: 0.0,   max: 10.0 },

    // ── Oscillator 2 ──────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "osc_2_wave_frame",            vital_key: "osc_2_wave_frame",            convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_2_level",                 vital_key: "osc_2_level",                 convert: osc_level,     min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_2_pan",                   vital_key: "osc_2_pan",                   convert: bipolar,       min: -1.0,  max: 1.0 },
    ParamMapping { gemini_name: "osc_2_tune",                  vital_key: "osc_2_tune",                  convert: bipolar,       min: -1.0,  max: 1.0 },
    ParamMapping { gemini_name: "osc_2_transpose",             vital_key: "osc_2_transpose",             convert: semitones_48,  min: -48.0, max: 48.0 },
    ParamMapping { gemini_name: "osc_2_unison_voices",         vital_key: "osc_2_unison_voices",         convert: unison_voices, min: 1.0,   max: 16.0 },
    ParamMapping { gemini_name: "osc_2_unison_detune",         vital_key: "osc_2_unison_detune",         convert: unison_detune, min: 0.0,   max: 10.0 },
    ParamMapping { gemini_name: "osc_2_unison_blend",          vital_key: "osc_2_unison_blend",          convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_2_phase",                 vital_key: "osc_2_phase",                 convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_2_random_phase",          vital_key: "osc_2_random_phase",          convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_2_distortion_amount",     vital_key: "osc_2_distortion_amount",     convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_2_distortion_phase",      vital_key: "osc_2_distortion_phase",      convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_2_distortion_type",       vital_key: "osc_2_distortion_type",       convert: int_range_12,  min: 0.0,   max: 11.0 },
    ParamMapping { gemini_name: "osc_2_frame_spread",          vital_key: "osc_2_frame_spread",          convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_2_spectral_morph_amount", vital_key: "osc_2_spectral_morph_amount", convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_2_spectral_morph_type",   vital_key: "osc_2_spectral_morph_type",   convert: int_range_12,  min: 0.0,   max: 11.0 },
    ParamMapping { gemini_name: "osc_2_stereo_spread",         vital_key: "osc_2_stereo_spread",         convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_2_detune_range",          vital_key: "osc_2_detune_range",          convert: detune_range,  min: 0.0,   max: 10.0 },

    // ── Oscillator 3 ──────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "osc_3_wave_frame",            vital_key: "osc_3_wave_frame",            convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_3_level",                 vital_key: "osc_3_level",                 convert: osc_level,     min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_3_pan",                   vital_key: "osc_3_pan",                   convert: bipolar,       min: -1.0,  max: 1.0 },
    ParamMapping { gemini_name: "osc_3_tune",                  vital_key: "osc_3_tune",                  convert: bipolar,       min: -1.0,  max: 1.0 },
    ParamMapping { gemini_name: "osc_3_transpose",             vital_key: "osc_3_transpose",             convert: semitones_48,  min: -48.0, max: 48.0 },
    ParamMapping { gemini_name: "osc_3_unison_voices",         vital_key: "osc_3_unison_voices",         convert: unison_voices, min: 1.0,   max: 16.0 },
    ParamMapping { gemini_name: "osc_3_unison_detune",         vital_key: "osc_3_unison_detune",         convert: unison_detune, min: 0.0,   max: 10.0 },
    ParamMapping { gemini_name: "osc_3_unison_blend",          vital_key: "osc_3_unison_blend",          convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_3_phase",                 vital_key: "osc_3_phase",                 convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_3_random_phase",          vital_key: "osc_3_random_phase",          convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_3_distortion_amount",     vital_key: "osc_3_distortion_amount",     convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_3_distortion_phase",      vital_key: "osc_3_distortion_phase",      convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_3_distortion_type",       vital_key: "osc_3_distortion_type",       convert: int_range_12,  min: 0.0,   max: 11.0 },
    ParamMapping { gemini_name: "osc_3_frame_spread",          vital_key: "osc_3_frame_spread",          convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_3_spectral_morph_amount", vital_key: "osc_3_spectral_morph_amount", convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_3_spectral_morph_type",   vital_key: "osc_3_spectral_morph_type",   convert: int_range_12,  min: 0.0,   max: 11.0 },
    ParamMapping { gemini_name: "osc_3_stereo_spread",         vital_key: "osc_3_stereo_spread",         convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "osc_3_detune_range",          vital_key: "osc_3_detune_range",          convert: detune_range,  min: 0.0,   max: 10.0 },

    // ── Filter 1 ──────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "filter_1_cutoff",    vital_key: "filter_1_cutoff",    convert: midi_cutoff,   min: 8.0,   max: 136.0 },
    ParamMapping { gemini_name: "filter_1_resonance", vital_key: "filter_1_resonance", convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "filter_1_drive",     vital_key: "filter_1_drive",     convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "filter_1_blend",     vital_key: "filter_1_blend",     convert: filter_blend,  min: 0.0,   max: 2.0 },
    ParamMapping { gemini_name: "filter_1_mix",       vital_key: "filter_1_mix",       convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "filter_1_keytrack",  vital_key: "filter_1_keytrack",  convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "filter_1_model",     vital_key: "filter_1_model",     convert: int_range_6,   min: 0.0,   max: 5.0 },
    ParamMapping { gemini_name: "filter_1_style",     vital_key: "filter_1_style",     convert: int_range_4,   min: 0.0,   max: 3.0 },

    // ── Filter 2 ──────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "filter_2_cutoff",    vital_key: "filter_2_cutoff",    convert: midi_cutoff,   min: 8.0,   max: 136.0 },
    ParamMapping { gemini_name: "filter_2_resonance", vital_key: "filter_2_resonance", convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "filter_2_drive",     vital_key: "filter_2_drive",     convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "filter_2_blend",     vital_key: "filter_2_blend",     convert: filter_blend,  min: 0.0,   max: 2.0 },
    ParamMapping { gemini_name: "filter_2_mix",       vital_key: "filter_2_mix",       convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "filter_2_keytrack",  vital_key: "filter_2_keytrack",  convert: identity,      min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "filter_2_model",     vital_key: "filter_2_model",     convert: int_range_6,   min: 0.0,   max: 5.0 },
    ParamMapping { gemini_name: "filter_2_style",     vital_key: "filter_2_style",     convert: int_range_4,   min: 0.0,   max: 3.0 },

    // ── Envelope 1 (amplitude) ────────────────────────────────────────────────
    ParamMapping { gemini_name: "env_1_attack",        vital_key: "env_1_attack",        convert: env_time,  min: 0.0,  max: 4.0 },
    ParamMapping { gemini_name: "env_1_decay",         vital_key: "env_1_decay",         convert: env_time,  min: 0.0,  max: 4.0 },
    ParamMapping { gemini_name: "env_1_sustain",       vital_key: "env_1_sustain",       convert: identity,  min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "env_1_release",       vital_key: "env_1_release",       convert: env_time,  min: 0.0,  max: 4.0 },
    ParamMapping { gemini_name: "env_1_attack_power",  vital_key: "env_1_attack_power",  convert: env_power, min: -4.0, max: 4.0 },
    ParamMapping { gemini_name: "env_1_decay_power",   vital_key: "env_1_decay_power",   convert: env_power, min: -4.0, max: 4.0 },
    ParamMapping { gemini_name: "env_1_release_power", vital_key: "env_1_release_power", convert: env_power, min: -4.0, max: 4.0 },
    ParamMapping { gemini_name: "env_1_delay",         vital_key: "env_1_delay",         convert: env_time,  min: 0.0,  max: 4.0 },
    ParamMapping { gemini_name: "env_1_hold",          vital_key: "env_1_hold",          convert: env_time,  min: 0.0,  max: 4.0 },

    // ── Envelope 2 (modulation) ───────────────────────────────────────────────
    ParamMapping { gemini_name: "env_2_attack",        vital_key: "env_2_attack",        convert: env_time,  min: 0.0,  max: 4.0 },
    ParamMapping { gemini_name: "env_2_decay",         vital_key: "env_2_decay",         convert: env_time,  min: 0.0,  max: 4.0 },
    ParamMapping { gemini_name: "env_2_sustain",       vital_key: "env_2_sustain",       convert: identity,  min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "env_2_release",       vital_key: "env_2_release",       convert: env_time,  min: 0.0,  max: 4.0 },
    ParamMapping { gemini_name: "env_2_attack_power",  vital_key: "env_2_attack_power",  convert: env_power, min: -4.0, max: 4.0 },
    ParamMapping { gemini_name: "env_2_decay_power",   vital_key: "env_2_decay_power",   convert: env_power, min: -4.0, max: 4.0 },
    ParamMapping { gemini_name: "env_2_release_power", vital_key: "env_2_release_power", convert: env_power, min: -4.0, max: 4.0 },
    ParamMapping { gemini_name: "env_2_delay",         vital_key: "env_2_delay",         convert: env_time,  min: 0.0,  max: 4.0 },
    ParamMapping { gemini_name: "env_2_hold",          vital_key: "env_2_hold",          convert: env_time,  min: 0.0,  max: 4.0 },

    // ── Envelope 3 ────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "env_3_attack",        vital_key: "env_3_attack",        convert: env_time,  min: 0.0,  max: 4.0 },
    ParamMapping { gemini_name: "env_3_decay",         vital_key: "env_3_decay",         convert: env_time,  min: 0.0,  max: 4.0 },
    ParamMapping { gemini_name: "env_3_sustain",       vital_key: "env_3_sustain",       convert: identity,  min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "env_3_release",       vital_key: "env_3_release",       convert: env_time,  min: 0.0,  max: 4.0 },
    ParamMapping { gemini_name: "env_3_attack_power",  vital_key: "env_3_attack_power",  convert: env_power, min: -4.0, max: 4.0 },
    ParamMapping { gemini_name: "env_3_decay_power",   vital_key: "env_3_decay_power",   convert: env_power, min: -4.0, max: 4.0 },
    ParamMapping { gemini_name: "env_3_release_power", vital_key: "env_3_release_power", convert: env_power, min: -4.0, max: 4.0 },
    ParamMapping { gemini_name: "env_3_delay",         vital_key: "env_3_delay",         convert: env_time,  min: 0.0,  max: 4.0 },
    ParamMapping { gemini_name: "env_3_hold",          vital_key: "env_3_hold",          convert: env_time,  min: 0.0,  max: 4.0 },

    // ── LFO 1 ─────────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "lfo_1_frequency",  vital_key: "lfo_1_frequency",  convert: lfo_freq,    min: -8.0, max: 8.0 },
    ParamMapping { gemini_name: "lfo_1_phase",      vital_key: "lfo_1_phase",      convert: identity,    min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "lfo_1_fade_time",  vital_key: "lfo_1_fade_time",  convert: identity,    min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "lfo_1_delay_time", vital_key: "lfo_1_delay_time", convert: identity,    min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "lfo_1_stereo",     vital_key: "lfo_1_stereo",     convert: identity,    min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "lfo_1_tempo",      vital_key: "lfo_1_tempo",      convert: tempo,       min: 0.0,  max: 12.0 },
    ParamMapping { gemini_name: "lfo_1_sync_type",  vital_key: "lfo_1_sync_type",  convert: int_range_4, min: 0.0,  max: 3.0 },

    // ── LFO 2 ─────────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "lfo_2_frequency",  vital_key: "lfo_2_frequency",  convert: lfo_freq,    min: -8.0, max: 8.0 },
    ParamMapping { gemini_name: "lfo_2_phase",      vital_key: "lfo_2_phase",      convert: identity,    min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "lfo_2_fade_time",  vital_key: "lfo_2_fade_time",  convert: identity,    min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "lfo_2_delay_time", vital_key: "lfo_2_delay_time", convert: identity,    min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "lfo_2_stereo",     vital_key: "lfo_2_stereo",     convert: identity,    min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "lfo_2_tempo",      vital_key: "lfo_2_tempo",      convert: tempo,       min: 0.0,  max: 12.0 },
    ParamMapping { gemini_name: "lfo_2_sync_type",  vital_key: "lfo_2_sync_type",  convert: int_range_4, min: 0.0,  max: 3.0 },

    // ── LFO 3 ─────────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "lfo_3_frequency",  vital_key: "lfo_3_frequency",  convert: lfo_freq,    min: -8.0, max: 8.0 },
    ParamMapping { gemini_name: "lfo_3_phase",      vital_key: "lfo_3_phase",      convert: identity,    min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "lfo_3_fade_time",  vital_key: "lfo_3_fade_time",  convert: identity,    min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "lfo_3_delay_time", vital_key: "lfo_3_delay_time", convert: identity,    min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "lfo_3_stereo",     vital_key: "lfo_3_stereo",     convert: identity,    min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "lfo_3_tempo",      vital_key: "lfo_3_tempo",      convert: tempo,       min: 0.0,  max: 12.0 },
    ParamMapping { gemini_name: "lfo_3_sync_type",  vital_key: "lfo_3_sync_type",  convert: int_range_4, min: 0.0,  max: 3.0 },

    // ── Reverb ────────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "reverb_dry_wet",          vital_key: "reverb_dry_wet",          convert: identity,    min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "reverb_decay_time",       vital_key: "reverb_decay_time",       convert: identity,    min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "reverb_size",             vital_key: "reverb_size",             convert: identity,    min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "reverb_chorus_amount",    vital_key: "reverb_chorus_amount",    convert: identity,    min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "reverb_chorus_frequency", vital_key: "reverb_chorus_frequency", convert: lfo_freq,    min: -8.0,  max: 8.0 },
    ParamMapping { gemini_name: "reverb_delay",            vital_key: "reverb_delay",            convert: identity,    min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "reverb_high_shelf_cutoff",vital_key: "reverb_high_shelf_cutoff",convert: midi_cutoff, min: 8.0,   max: 136.0 },
    ParamMapping { gemini_name: "reverb_high_shelf_gain",  vital_key: "reverb_high_shelf_gain",  convert: shelf_gain,  min: -6.0,  max: 6.0 },
    ParamMapping { gemini_name: "reverb_low_shelf_cutoff", vital_key: "reverb_low_shelf_cutoff", convert: midi_cutoff, min: 8.0,   max: 136.0 },
    ParamMapping { gemini_name: "reverb_low_shelf_gain",   vital_key: "reverb_low_shelf_gain",   convert: shelf_gain,  min: -6.0,  max: 6.0 },
    ParamMapping { gemini_name: "reverb_pre_high_cutoff",  vital_key: "reverb_pre_high_cutoff",  convert: midi_cutoff, min: 8.0,   max: 136.0 },
    ParamMapping { gemini_name: "reverb_pre_low_cutoff",   vital_key: "reverb_pre_low_cutoff",   convert: midi_cutoff, min: 8.0,   max: 136.0 },

    // ── Delay ─────────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "delay_dry_wet",       vital_key: "delay_dry_wet",       convert: identity,    min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "delay_feedback",      vital_key: "delay_feedback",      convert: identity,    min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "delay_frequency",     vital_key: "delay_frequency",     convert: delay_freq,  min: -6.0,  max: 6.0 },
    ParamMapping { gemini_name: "delay_filter_cutoff", vital_key: "delay_filter_cutoff", convert: midi_cutoff, min: 8.0,   max: 136.0 },
    ParamMapping { gemini_name: "delay_filter_spread", vital_key: "delay_filter_spread", convert: identity,    min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "delay_style",         vital_key: "delay_style",         convert: int_range_4, min: 0.0,   max: 3.0 },
    ParamMapping { gemini_name: "delay_tempo",         vital_key: "delay_tempo",         convert: tempo,       min: 0.0,   max: 12.0 },

    // ── Chorus ────────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "chorus_dry_wet",   vital_key: "chorus_dry_wet",   convert: identity,      min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "chorus_feedback",  vital_key: "chorus_feedback",  convert: identity,      min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "chorus_cutoff",    vital_key: "chorus_cutoff",    convert: midi_cutoff,   min: 8.0,  max: 136.0 },
    ParamMapping { gemini_name: "chorus_frequency", vital_key: "chorus_frequency", convert: lfo_freq,      min: -8.0, max: 8.0 },
    ParamMapping { gemini_name: "chorus_mod_depth", vital_key: "chorus_mod_depth", convert: identity,      min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "chorus_spread",    vital_key: "chorus_spread",    convert: identity,      min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "chorus_voices",    vital_key: "chorus_voices",    convert: chorus_voices, min: 1.0,  max: 4.0 },
    ParamMapping { gemini_name: "chorus_tempo",     vital_key: "chorus_tempo",     convert: tempo,         min: 0.0,  max: 12.0 },

    // ── Distortion ────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "distortion_drive",            vital_key: "distortion_drive",            convert: identity,     min: 0.0, max: 1.0 },
    ParamMapping { gemini_name: "distortion_mix",              vital_key: "distortion_mix",              convert: identity,     min: 0.0, max: 1.0 },
    ParamMapping { gemini_name: "distortion_type",             vital_key: "distortion_type",             convert: int_range_8,  min: 0.0, max: 7.0 },
    ParamMapping { gemini_name: "distortion_filter_cutoff",    vital_key: "distortion_filter_cutoff",    convert: midi_cutoff,  min: 8.0, max: 136.0 },
    ParamMapping { gemini_name: "distortion_filter_resonance", vital_key: "distortion_filter_resonance", convert: identity,     min: 0.0, max: 1.0 },
    ParamMapping { gemini_name: "distortion_filter_blend",     vital_key: "distortion_filter_blend",     convert: filter_blend, min: 0.0, max: 2.0 },

    // ── Compressor ────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "compressor_attack",    vital_key: "compressor_attack",    convert: identity,        min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "compressor_release",   vital_key: "compressor_release",   convert: identity,        min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "compressor_mix",       vital_key: "compressor_mix",       convert: identity,        min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "compressor_band_gain", vital_key: "compressor_band_gain", convert: compressor_gain, min: -6.0, max: 30.0 },
    ParamMapping { gemini_name: "compressor_low_gain",  vital_key: "compressor_low_gain",  convert: compressor_gain, min: -6.0, max: 30.0 },
    ParamMapping { gemini_name: "compressor_high_gain", vital_key: "compressor_high_gain", convert: compressor_gain, min: -6.0, max: 30.0 },

    // ── EQ ────────────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "eq_band_cutoff",    vital_key: "eq_band_cutoff",    convert: midi_cutoff, min: 8.0,   max: 136.0 },
    ParamMapping { gemini_name: "eq_band_gain",      vital_key: "eq_band_gain",      convert: eq_gain,     min: -15.0, max: 15.0 },
    ParamMapping { gemini_name: "eq_band_resonance", vital_key: "eq_band_resonance", convert: identity,    min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "eq_high_cutoff",    vital_key: "eq_high_cutoff",    convert: midi_cutoff, min: 8.0,   max: 136.0 },
    ParamMapping { gemini_name: "eq_high_gain",      vital_key: "eq_high_gain",      convert: eq_gain,     min: -15.0, max: 15.0 },
    ParamMapping { gemini_name: "eq_high_resonance", vital_key: "eq_high_resonance", convert: identity,    min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "eq_low_cutoff",     vital_key: "eq_low_cutoff",     convert: midi_cutoff, min: 8.0,   max: 136.0 },
    ParamMapping { gemini_name: "eq_low_gain",       vital_key: "eq_low_gain",       convert: eq_gain,     min: -15.0, max: 15.0 },
    ParamMapping { gemini_name: "eq_low_resonance",  vital_key: "eq_low_resonance",  convert: identity,    min: 0.0,   max: 1.0 },

    // ── Flanger ───────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "flanger_center",       vital_key: "flanger_center",       convert: midi_cutoff, min: 8.0,  max: 136.0 },
    ParamMapping { gemini_name: "flanger_dry_wet",      vital_key: "flanger_dry_wet",      convert: identity,    min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "flanger_feedback",     vital_key: "flanger_feedback",     convert: identity,    min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "flanger_frequency",    vital_key: "flanger_frequency",    convert: lfo_freq,    min: -8.0, max: 8.0 },
    ParamMapping { gemini_name: "flanger_mod_depth",    vital_key: "flanger_mod_depth",    convert: identity,    min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "flanger_phase_offset", vital_key: "flanger_phase_offset", convert: identity,    min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "flanger_tempo",        vital_key: "flanger_tempo",        convert: tempo,       min: 0.0,  max: 12.0 },

    // ── Phaser ────────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "phaser_center",       vital_key: "phaser_center",       convert: midi_cutoff,     min: 8.0,  max: 136.0 },
    ParamMapping { gemini_name: "phaser_dry_wet",      vital_key: "phaser_dry_wet",      convert: identity,        min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "phaser_feedback",     vital_key: "phaser_feedback",     convert: identity,        min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "phaser_frequency",    vital_key: "phaser_frequency",    convert: lfo_freq,        min: -8.0, max: 8.0 },
    ParamMapping { gemini_name: "phaser_mod_depth",    vital_key: "phaser_mod_depth",    convert: phaser_mod_depth,min: 0.0,  max: 48.0 },
    ParamMapping { gemini_name: "phaser_phase_offset", vital_key: "phaser_phase_offset", convert: identity,        min: 0.0,  max: 1.0 },
    ParamMapping { gemini_name: "phaser_tempo",        vital_key: "phaser_tempo",        convert: tempo,           min: 0.0,  max: 12.0 },

    // ── Sample ────────────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "sample_level",     vital_key: "sample_level",     convert: identity,     min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "sample_pan",       vital_key: "sample_pan",       convert: bipolar,      min: -1.0,  max: 1.0 },
    ParamMapping { gemini_name: "sample_transpose", vital_key: "sample_transpose", convert: semitones_48, min: -48.0, max: 48.0 },
    ParamMapping { gemini_name: "sample_tune",      vital_key: "sample_tune",      convert: bipolar,      min: -1.0,  max: 1.0 },

    // ── Voice / Global ────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "polyphony",        vital_key: "polyphony",        convert: polyphony,    min: 1.0,   max: 32.0 },
    ParamMapping { gemini_name: "portamento_time",  vital_key: "portamento_time",  convert: env_time,     min: 0.0,   max: 4.0 },
    ParamMapping { gemini_name: "pitch_bend_range", vital_key: "pitch_bend_range", convert: semitones_48, min: -48.0, max: 48.0 },
    ParamMapping { gemini_name: "velocity_track",   vital_key: "velocity_track",   convert: identity,     min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "voice_amplitude",  vital_key: "voice_amplitude",  convert: identity,     min: 0.0,   max: 1.0 },
    ParamMapping { gemini_name: "legato",           vital_key: "legato",           convert: identity,     min: 0.0,   max: 1.0 },

    // ── Macro Controls ────────────────────────────────────────────────────────
    ParamMapping { gemini_name: "macro_control_1", vital_key: "macro_control_1", convert: identity, min: 0.0, max: 1.0 },
    ParamMapping { gemini_name: "macro_control_2", vital_key: "macro_control_2", convert: identity, min: 0.0, max: 1.0 },
    ParamMapping { gemini_name: "macro_control_3", vital_key: "macro_control_3", convert: identity, min: 0.0, max: 1.0 },
    ParamMapping { gemini_name: "macro_control_4", vital_key: "macro_control_4", convert: identity, min: 0.0, max: 1.0 },
];

/// Look up a parameter mapping by Gemini output name.
pub fn find_mapping(gemini_name: &str) -> Option<&'static ParamMapping> {
    PARAM_MAPPINGS
        .iter()
        .find(|m| m.gemini_name == gemini_name)
}

// ── Seed Bank ────────────────────────────────────────────────────────────────
//
// 18 curated .vital presets embedded at compile time.
// Each provides rich wavetables, mod routings, samples, and LFO shapes
// that Gemini's 190 knob values are merged on top of.

const SEED_ALIEN_BASS: &str = include_str!("seeds/alien_bass.vital");
const SEED_ARP_SEQUENCE: &str = include_str!("seeds/arp_sequence.vital");
const SEED_BELL: &str = include_str!("seeds/bell.vital");
const SEED_DRUM_KICK: &str = include_str!("seeds/drum_kick.vital");
const SEED_DRUM_SNARE: &str = include_str!("seeds/drum_snare.vital");
const SEED_FX_RISER: &str = include_str!("seeds/fx_riser.vital");
const SEED_GROWL_BASS: &str = include_str!("seeds/growl_bass.vital");
const SEED_KEYS: &str = include_str!("seeds/keys.vital");
const SEED_KEYS_AMBIENT: &str = include_str!("seeds/keys_ambient.vital");
const SEED_MEGASAW_LEAD: &str = include_str!("seeds/megasaw_lead.vital");
const SEED_ORGAN: &str = include_str!("seeds/organ.vital");
const SEED_PAD_AMBIENT: &str = include_str!("seeds/pad_ambient.vital");
const SEED_PAD_WARM: &str = include_str!("seeds/pad_warm.vital");
const SEED_PLUCK: &str = include_str!("seeds/pluck.vital");
const SEED_SUB_BASS: &str = include_str!("seeds/sub_bass.vital");
const SEED_SUPERSAW_LEAD: &str = include_str!("seeds/supersaw_lead.vital");
const SEED_TEAROUT_BASS: &str = include_str!("seeds/tearout_bass.vital");
const SEED_VOCAL: &str = include_str!("seeds/vocal.vital");

/// Valid seed category names (used in Flash classification prompt).
pub const SEED_CATEGORIES: &[&str] = &[
    "alien_bass", "arp_sequence", "bell", "drum_kick", "drum_snare",
    "fx_riser", "growl_bass", "keys", "keys_ambient", "megasaw_lead",
    "organ", "pad_ambient", "pad_warm", "pluck", "sub_bass",
    "supersaw_lead", "tearout_bass", "vocal",
];

/// Look up a seed template by category name. Falls back to Init.vital.
pub fn get_seed_template(category: &str) -> &'static str {
    match category {
        "alien_bass" => SEED_ALIEN_BASS,
        "arp_sequence" => SEED_ARP_SEQUENCE,
        "bell" => SEED_BELL,
        "drum_kick" => SEED_DRUM_KICK,
        "drum_snare" => SEED_DRUM_SNARE,
        "fx_riser" => SEED_FX_RISER,
        "growl_bass" => SEED_GROWL_BASS,
        "keys" => SEED_KEYS,
        "keys_ambient" => SEED_KEYS_AMBIENT,
        "megasaw_lead" => SEED_MEGASAW_LEAD,
        "organ" => SEED_ORGAN,
        "pad_ambient" => SEED_PAD_AMBIENT,
        "pad_warm" => SEED_PAD_WARM,
        "pluck" => SEED_PLUCK,
        "sub_bass" => SEED_SUB_BASS,
        "supersaw_lead" => SEED_SUPERSAW_LEAD,
        "tearout_bass" => SEED_TEAROUT_BASS,
        "vocal" => SEED_VOCAL,
        _ => INIT_TEMPLATE_JSON,
    }
}

