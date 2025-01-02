# full-wav-rectifier

do horrible things to your audio

in particular, you can rectify it, skipclip it, or accumulate it.

rectification just takes the abs() of the signal, then subtracts a dc offset to recentre the waveform, then amplifies to reobtain a full-scale signal. this is theoretically streamable.

skipclipping is a name i made up for an operation i made up.
usually clipping clamps the samples at a certain level. skipclipping instead discards all samples that are above a certain threshold, then re-amplifies it to be full-scale. this is not streamable.
skipclipping is a weird one, because dropping samples means it will increase frequencies and decrease file length in a somewhat unpredictable fashion roughly correlating to the loudness of the input signal.

accumulation means the value of each sample is the sum of all the previous samples' abs(), modulo'd into a full-scale signal. this is theoretically streamable.

if you do some of these operations with integer PCM audio, there's probably going to be some noise from bit depth conversion. floats don't have this issue, so for optimum audio quality, i recommend using floats.
