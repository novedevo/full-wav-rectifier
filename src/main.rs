use full_wav_rectifier::*;
use hound::WavWriter;

const CORRUPTED_ERROR: &str = "corrupted wav file: sample bit depth doesn't match header";

fn main() {
    let input_file = std::env::args()
        .nth(1)
        .unwrap_or("data/input.wav".to_string()); //.expect("No input file provided");
    let reader = hound::WavReader::open(input_file).expect("Failed to open input file");
    let spec = reader.spec();

    let samples: Box<dyn Iterator<Item=f64>> = if spec.sample_format == hound::SampleFormat::Int {
        let iter = reader
            .into_samples::<i32>()
            .map(|s| {
                s.expect(CORRUPTED_ERROR) as f64 / 2_i32.pow(spec.bits_per_sample as u32) as f64
            });
        Box::new(iter)
    } else {
        let iter = reader
            .into_samples::<f32>()
            .map(|s| s.expect(CORRUPTED_ERROR) as f64);
        Box::new(iter)
    };

    let dsl = std::env::args().nth(3).unwrap();
    let dsl = dsl.split(",");

    let mut operating: Box<dyn Iterator<Item = f64>> = Box::new(samples.into_iter());
    for command in dsl {
        match command {
            "skipclip" => {
                operating = Box::new(operating.filter_map(|s| skipclip(s, -3.0)));
            }
            "rectify" => {
                operating = Box::new(operating.map(rectify));
            }
            _ => unimplemented!("no support for this operation yet"),
        }
    }

    let mut writer = WavWriter::create("data/output.wav", spec).expect("couldn't create output file");
    for sample in operating {
        if spec.sample_format == hound::SampleFormat::Int {
            let sample = (sample * 2_i32.pow(spec.bits_per_sample as u32 - 1) as f64) as i32;
            writer.write_sample(sample).expect("IO error");
        } else {
            writer.write_sample(sample as f32).expect("IO error");
        }
    }
    writer.flush().expect("IO error");
}
