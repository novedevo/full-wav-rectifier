use full_wav_rectifier::*;
use std::{fs::File, io::Cursor, path::Path};

fn main() {
    let raw = include_bytes!("../data/input.wav");
    let mut inp: Cursor<Vec<u8>> = Cursor::new(raw.to_vec());
    let mut out = File::create(Path::new("data/output_acc.wav")).unwrap();

    let (header, data) = wav::read(&mut inp).unwrap();

    let new_data: wav::BitDepth = match data {
        wav::BitDepth::Eight(_) => unimplemented!("lmao 8bit wav file, get better audio or upsample u scrub. (i dont think this library knows how to handle unsigned integers)"),
        wav::BitDepth::Sixteen(d) => acc_16(d).into(),
        wav::BitDepth::TwentyFour(d) => acc_24(d).into(),
        wav::BitDepth::ThirtyTwoFloat(d) => acc_float(d).into(),
        wav::BitDepth::Empty => unimplemented!("Empty .wav file detected. What were you hoping to accomplish?"),
    };

    wav::write(header, &new_data, &mut out).unwrap();
}
