use hound::{WavSpec, WavWriter};
use repiquemos::*;

const CORRUPTED_ERROR: &str = "corrupted wav file: sample bit depth doesn't match header or floats aren't within -1.0 and 1.0";

fn main() {
    let input_file = std::env::args()
        .nth(1)
        .unwrap_or("data/input.wav".to_string()); //.expect("No input file provided");
    let mut reader = hound::WavReader::open(input_file).expect("Failed to open input file");
    let spec = reader.spec();

    let mut samples: Box<dyn Iterator<Item = f64>> =
        if spec.sample_format == hound::SampleFormat::Int {
            let iter = reader.samples::<i32>().map(|s| {
                s.expect(CORRUPTED_ERROR) as f64 / 2_i32.pow(spec.bits_per_sample as u32) as f64
            });
            Box::new(iter)
        } else {
            let iter = reader.samples::<f32>().map(|s| {
                s.and_then(|f| {
                    if f >= -1.0 && f <= 1.0 {
                        Ok(f)
                    } else {
                        Err(hound::Error::UnfinishedSample)
                    }
                })
                .expect(CORRUPTED_ERROR) as f64
            });
            Box::new(iter)
        };

    let dsl = std::env::args().nth(3).unwrap();
    let dsl = dsl.split(",");

    for command in dsl {
        match command {
            "skipclip" => {
                samples = Box::new(samples.filter_map(|s| skipclip(s, -3.0)));
            }
            "rectify" => {
                samples = Box::new(
                    samples
                        .map(rectify)
                        .map(|s| dc_offset(s, -0.5))
                        .map(|s| ring_mod(s, 2.0)),
                );
            }
            "accumulate" => {
                let mut accumulator = 0.0f64;
                samples = Box::new(samples.map(move |sample| {
                    let (sample, newacc) = acc(sample, accumulator);
                    accumulator = newacc;
                    sample
                }))
            }
            "mul-by-previous" => {
                let mut prev = 1.0;
                samples = Box::new(samples.map(move |sample| {
                    let retval = safe_mul_by_previous(sample, prev);
                    prev = sample;
                    retval
                }))
            }
            "div-by-previous" => {
                let mut prev = 1.0;
                samples = Box::new(
                    samples
                        .filter_map(skip_silence)
                        .map(move |sample| {
                            let retval: f64 = div_by_previous(sample, prev);
                            prev = sample;
                            retval
                        })
                        .map(|s| dc_offset(s, -1.0))
                        .map(clamp),
                )
            }
            _ => unimplemented!("no support for this operation yet"),
        }
    }
    let mut writer =
        WavWriter::create("data/output.wav", spec).expect("couldn't create output file");
    let samples = samples.collect::<Vec<_>>();
    for sample in samples {
        if spec.sample_format == hound::SampleFormat::Int {
            let inted_sample =
                (sample * (2_i32.pow(spec.bits_per_sample as u32 - 1) - 1) as f64) as i32;
            if let Err(e) = writer.write_sample(inted_sample) {
                panic!(
                    "IO error: writing float sample: {sample} (inted: {inted_sample}): {:?}: {:#}",
                    e, e
                )
            }
        } else {
            writer
                .write_sample(sample as f32)
                .expect("IO error: writing f32 sample");
        }
    }
    writer.flush().expect("IO error");
}
