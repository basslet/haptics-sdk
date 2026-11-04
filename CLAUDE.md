# haptics-sdk — Claude Code Guide

The operator's long-term copy of Meta's Haptics SDK: the Rust crates behind Haptics Studio.

## Sibling repo

[`basslet/haptics-studio`](https://github.com/basslet/haptics-studio) is the Electron app. It pins a commit of this repo as the submodule `third-party/haptics-sdk`, and its `scripts/install_deps.sh` builds it.

- A Rust change the app needs takes two PRs: one here, then one there that moves the submodule pointer to the merged commit. After a pointer move the app's `scripts/install_deps.sh` rebuilds the native SDK.
- Rust requirements, markers and tests live here. The app repo traces only its own TypeScript and shell, because `tracey-lint` cannot see inside a submodule.

## Copy model

`basslet/haptics-sdk` is a private copy with the full history of `meta/meta-haptics-sdk`, not a GitHub fork: GitHub keeps every fork of a public repo public.

- `upstream` is `https://github.com/meta/meta-haptics-sdk.git`, read only. `origin` is `basslet/haptics-sdk`.
- `origin/main` is `upstream/main` plus the tooling layer: `CLAUDE.md`, `.claude/`, `.config/tracey/`, `.mcp.json`, `.githooks/`, `docs/`, `tests/`, `core/<crate>/tests/requirements.rs`, and the marked block at the end of `.gitignore`.
- Inside upstream's files the tooling layer adds only one-line `// r[...]` marker comments. No reformat and no style edits: each one conflicts at every sync.

## Sync from Meta

Sync here first, then in the app, whose sync PR moves its pointer to the commit merged here. Sync through a PR with a merge commit, never a squash: a squash loses the merge base and the next sync conflicts on everything.

```bash
git fetch upstream
git switch -c sync/upstream-<date> origin/main
git merge upstream/main
bash tests/run_all.sh
git push -u origin sync/upstream-<date>
```

## Work bound for Meta

A branch meant for Meta starts from Meta's code, so it carries none of the tooling layer and no `// r[...]` marker:

```bash
git switch --no-track -c <topic> upstream/main
```

No check guards this here, and the private copy has no public fork to open a PR from: `DEBT-2026-11-04-no-public-fork` in `docs/backlog/DEBT.md`.

## New clone

```bash
git clone https://github.com/basslet/haptics-sdk.git && cd haptics-sdk
git remote add upstream https://github.com/meta/meta-haptics-sdk.git && git fetch upstream
git remote set-url --push upstream no-push   # upstream is read only
gh repo set-default basslet/haptics-sdk   # gh must never target Meta's repo
git config core.hooksPath .githooks       # the pre-push guard
```

The nested submodules under `third-party/` serve the C++ CMake parts outside the Cargo workspace; the suite runs without them.

## Environment

Rust stable, no venv. `.claude/context/builder.md` holds the mechanics.

## Suite

`bash tests/run_all.sh` runs `cargo test` over the workspace and `_tooling/bin/tracey-gate`; a green run on a clean tree writes the push receipt `.githooks/pre-push` asks for. It skips `audio_loading::tests::load_test_audio_file` and `audio_loading::tests::multichannel_behavior` by full path, because Meta does not publish their fixtures (`DEBT-2026-11-04-no-audio-fixtures` in `docs/backlog/DEBT.md`). Upstream's C++ tests stay outside it (`DEBT-2026-11-04-cpp-tests-outside-suite`). This repo has no CI workflow, as upstream has none.

## Code conventions

- Match the surrounding code.
- Requirement markers: `// r[impl <id>]` directly above a function, `// r[verify <id>]` directly above a test. A test on upstream's code that upstream lacks goes in `core/<crate>/tests/requirements.rs`, because upstream's files take only markers.
- 400 lines per file and 60 per function apply to the files this copy adds, not to upstream's.

## Registers (park ⇒ register, can't verify ⇒ register)

Two tracked files, both governed by the global "Two registers" rule:

| File | Holds | Row is written |
|---|---|---|
| `docs/backlog/DEBT.md` | Parked debt — out of scope, known follow-up, deliberately unfixed | In the PR that parks it |
| `docs/backlog/EXPECTATIONS.md` | What the next live run must show for a change not verified against a real run before merge | In the PR that ships the change |

Never delete a row; retire it by strike-through naming the closing PR and date. Reviewers reject a PR that defers or parks without its row.
