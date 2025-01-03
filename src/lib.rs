const I24_MAX: i32 = 2_i32.pow(24 - 1);

pub fn is_normalized(samples: &mut impl Iterator<Item = f32>) -> bool {
    samples.all(|s| s.abs() <= 1.0 + f32::EPSILON)
}

pub fn rectify(sample: f64) -> f64 {
    (sample.abs() - 0.5) * 2.0
}

pub fn skipclip(sample: f64, threshold_db: f64) -> Option<f64> {
    let amp = db_to_amplitude(threshold_db);
    if sample.abs() < amp {
        Some(sample / amp)
    } else {
        None
    }
}

pub fn acc(sample: f64, accumulator: f64) -> (f64, f64) {
    let acc = (accumulator + sample.abs()) % 2.0;
    (acc - 1.0, acc)
}

pub fn mul_by_previous_16(mut v: Vec<i16>) -> Vec<i16> {
    let mut last = 1.0;
    for sample in v.iter_mut() {
        let next_last = *sample as f64 / i16::MAX as f64;
        *sample = (((*sample as f64 * last) - 0.5) * 2.0) as i16;
        last = next_last;
    }
    v
}

pub fn mul_by_previous_24(mut v: Vec<i32>) -> Vec<i32> {
    let mut last = 1.0;
    for sample in v.iter_mut() {
        let next_last = *sample as f64 / I24_MAX as f64;
        *sample = (*sample as f64 * last) as i32;
        *sample -= I24_MAX / 2;
        *sample *= 2;
        last = next_last;
    }
    v
}

pub fn mul_by_previous_float(mut v: Vec<f32>) -> Vec<f32> {
    let mut last = 1.0;
    for sample in v.iter_mut() {
        let next_last = *sample;
        *sample *= last;
        *sample -= 0.5;
        *sample *= 2.0;
        last = next_last;
    }
    v
}

pub fn div_by_previous_16(mut v: Vec<i16>) -> Vec<i16> {
    let mut last = 1.0;
    for sample in v.iter_mut() {
        let next_last = *sample as f64 / i16::MAX as f64;
        if last != 0.0 {
            *sample = (*sample as f64 / last).clamp(-1.0, 1.0) as i16;
        }
        last = next_last;
    }
    v
}
pub fn div_by_previous_24(mut v: Vec<i32>) -> Vec<i32> {
    let mut last = 1.0;
    for sample in v.iter_mut() {
        let next_last = *sample as f64 / I24_MAX as f64;
        if last != 0.0 {
            *sample = (*sample as f64 / last).clamp(-1.0, 1.0) as i32;
        }
        last = next_last;
    }
    v
}
pub fn div_by_previous_float(mut v: Vec<f32>) -> Vec<f32> {
    let mut last = 1.0;
    for sample in v.iter_mut() {
        let next_last = *sample;
        if last != 0.0 {
            let calced = *sample / last;
            *sample = calced.clamp(-1.0, 1.0);
        }
        last = next_last;
    }
    v
    // skipclip_float(v)
}

fn db_to_amplitude(db: f64) -> f64 {
    10_f64.powf(db / 20.0)
}
