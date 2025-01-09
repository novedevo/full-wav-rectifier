use core::f64;

pub fn rectify(sample: f64) -> f64 {
    sample.abs()
}

pub fn saturate(sample: f64) -> f64 {
    sample.tanh()
}

pub fn dc_offset(sample: f64, offset: f64) -> f64 {
    sample + offset
}

pub fn amplify(sample: f64, amplification_factor: f64) -> f64 {
    ring_mod(sample, amplification_factor)
}

pub fn ring_mod(signal: f64, modulator: f64) -> f64 {
    signal * modulator
}

pub fn skipclip(sample: f64, threshold_db: f64) -> Option<f64> {
    let amp = db_to_amplitude(threshold_db);
    if sample.abs() < amp {
        Some(sample / amp)
    } else {
        None
    }
}

pub fn skip_silence(sample: f64) -> Option<f64> {
    if sample == 0.0 {
        None
    } else {
        Some(sample)
    }
}

pub fn acc(sample: f64, accumulator: f64) -> (f64, f64) {
    let acc = (accumulator + sample.abs()) % 2.0;
    (acc - 1.0, acc)
}

/// it's recommended that you give this function full-scale audio. low-frequency max amplitude samples are
/// mostly unaffected, but quieter samples are squished even closer to zero.
/// 
/// the effect is kinda like an inverse compressor (expander) fed into a saturator
///
///author: gen@nyble.dev
pub fn safe_mul_by_previous(sample: f64, prev: f64) -> f64 {
    sample.abs() * prev.abs() * sample.signum()
}

pub fn mul_by_previous(sample: f64, prev: f64) -> f64 {
    ring_mod(sample, prev)
}

/// naturally, repeatedly dividing by the previous sample results in samples often
/// asymptotically tending to infinity.
///
/// also, the waveform is centered on +1 amplitude because peaks and valleys are effectively
/// dividing by themselves.
///
/// therefore, for somewhat listenable audio, i recommend a DC offset of -1 followed by clamping
pub fn div_by_previous(sample: f64, prev: f64) -> f64 {
    sample / prev
}

pub fn clamp(sample: f64) -> f64 {
    sample.clamp(-1.0, 1.0)
}

#[inline(always)]
fn db_to_amplitude(db: f64) -> f64 {
    10_f64.powf(db / 20.0)
}
