use full_wav_rectifier::*;
use std::{fs::File, path::Path};

fn main() {
    let input_file = std::env::args().nth(1).expect("No input file provided");
    let mut buf = File::open(input_file).unwrap();
    let (header, data) = wav::read(&mut buf).expect("Failed to read .wav file");
    let mut out = File::create(Path::new("data/output_skip.wav")).expect("Failed to create output file");

    let new_data: wav::BitDepth = match data {
        wav::BitDepth::Eight(_) => unimplemented!("8-bit wav files aren't supported yet. You can upsample then as 16-bit."),
        wav::BitDepth::Sixteen(d) => skipclip_16(d).into(),
        wav::BitDepth::TwentyFour(d) => skipclip_24(d).into(),
        wav::BitDepth::ThirtyTwoFloat(d) => skipclip_float(d).into(),
        wav::BitDepth::Empty => unimplemented!("Empty .wav file detected. Is something wrong upstream?"),
    };

    wav::write(header, &new_data, &mut out).unwrap();
}
