<img src="resources/images/logo/Transparent/Banner-SDK-Transparent.png" alt="Haptics SDK Logo" style="width:95%; max-width:800px; display:block; margin-left:auto; margin-right:auto;">

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A low-level haptic rendering and data processing library based on a universal
format. Designed as a core component for integration into game engines, audio
middleware, and other systems, Haptics SDK runs in production across millions of
devices daily.

---

## Table of Contents

- [Feature overview](#feature-overview)
- [Industry Adoption](#industry-adoption)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Quick Start](#quick-start)
  - [Building](#building)
- [Examples](#examples)
- [How It Works](#how-it-works)
- [Design Philosophy](#design-philosophy)
- [Documentation](#documentation)
- [FAQ](#faq)
- [Contributing](#contributing)
- [License](#license)

---

## Feature overview

![Conceptual System Diagram](resources/images/Feature-Overview-Transparent.png)

### 🎨 Platform-Agnostic Haptic Input Data

Works with normalized haptic parameters such as amplitude, frequency, and
transients, rather than raw waveforms. This approach aligns with the broader
industry shift toward standardized, device-agnostic haptic data structures (not
only via the `.haptic` format in this repo but also Apple's `.ahap` format). By
separating haptic design intent from device playback capabilities, you can
ensure consistent experiences across diverse hardware.

### ⚙️ Configuration-Based Render Output

Actuator Configuration Files (ACF) define how abstract haptic data maps to
specific hardware. This makes it easy to support multiple output platforms
without modifying your code, streamlining integration and maintenance.

### 🎛️ Dual Rendering Modes

Supports a wide range of platform API and device capabilites, including both
PCM-based systems and those based on simple amplitude curves.

### 🔧 Integration Friendly

- **No external dependencies** - Standalone C/C++ source code
- **Minimal API surface** - Simple, easy-to-understand integration points
- **Cross-platform** - Windows, macOS, Linux, consoles & firmware

---

## Industry Adoption

Meta Haptics SDK serves as the core haptic rendering technology in major tools
and platforms:

### Audio Middleware

Natively integrated into professional audio middleware:

- **[FMOD Haptics Instrument](https://www.fmod.com/docs/2.03/studio/instrument-reference.html#haptics-instrument)** -
  Native haptic authoring for PlayStation® (including DualSense™), Xbox, and
  Meta Quest
- **[Wwise Haptic Clip Player](https://www.audiokinetic.com/en/public-library/2025.1.6_9117/?source=Help&id=haptic_clip_player_plug_in)** -
  Audiokinetic's native haptic authoring integration

### Game Engines

- **[Meta Haptics SDK for Unity](https://developers.meta.com/horizon/documentation/unity/unity-haptics-sdk/)** -
  Quest and PCVR haptics for Unity applications
- **[Meta Haptics SDK for Unreal](https://developers.meta.com/horizon/documentation/unreal/unreal-haptics-sdk/)** -
  Quest and PCVR haptics for Unreal Engine applications

### Design Tools

- **[Meta Haptics Studio](https://developers.meta.com/horizon/resources/haptics-studio)** -
  Professional haptic design tool using this SDK as its core renderer on
  Windows, Mac and Meta Quest's Android based runtime.

---

## Getting Started

### Prerequisites

#### Build Tools

- **[CMake](https://cmake.org/)** 3.10 or higher
- **[just](https://github.com/casey/just)** (optional, makes building and
  testing more convenient)
- C/C++ compiler (GCC, Clang, MSVC, or other)

#### Supported Platforms

- Windows (MSVC, MinGW)
- macOS (Clang)
- Linux (GCC, Clang)
- VR headsets (Quest, PCVR)
- Consoles (PlayStation®, Xbox)

### Quick Start

```bash
# Clone the repository
git clone git@github.com:basslet/haptics-sdk.git
cd haptics-sdk

# Initialize submodules
git submodule init
git submodule update
```

### Building

#### Using `just` (Recommended)

```bash
just            # Build and test all components
just build-all  # Build all components without testing
```

#### Using CMake

The justfile in the directory of each component contains the CMake commands.

---

## Documentation

### Complete Documentation Set

| Document                                                             | Description                                    |
| -------------------------------------------------------------------- | ---------------------------------------------- |
| **[Parametric Haptic Renderer](core/renderer_parametric/README.md)** | Complete documentation and integration example |
| **[Parametric Haptic Data](core/haptic_data_parametric/README.md)**  | Short description                              |
| **[.haptic File Spec](resources/haptic-file-spec.md)**               | Format specification with validation rules     |
| **[ACF Specification](resources/acf/README.md)**                     | Tuning guide and parameter reference           |
| **[Unity PCM Haptics](interfaces/unity/PCMHaptics/README.md)**       | Complete integration example                   |
| **[Unreal PCM Haptics](interfaces/unreal/PCMHaptics/README.md)**     | Complete integration example                   |

### External Resources

- **[Meta Haptics Studio](https://developers.meta.com/horizon/resources/haptics-studio)** -
  Design tool for creating .haptic files

---

## FAQ

### General Questions

**Q: What platforms does Meta Haptics SDK support?**

**A: The SDK is platform-agnostic C/C++ code.** It runs on Windows, macOS,
Linux, and consoles. Integration examples focus on Meta Quest via OpenXR.

**Q: Can I use this with my existing game engine?**

**A: Yes!** The SDK integrates into Unity, Unreal, FMOD, and Wwise. You can also
integrate directly into custom engines—it's just C/C++.

**Q: Do I need Meta hardware to use this?**

**A: No.** While the SDK powers Meta Quest haptics, it's designed to work with
any vibrotactile actuator. Use the ACF system to tune for your target hardware.

**Q: Is this only for VR?**

**A: No.** The SDK works with any haptic-enabled device: VR controllers, PCVR,
game console controllers (PlayStation® DualSense™, Xbox), and more.

### Technical Questions

**Q: When should I use Synthesis vs Amplitude Curve mode?**

**A:**

- **Synthesis Mode** - When the platform supports PCM waveforms (Quest,
  DualSense). Provides full frequency control.
- **Amplitude Curve Mode** - When the platform only accepts amplitude values
  (some Android APIs, simpler consoles).

**Q: How do I tune an ACF for my hardware?**

**A:**

1. Start with a template ACF (e.g., `Generic-Console-PCM.acf`)
2. Consult your actuator's datasheet for frequency response
3. Set `frequency_min`/`frequency_max` to the usable range (e.g., 55-200 Hz)
4. Tune emphasis settings around the resonant frequency (f₀)
5. Test with real content and adjust by feel

See the [ACF specification](resources/acf/README.md) for detailed guidance.

**Q: Can I create .haptic files programmatically?**

**A: Yes.** The [.haptic specification](resources/haptic-file-spec.md) defines
the data format. You can generate files in any language and validate against the
spec.

**Q: What's the difference between emphasis and transients?**

**A: They're the same concept, different terminology.** "Emphasis" is the term
used in the renderer API, "transient" is used in haptic design (short,
click-like events).

### Compatibility Questions

**Q: Can I use .haptic files created in Meta Haptics Studio?**

**A: Absolutely!** That's the primary design workflow. Studio exports .haptic
files that this SDK renders at runtime.

---

## Contributing

We welcome contributions! Whether you're fixing bugs, adding platform examples,
or improving documentation, your help is greatly appreciated. To set
expectations: the maintainers of this repository are committed to strong
stewardship of its technical direction and feature set. As a result, not every
suggestion or contribution may be accepted. However, please don't let that
discourage you: we're a friendly team and always happy to discuss your ideas for
improvements!

Thank you for your interest in making this project better.

See our [Contribution Guidelines](CONTRIBUTING.md) for more information.

### Code of Conduct

See our [Code of Conduct](CODE_OF_CONDUCT.md)

---

## License

Meta Haptics SDK is released under the MIT License. See [LICENSE](LICENSE) for
details.

---

**Need help?**

Open an issue or check our [documentation](#documentation).

**Ready to design haptics?**

Download
[Meta Haptics Studio](https://developers.meta.com/horizon/resources/haptics-studio).

---

Built with love by Meta's IxI Haptics team (formerly Lofelt ❤️)
