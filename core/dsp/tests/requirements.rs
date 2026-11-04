//! Cases upstream's inline tests do not hold, kept here so upstream's files take only marker
//! comments.

use haptic_dsp::{WindowedMovingRms, linspace};

// r[verify dsp.linspace]
#[test]
fn linspace_of_2_yields_both_ends() {
    let values: Vec<f32> = linspace(0.0, 1.0, 2).collect();
    assert_eq!(values, [0.0, 1.0]);
}

// r[verify dsp.linspace]
#[test]
fn linspace_reaches_end_up_to_f32_rounding() {
    let last = linspace(0.0, 1.0, 42).last().unwrap();
    assert!((last - 1.0).abs() <= f32::EPSILON, "last value {last}");
}

// r[verify dsp.windowed-rms]
#[test]
fn rms_counts_samples_not_yet_received_as_zero() {
    let mut rms = WindowedMovingRms::new(4.0, 1.0);
    assert_eq!(rms.process(1.0), 0.5);
}

// r[verify dsp.windowed-rms-length]
#[test]
fn window_length_rounds_down_below_a_half() {
    // 1.4 rounds to 1: the first sample has left the window by the second call.
    let mut rms = WindowedMovingRms::new(1.4, 1.0);
    assert_eq!(rms.process(1.0), 1.0);
    assert_eq!(rms.process(0.0), 0.0);
}

// r[verify dsp.windowed-rms-length]
#[test]
fn window_length_rounds_up_above_a_half() {
    // 1.6 rounds to 2: a single sample fills half of the window.
    let mut rms = WindowedMovingRms::new(1.6, 1.0);
    assert_eq!(rms.process(1.0), (1.0 / 2.0f32).sqrt());
    assert_eq!(rms.process(1.0), 1.0);
}
