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

pub fn mul_by_previous(sample: f64, prev: f64) -> f64 {
    (sample * prev - 0.5) * 2.0
}

pub fn div_by_previous(sample: f64, prev: f64) -> f64 {
    if prev != 0.0 {
        (sample / prev).clamp(-1.0, 1.0)
    } else {
        sample
    }
}

#[inline(always)]
fn db_to_amplitude(db: f64) -> f64 {
    10_f64.powf(db / 20.0)
}
