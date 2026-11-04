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

## In flight

| workstream | state | owner | blocked on |
|---|---|---|---|
| Tooling layer | PR #1 open from `chore/claude-setup`; needs `_tooling` `feat/tracey-lint-comments` merged for the gate to read `.rs` markers | this session | architect re-review; `_tooling` PR |

## Unowned

- Requirements for the rest of the crates: only `core/dsp`'s `linspace` and `WindowedMovingRms` are traced.
- Upstream's C++ tests run nowhere here: `DEBT-2026-11-04-cpp-tests-outside-suite`.

## Next action

The architect re-reviews PR #1 after its first rejection, then the outside review runs.

## Standing risks / not verified

- `docs/backlog/EXPECTATIONS.md`: the sync from Meta has never run.
- The skipped audio test functions: `docs/backlog/DEBT.md` `DEBT-2026-11-04-no-audio-fixtures`.
