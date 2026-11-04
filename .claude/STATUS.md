# haptics-sdk — status

Artifact: none   Updated: 2026-11-04   Verified against: branch `chore/claude-setup`

## Goal

A private long-term copy of `meta/meta-haptics-sdk` that the operator's agents work in under the shared tooling, consumed by `basslet/haptics-studio` as a submodule.

## Position

The copy holds upstream's `main` and the fork-versus-standalone question is open with the operator's colleagues. `chore/claude-setup` adds the tooling layer and the first requirements in `docs/requirements/dsp.md`.

## Decisions taken

| date | decision | by |
|---|---|---|
| 2026-11-04 | A private copy, not a fork, until the colleagues report | operator |
| 2026-11-04 | The SDK is its own repo, traced at its own root | operator |
| 2026-11-04 | The suite skips the fixture tests by exact name with `--exact`, so only a test with one of these twelve full names is skipped, in any crate, since a name carries no crate (a `harness = false` target excepted) | operator |
| 2026-11-04 | Outside review through OpenRouter is allowed for this repo and `basslet/haptics-studio`; it applies once that line is on the base branch | operator |
| 2026-11-04 | The operator reviews and merges this repo's PRs | operator |

## In flight

| workstream | state | owner | blocked on |
|---|---|---|---|
| Tooling layer | PR #1 open from `chore/claude-setup`; needs `_tooling` PR #133 (`feat/tracey-lint-comments`) merged first, because `tracey-lint` checks `.rs` markers only there and this repo's config cites that branch's probe cases | this session | architect re-review; `_tooling` PR #133 |

## Unowned

- Requirements for the rest of the crates: only `core/dsp`'s `linspace` and `WindowedMovingRms` are traced.
- Upstream's C++ tests run nowhere here: `DEBT-2026-11-04-cpp-tests-outside-suite`.

## Next action

The architect re-reviews PR #1, then `/ponytail-review` and `pr-review` run on it; `_tooling` PR #133 merges first.

## Standing risks / not verified

- `docs/backlog/EXPECTATIONS.md`: the sync from Meta has never run.
- The skipped audio test functions: `docs/backlog/DEBT.md` `DEBT-2026-11-04-no-audio-fixtures`.
