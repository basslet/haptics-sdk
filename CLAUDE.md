# haptics-sdk — Claude Code Guide

The operator's long-term copy of Meta's Haptics SDK: the Rust crates behind Haptics Studio.

## Sibling repo

[`basslet/haptics-studio`](https://github.com/basslet/haptics-studio) is the Electron app. It pins a commit of this repo as the submodule `third-party/haptics-sdk`, and its `scripts/install_deps.sh` builds it.

- A Rust change the app needs takes two PRs: one here, then one there that moves the submodule pointer to the merged commit.
- Rust requirements, markers and tests live here. The app repo traces only its own TypeScript and shell, because `tracey-lint` cannot see inside a submodule.

## Copy model

`basslet/haptics-sdk` is a private copy with the full history of `meta/meta-haptics-sdk`, not a GitHub fork: GitHub keeps every fork of a public repo public.

- `upstream` is `https://github.com/meta/meta-haptics-sdk.git`, read only. `origin` is `basslet/haptics-sdk`.
- `origin/main` is `upstream/main` plus the tooling layer: `CLAUDE.md`, `.claude/`, `.config/tracey/`, `.mcp.json`, `.githooks/`, `docs/`, `tests/`, and the marked block at the end of `.gitignore`.
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

## New clone

```bash
git clone https://github.com/basslet/haptics-sdk.git && cd haptics-sdk
git remote add upstream https://github.com/meta/meta-haptics-sdk.git && git fetch upstream
gh repo set-default basslet/haptics-sdk   # gh must never target Meta's repo
git config core.hooksPath .githooks       # the pre-push guard
```

The nested submodules under `third-party/` serve the C++ CMake parts outside the Cargo workspace; the suite runs without them.

## Environment

Rust stable, no venv. `.claude/context/builder.md` holds the mechanics.

## Suite

`bash tests/run_all.sh` runs `cargo test` over the workspace and `_tooling/bin/tracey-gate`; a green run on a clean tree writes the push receipt `.githooks/pre-push` asks for. It skips two audio tests whose fixtures Meta does not publish (`DEBT-2026-11-04-no-audio-fixtures` in `docs/backlog/DEBT.md`). This repo has no CI workflow, as upstream has none.

## Code conventions

- Match the surrounding code.
- Requirement markers: `// r[impl <id>]` directly above a function, `// r[verify <id>]` directly above a test. Tests stay inline in `#[cfg(test)]` modules, as upstream's do.
- 400 lines per file and 60 per function apply to the files this copy adds, not to upstream's.

## Registers (park ⇒ register, can't verify ⇒ register)

Two tracked files, both governed by the global "Two registers" rule:

| File | Holds | Row is written |
|---|---|---|
| `docs/backlog/DEBT.md` | Parked debt — out of scope, known follow-up, deliberately unfixed | In the PR that parks it |
| `docs/backlog/EXPECTATIONS.md` | What the next live run must show for a change not verified against a real run before merge | In the PR that ships the change |

Never delete a row; retire it by strike-through naming the closing PR and date. Reviewers reject a PR that defers or parks without its row.

## External API probes (probe-first gate)

Governed by the global "Probe before you integrate" rule. This repo's slots:

| Slot | Path |
|---|---|
| Committed probe scripts | `scripts/checks/<vendor>_<subject>_probe.sh` |
| Probe output (git-invisible, input not record) | `tmp/<name>.log` |
| Contract of record | Dated section in the `docs/backlog/` design note |
| Settled unit/currency for one field | Comment in the module that depends on it |
| Post-merge verification | A row in `docs/backlog/EXPECTATIONS.md` |
