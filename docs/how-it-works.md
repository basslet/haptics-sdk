## How It Works

Meta Haptics SDK is a haptic rendering library with a five-component
architecture that separates content from hardware.

### Core Components

The SDK consists of five main components that work together:

1. **.haptic Files (Specification: `resources/haptic-file-spec.md`)**: The
   `.haptic` format is a JSON-based, device-agnostic representation of
   vibrotactile effects. This repository open-sources the `.haptic` file
   specification, but the underlying renderer is **file format agnostic** and
   easily compatible with similar parametric formats like Apple's AHAP. The
   format is designed for interchange between design tools, game engines, and
   runtime systems.

2. **Parametric Haptic Data (`core/haptic_data_parametric`)**: A C++ library for
   the file format used by Parametric Haptic Renderer (below). It takes .haptic
   files as input and converts them to Parametric Haptic Data.

3. **Actuator Configuration Files (ACF) (`resources/acf`)**: JSON5 configuration
   files that define how normalized haptic data (0.0-1.0) within .haptic files
   (and Parametric Haptic Data) maps to specific hardware characteristics. They
   contain device-specific parameters like frequency ranges, gain curves, and
   transient waveform shapes.

4. **Parametric Renderer (`core/renderer_parametric`)**: A realtime rendering
   C++ library. You initialize it with Parametric Haptic Data and ACF data and
   then drive it from update loop or callback. In turn it has Haptic Renderer
   (see below) render slices of parametric data to a the format required by the
   platform haptic API.

5. **Haptic Renderer (`core/renderer_c`)**: A lightweight C library that
   processes parametric control points (amplitude and frequency ramps,
   transients) and generates output samples according to the ACF at a target
   sample rate. It operates in real-time, maintaining internal state for smooth
   interpolation between control points.

### Rendering Process

Meta Haptics SDK interpolates between hardware-agnostic parametric control
points, and based on the ACF, renders the optimal output for the platform API or
device hardware.

![Haptics SDK Rendering Process](resources/images/haptic_file_spec/system-overview-rendering.png)

_The renderer processes time-based control points (amplitude, frequency,
transients) and generates device-specific output samples._

This separation enables:

- Targeting any configured hardware with the same content
- Tuning haptic feel per-device via ACF configuration
- Using the same content pipeline across all platforms

---

## Design Philosophy

Meta Haptics SDK uses **normalized, parametric data** to distinctly separate
design-time concerns from runtime implementation. This approach provides
significant benefits for both content creators and system integrators.

### A Low-Level Building Block for Haptic Systems

Meta Haptics SDK is a **haptic rendering and data processing library** designed
to be integrated into larger systems. It focuses on a single responsibility:
efficiently converting device-agnostic vibrotactile data into device and
platform-specific output.

**What it provides:**

- Real-time rendering of parametric haptic data (amplitude/frequency envelopes,
  transients)
- Device-specific adaptation through Actuator Configuration Files (ACF)
- Pure C/C++ implementation with zero external dependencies
- File format agnostic renderer compatible with `.haptic`, AHAP, or similar
  formats

**What it doesn't provide:**

- High-level playback APIs (play, stop, pause, seek) or playback location.
- Haptic prioritization or mixing systems
- Threading models or process communication
- Asset management or file I/O

This design allows integration into systems that already have their own models
for playback control, threading, and resource management—such as game engines,
audio middleware, and platform runtimes.

### For Haptic Designers

Hardware capabilities vary widely across platforms—from high-end console
controllers to simple gamepads. Requiring designers to create platform-specific
haptic assets is highly inefficient:

- **Haptics typically has the lowest priority and smallest budget** in most
  projects
- **Rework is cost-prohibitive** - designers need to author once and deploy
  everywhere
- **Design intent, not hardware specifics** - normalized envelopes (0.0-1.0)
  capture what designers want to express, not how specific motors should vibrate

By storing only design intent in normalized primitives, the system enables:

- **Graceful degradation** on lower-capability hardware
- **High-quality output** on advanced systems
- **Cross-platform compatibility** - haptics work on new platforms without
  rework
- **Forward and backward compatibility** - content remains valid as systems
  evolve

### For System Integrators

Middleware and game engine creators need maintainable solutions across numerous
platforms. This SDK provides:

- **Configuration-based rendering** - add new platforms with ACF files, not code
  changes
- **No upstream content changes** - existing `.haptic` files work on new
  platforms automatically
- **Cost-effective maintenance** - minimal engineering effort for platform
  support

### Why Not PCM?

Many systems use PCM (Pulse Code Modulation) as their haptic data format, but
**PCM leaks hardware abstraction into the design**:

- PCM waveforms encode frequency information specific to target hardware
- Designers must re-render PCM files for every platform with different motor
  characteristics
- Changes in hardware require re-authoring all content

**Parametric data** solves this by describing _what_ to feel (amplitude,
frequency, transients), not _how_ motors should move. The renderer handles
hardware translation at runtime.
