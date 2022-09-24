const I24_MAX: i32 = 8_388_608;
const SKIPCLIP_THRESHOLD: f64 = -6.0;

fn rectify_16(v: Vec<i16>) -> Vec<i16> {
    v.into_iter()
        .map(|sample| (sample.abs() + i16::MIN / 2) * 2)
        .collect()
}
fn rectify_24(v: Vec<i32>) -> Vec<i32> {
    v.into_iter()
        .map(|sample| (sample.abs() - I24_MAX / 2) * 2)
        .collect()
}
fn rectify_float(v: Vec<f32>) -> Vec<f32> {
    v.into_iter()
        .map(|sample| {
            if sample.abs() > 1.1 {
                unimplemented!("why is ur float so big")
            }
            (sample.abs() - 0.5) * 2.0
        })
        .collect()
}

fn skipclip_16(v: Vec<i16>) -> Vec<i16> {
    let amp = db_to_amplitude(SKIPCLIP_THRESHOLD) * i16::MAX as f64;
    v.into_iter()
        .map(|sample| sample as f64)
        .filter(|sample| sample.abs() < amp)
        .map(|sample| sample / amp)
        .map(|sample| sample as i16)
        .collect()
}

fn skipclip_24(v: Vec<i32>) -> Vec<i32> {
    let amp = db_to_amplitude(SKIPCLIP_THRESHOLD) * I24_MAX as f64;
    v.into_iter()
        .map(|sample| sample as f64)
        .filter(|sample| sample.abs() < amp)
        .map(|sample| sample / amp)
        .map(|sample| sample as i32)
        .collect()
}

fn skipclip_float(v: Vec<f32>) -> Vec<f32> {
    let amp = db_to_amplitude(SKIPCLIP_THRESHOLD) as f32;
    v.into_iter()
        .filter(|sample| sample.abs() < amp)
        .map(|sample| sample / amp)
        .collect()
}

fn db_to_amplitude(db: f64) -> f64 {
    10_f64.powf(db / 20.0)
}
