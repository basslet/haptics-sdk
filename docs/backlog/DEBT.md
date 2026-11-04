# Parked debt — haptics-sdk

Anything out of scope, a known follow-up, or deliberately not fixed. Never
delete a row — retire it by strike-through naming the PR/date that closed it.

| ID | What and where | Where parked | Repay trigger | Status |
|---|---|---|---|---|
| DEBT-2026-11-04-no-audio-fixtures | `haptic_audio_analysis` `audio_loading::tests::{load_test_audio_file,multichannel_behavior}` read `resources/audio`, which Meta's public repo does not track, so they fail with NotFound. `tests/run_all.sh` skips their cases with a trailing `::`, because `--skip` matches a substring. | `chore/claude-setup`, 2026-11-04 | Meta publishes the fixtures, or this repo adds its own under `resources/audio` | OPEN |
| DEBT-2026-11-04-no-public-fork | Work bound for Meta has no public fork to open a PR from, and nothing here checks that a branch bound for Meta is free of the tooling layer and `// r[...]` markers; the app repo has `scripts/upstream_pr_check.sh`, this repo has no equivalent. | `chore/claude-setup`, 2026-11-04 | The operator's colleagues report on fork versus standalone, or the first change bound for Meta | OPEN |
| DEBT-2026-11-04-cpp-tests-outside-suite | Upstream's `justfile` `test-all` also runs the CMake tests of `core/renderer_c`, `core/haptic_data_parametric` and `core/renderer_parametric`. `tests/run_all.sh` runs only `cargo test`, although `core/renderer/build.rs` compiles `core/renderer_c` into the Rust renderer. | `chore/claude-setup`, 2026-11-04 | The first change to those directories or to a sync that touches them | OPEN |
| DEBT-2026-11-04-squash-allowed | `CLAUDE.md` says never squash a sync, while the repo allows squash merges and `main` is unprotected, so nothing enforces it. | `chore/claude-setup`, 2026-11-04 | The first sync PR from Meta | OPEN |
| DEBT-2026-11-04-author-email-public | Commits on this repo carry the address in the global git config, and a public repo publishes it with the history. | `chore/claude-setup`, 2026-11-04 | Before any public repo or public fork | OPEN |
| DEBT-2026-11-04-tooling-outside-repo | `tests/run_all.sh` and `.githooks/pre-push` read `_tooling`, and ponytail, the global rules and the shared agents live in `_tooling` and on the operator's Macs. A colleague's clone gets `CLAUDE.md`, the tracey config and the tests, not those. What the suite does without `_tooling` is unmeasured. | `chore/claude-setup`, 2026-11-04 | Before anyone off the operator's Macs works in this repo | OPEN |
