# full-wav-rectifier

do horrible things to your audio

notably, you can rectify, it skipclip it, or accumulate it.

rectification just takes the abs() of the signal, then does some other stuff to make it the same amplitude and zero-offset.

skipclipping is a name i made up for an operation i made up.
instead of the nerd-based clipping algorithms of saturation, clamping, etc; this straight-up discards all samples that are too loud, then re-amplifies it to be full-scale.
skipclipping is a weird one, because dropping samples means it will increase frequencies and decrease file length. who needs compression.

accumulation has lots of meanings, but here, i say it means the value of each sample is the sum of all the previous samples' abs(), modulo'd into a full-scale signal.

if you do any of these operations with integer pcm wavs, there's probably going to be some noise from bit depth conversion. use floats, they're better.
