| | |
| :--- | :--- |
| **Haptics Studio** | [Overview](https://github.com/basslet/haptics-studio) · [Design & Integration](https://github.com/basslet/haptics-studio/blob/main/docs/design-and-integration/README.md) · [Source Code](https://github.com/basslet/haptics-studio/blob/main/docs/building.md) · [Architecture](https://github.com/basslet/haptics-studio/blob/main/docs/architecture.md) |
| **Haptics SDK** | [Overview](README.md) · [Getting Started](docs/getting-started.md) · [Documentation](docs/README.md) |
| **Haptics Studio Examples** | [Overview](https://github.com/basslet/haptics-studio-examples) · [Guidelines](https://github.com/basslet/haptics-studio-examples/blob/main/1%20Design%20Guidelines/README.md) · [Examples](https://github.com/basslet/haptics-studio-examples/blob/main/2%20Design%20Examples/README.md) · [Code](https://github.com/basslet/haptics-studio-examples/blob/main/3%20Integration%20%26%20Code%20Snippets/README.md) · [Knowledge](https://github.com/basslet/haptics-studio-examples/blob/main/4%20Knowledge%20Base/README.md) · [Skills](https://github.com/basslet/haptics-studio-examples/blob/main/5%20Skills/README.md) |
| **General** | [Code of Conduct](CODE_OF_CONDUCT.md) · **▸ Contributing** · [License](LICENSE) · [Security](SECURITY.md) |

# Contributing to Haptics SDK

We want to make contributing to this project as easy and transparent as possible. Everyone taking part follows the [Code of Conduct](CODE_OF_CONDUCT.md).

## Pull Requests

We welcome your pull requests. They are reviewed and merged in this repository.

1. Fork the repository and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs, update the documentation.
4. If you've changed a documentation page, run `node scripts/check_doc_links.js`. It checks links and the navigation block at the top of each page; `node scripts/check_doc_links.js --fix` writes the block. In a fork under another name, or a clone without an `origin`, set the environment variable `DOCS_REPO` to `haptics-sdk` before running it; an `origin` named after one of the three repositories still takes precedence.
5. Make sure the tests pass: run `just` (see [Building](README.md#building)) and `cargo test --workspace`.
6. Keep code changes and documentation changes in separate commits, so code changes can be taken to other copies of this project without the docs.

## Issues

We use GitHub issues to track bugs and requests. Please describe the problem clearly, with enough detail to reproduce it, and include the commit you use and your operating system. The project is maintained by volunteers in their free time, so reviews and answers are best effort, with no promised response time.

## Security

Do not report security problems in a public issue. See [SECURITY.md](SECURITY.md).

## License

By contributing, you agree that your contributions are licensed under the [LICENSE](LICENSE) file in the root of this repository. No Contributor License Agreement is required.

## Relationship to Meta

Haptics SDK was created by the Meta Haptics team (formerly Lofelt) and released by Meta under the MIT license. This repository is an independent project, not affiliated with, endorsed by, or supported by Meta Platforms, Inc. Meta's internal process, its Contributor License Agreement and its bounty program do not apply here.
