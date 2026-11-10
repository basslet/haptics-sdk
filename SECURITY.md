| | |
| :--- | :--- |
| **Haptics Studio** | [Getting Started](https://github.com/basslet/haptics-studio) · [Design & Integration](https://github.com/basslet/haptics-studio/blob/main/docs/design-and-integration.md) · [Architecture](https://github.com/basslet/haptics-studio/blob/main/docs/architecture.md) |
| **Haptics SDK** | [Overview](README.md) · [Getting Started](docs/getting-started.md) · [Documentation](docs/README.md) |
| **Haptics Studio Examples** | [Overview](https://github.com/basslet/haptics-studio-examples) · [Guidelines](https://github.com/basslet/haptics-studio-examples/blob/main/1%20Design%20Guidelines/README.md) · [Examples](https://github.com/basslet/haptics-studio-examples/blob/main/2%20Design%20Examples/README.md) · [Code](https://github.com/basslet/haptics-studio-examples/blob/main/3%20Integration%20%26%20Code%20Snippets/README.md) · [Knowledge](https://github.com/basslet/haptics-studio-examples/blob/main/4%20Knowledge%20Base/README.md) · [Skills](https://github.com/basslet/haptics-studio-examples/blob/main/5%20Skills/README.md) |
| **General** | [Code of Conduct](CODE_OF_CONDUCT.md) · [Contributing](CONTRIBUTING.md) · [License](LICENSE) · **▸ Security** |

# Security and Privacy

## Reporting a Vulnerability

Do not report a vulnerability in a public issue. Open this repository's **Security** tab and choose **Report a vulnerability**, which keeps the report private. If that button is not shown, open an issue asking [@basslet](https://github.com/basslet) for a private contact, without giving any details of the problem.

Haptics SDK is maintained by volunteers in their free time, so handling is best effort, with no promised response time or bounty.

## Privacy: No Telemetry

This SDK's own source contains no networking code and no telemetry. These statements come from searching the source; traffic from a running build has not been captured. Run these commands in a clone to check the source yourself. Each check is followed by a control that must print the files searched, which shows the search reaches them. A control does not show that a check's words would catch everything; see the limits below.

```bash
# No networking code: prints nothing
git grep -nIiE 'socket|std::net|system\.net|netdb|winsock|getaddrinfo|tcp|udp|curl|httpclient|webclient|webrequest|fhttpmodule|networking|reqwest|hyper|new www|"http"|http\.(get|request)|inet\.h|connect\(|openurl|httpmodule|winhttp|wininet|urlsession' -- core interfaces
# Control: prints every text file searched
git grep -lI "." -- core interfaces

# No telemetry words: prints nothing
git grep -nIiE 'telemetry|analytics|crash.?report|sentry|tracking|phone.?home' -- core interfaces
# Control: prints every text file searched
git grep -lI "." -- core interfaces

# No HTTP crates in any Cargo.toml: prints nothing
git grep -nE 'reqwest|hyper|ureq|curl|surf|isahc|attohttpc|socket2|mio|tokio|async-std|rustls|native-tls|openssl|tungstenite|actix|quinn|smol|http-body|(^|[^a-z0-9_-])(http|h2)( |=|\.|")|dependencies\.(http|h2)' -- '*Cargo.toml'
# Control: prints every Cargo.toml searched
git grep -l "." -- '*Cargo.toml'
```

Limits of this statement:

- A text search finds only the words it names. The lists hold common names, not every way to reach a network, and a program this code started would not show up, so these checks are evidence, not proof.
- It covers this repository's source in `core/` and `interfaces/`, and the names in its `Cargo.toml` files.
- It does not cover `third-party/` (OpenXR, GoogleTest and JsonCpp), because these are submodules and `git grep` does not search inside them.
- It does not cover the code of the crates this project uses, whether listed directly or pulled in by others (see `Cargo.lock`).
- It does not cover Meta's own *Meta Haptics SDK for Unity* and *Meta Haptics SDK for Unreal* packages, the OpenXR runtime that the Unity and Unreal examples hand haptics to, or the game engine or application you build this into.
- It covers the SDK, not the tools around it. [Haptics Studio](https://github.com/basslet/haptics-studio) and [Haptics Studio Examples](https://github.com/basslet/haptics-studio-examples) are separate repositories.
- Building is different from running: Cargo downloads crates, and `git submodule update` downloads the submodules.
