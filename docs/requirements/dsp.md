# DSP

The `haptic_dsp` crate, `core/dsp`.

r[dsp.linspace]
For a `count` of 2 or more, `linspace` shall yield `count` evenly spaced values from `start` to `end`, both included.

A `count` of 1 makes the step a division by zero in `core/dsp/src/linspace.rs`, so this requirement does not cover it.

r[dsp.windowed-rms]
`WindowedMovingRms::process` shall return the RMS of the last N samples it received, where N is the window length that `new` derives from the averaging time and the sample rate.
