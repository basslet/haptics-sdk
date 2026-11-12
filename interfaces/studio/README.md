| | |
| :--- | :--- |
| **Haptics Studio** | [Overview](https://github.com/basslet/haptics-studio) · [Design & Integration](https://github.com/basslet/haptics-studio/blob/main/docs/design-and-integration/README.md) · [Source Code](https://github.com/basslet/haptics-studio/blob/main/docs/building.md) · [Architecture](https://github.com/basslet/haptics-studio/blob/main/docs/architecture.md) |
| **Haptics SDK** | **▸ Overview** · [Getting Started](../../docs/getting-started.md) · [Documentation](../../docs/README.md) |
| **Haptics Studio Examples** | [Overview](https://github.com/basslet/haptics-studio-examples) · [Guidelines](https://github.com/basslet/haptics-studio-examples/blob/main/1%20Design%20Guidelines/README.md) · [Examples](https://github.com/basslet/haptics-studio-examples/blob/main/2%20Design%20Examples/README.md) · [Code](https://github.com/basslet/haptics-studio-examples/blob/main/3%20Integration%20%26%20Code%20Snippets/README.md) · [Knowledge](https://github.com/basslet/haptics-studio-examples/blob/main/4%20Knowledge%20Base/README.md) · [Skills](https://github.com/basslet/haptics-studio-examples/blob/main/5%20Skills/README.md) |
| **General** | [Code of Conduct](../../CODE_OF_CONDUCT.md) · [Contributing](../../CONTRIBUTING.md) · [License](../../LICENSE) · [Security](../../SECURITY.md) |

# `interfaces/studio`

This folder contains the Haptics SDK interface used by Haptics Studio.

## Directory Structure
- `audio_analysis`
  - Contains support for generating haptics from audio data.
- `napi`
  - Contains a Node-API interface for the Haptics SDK, along with additional
    functionality from `audio_analysis`.

## Historical Note

In the old days of Lofelt Studio, all of the heavy-lifting algorithmic work was
bundled together into a single Wasm package for ease of importing and
deployment. We've carried over the same bundled structure for the time being,
so this interface includes support for audio data decoding and waveform
generation that wouldn't typically belong in an SDK for haptics.

## `haptics_sdk_napi`

The `haptics_sdk_napi` library contains a Node-API interface to the Haptics SDK,
along with additional functionality from [`audio_analysis`](./audio_analysis/).
