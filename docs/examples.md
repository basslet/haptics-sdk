## Examples

### 1. Authoring a .haptic file

.haptic files contain the contents of a haptic effect.

#### Example .haptic File

```json
{
  "version": {"major": 1, "minor": 0, "patch": 0},
  "metadata": {
    "editor": "Haptics Studio",
    "description": "Button click effect"
  },
  "signals": {
    "continuous": {
      "envelopes": {
        "amplitude": [
          {"time": 0.0, "amplitude": 0.0},
          {
            "time": 0.05,
            "amplitude": 1.0,
            "emphasis": {"amplitude": 1.0, "frequency": 1.0}
          },
          {"time": 0.1, "amplitude": 0.0}
        ],
        "frequency": [
          {"time": 0.0, "frequency": 0.7},
          {"time": 0.1, "frequency": 0.3}
        ]
      }
    }
  }
}
```

#### Key Concepts

- **Amplitude Envelope** - Controls intensity over time (0.0 = silent, 1.0 =
  strong)
- **Frequency Envelope** - Controls pitch/texture (0.0 = round, 1.0 = sharp)
- **Emphasis** - Short, transient events (clicks, impacts) attached to amplitude
  points
- **Normalized Values** - All values are 0.0-1.0; the renderer maps to hardware
  via ACF

**Design Tools:** Create `.haptic` files visually using
[Haptics Studio](https://developers.meta.com/horizon/resources/haptics-studio)
or author them by hand.

**.haptic Specification:**
[`resources/haptic-file-spec.md`](../resources/haptic-file-spec.md)

### 2. Authoring an Actuator Configuration File (ACF)

ACF files define device-specific characteristics, translating normalized haptic
data (0.0-1.0) into hardware-appropriate parameters.

#### Example ACF (Generic PCM Console)

```json5
{
  metadata: {
    version: '1.0.0',
    device: 'Generic console PCM',
    author: 'Author name',
  },
  continuous: {
    gain: 0.8, // Master volume for continuous vibration
    emphasis_ducking: 0.5, // Reduce continuous by 50% during transients
    frequency_min: 55.0, // Map 0.0 frequency → 55 Hz
    frequency_max: 200.0, // Map 1.0 frequency → 200 Hz
  },
  emphasis: {
    gain: 1.0, // Full strength transients
    fade_out_percent: 0.0,
    frequency_min: {
      output_frequency: 55.0, // Low sharpness: 55 Hz
      duration_ms: 27.0, // ~1.5 cycles
      shape: 'sine', // Smooth waveform
    },
    frequency_max: {
      output_frequency: 130.0, // High sharpness: 130 Hz
      duration_ms: 20.0, // As short as possible
      shape: 'square', // Sharp waveform
    },
  },
}
```

#### Provided ACF Templates

- **[`Generic-Console-PCM.acf`](../resources/acf/Generic-Console-PCM.acf)** - Full
  waveform synthesis (PlayStation® DualSense, etc.)
- **[`Generic-Console-SimpleHaptics.acf`](../resources/acf/Generic-Console-SimpleHaptics.acf)** -
  Amplitude-only output (Xbox, etc.)
- **[`Meta-Quest.acf`](../resources/acf/MetaQuest.acf)** - Meta Quest controller
  tuning

**Tuning Your ACF:** Start with a template and tune by feel. Adjust frequency
ranges based on your actuator's datasheet and perceptual testing.

**ACF Specification:** [`resources/acf/README.md`](../resources/acf/README.md)

### 3. Parsing and Rendering Haptic Data in C++

To parse and render haptic data you need a `.haptic` file and an ACF (see
examples above). Then do the following:

1. Convert the `.haptic` to a ParametricHapticClip using the Parametric Haptic
   Data library.
2. Initialize ParametricHapticRenderer with the render settings from the ACF and
   the ParametricHapticClip.
3. From an update loop or callback, preferably running at a fixed clock rate,
   drive ParametricHapticRenderer in realtime.

**[View Parametric Haptic Renderer Example](../core/renderer_parametric/README.md)**

### 4. PCM Haptics for Unity and Unreal

Complete integration examples for Unity and Unreal demonstrating how to render haptics on
devices that implement OpenXR PCM haptics (Meta Quest).

**[View Unity Example](../interfaces/unity/PCMHaptics/README.md)**

**[View Unreal Example](../interfaces/unreal/PCMHaptics/README.md)**
