# Decisions — haptics-sdk

## 2026-11-04 — decision: the SDK is its own repo, traced at its own root

- what: `basslet/haptics-sdk` carries its own requirements, markers, suite and registers; `basslet/haptics-studio` consumes it as the submodule `third-party/haptics-sdk`.
- why: the operator's decision. A Rust requirement then fits one PR, and `tracey-lint` cannot see files inside a submodule. The cost is two PRs when the app must move its pointer.
- refs: `CLAUDE.md` "Sibling repo"; `_tooling` `scripts/checks/probe_tracey.sh` cases R1-R6.
