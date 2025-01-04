# ¡repiquemos!

<small>(english translation: let's ring out!)</small>

## do horrible things to your audio

### supported operations:
rectification, skipclipping, accumulation, and mul-by-previous/div-by-previous. 

- rectification just takes the `abs()` of the signal, then subtracts a dc offset to recentre the waveform, then amplifies to reobtain a full-scale signal.

- skipclipping is a name i made up.
normal clipping clamps the samples at a certain level. skipclipping instead discards all samples that are above a certain threshold, then re-amplifies it to be full-scale.
skipclipping is a weird one, because dropping samples means it will increase frequencies and decrease file length in a somewhat unpredictable fashion roughly correlating to the loudness of the input signal.

- accumulation means the value of each sample is the sum of all the previous samples' `abs()`, modulo'd into a full-scale signal.

- mul-by-previous (name suggestions appreciated) multiplies each sample by the amplitude of the previous sample. div-by-previous follows this pattern. there's a little post-processing to improve the listening experience.

### notes

the `repiquemos` library crate exposes these operations as simple scalar functions. the `repiquemos-cli` binary crate builds them into a simple CLI that lets you process WAV files with a simple DSL to compose these operations.

if you do some of these operations with integer PCM audio, there might be some noise from bit depth conversion. floats don't have this issue, so for optimum audio quality, i recommend using floats.

a note on naming: this library used to be known as `full-wav-rectifier` until i added more operations than rectify and genericized the library to not rely on one file format. it was then briefly known as `caramelo`, but audio processed by these operations does not tend to be as smooth as caramel. 
