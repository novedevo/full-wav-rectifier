use core::f64;

pub fn rectify(sample: f64) -> f64 {
    sample.abs()
}

pub fn dc_offset(sample: f64, offset: f64) -> f64 {
    sample + offset
}

pub fn ring_mod(sample: f64, amplification: f64) -> f64 {
    sample * amplification
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

//author: gen@nyble.dev
pub fn safe_mul_by_previous(sample: f64, prev: f64) -> f64 {
    let retval = sample.abs() * prev.abs();
    if sample.is_sign_positive() {
        retval
    } else {
        -retval
    }
}

pub fn mul_by_previous(sample: f64, prev: f64) -> f64 {
    sample * prev
}

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
