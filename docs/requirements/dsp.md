# DSP

The `haptic_dsp` crate, `core/dsp`.

r[dsp.linspace]
For a `count` of 2 or more, `linspace` shall yield `count` values, the first equal to `start` and each later one a step of `(end - start) / (count - 1)` further on, so that the last reaches `end` up to `f32` rounding.

Outside this requirement: a `count` of 1 yields `NaN` from a division by zero in `core/dsp/src/linspace.rs`, and a `count` of 0 panics in a debug build and yields nothing in a release build.

r[dsp.windowed-rms]
`WindowedMovingRms::process` shall return the RMS of the window of the last N samples, in which a sample not yet received counts as zero.

r[dsp.windowed-rms-length]
`WindowedMovingRms::new` shall set the window length N to `averaging_time * sample_rate` rounded to the nearest integer.

Outside this requirement: a product that rounds to 0 makes `process` return `NaN`.
