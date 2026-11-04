# haptics-sdk — status

Artifact: none   Updated: 2026-11-04   Verified against: branch `chore/claude-setup`

## Goal

A private long-term copy of `meta/meta-haptics-sdk` that the operator's agents work in under the shared tooling, consumed by `basslet/haptics-studio` as a submodule.

## Position

The copy holds upstream's `main` and the fork-versus-standalone question is open with the operator's colleagues. `chore/claude-setup` adds the tooling layer and the first two requirements in `docs/requirements/dsp.md`.

## Decisions taken

| date | decision | by |
|---|---|---|
| 2026-11-04 | A private copy, not a fork, until the colleagues report | operator |
| 2026-11-04 | The SDK is its own repo, traced at its own root | operator |

## In flight

| workstream | state | owner | blocked on |
|---|---|---|---|
| Tooling layer | PR #1 open from `chore/claude-setup`; needs `_tooling` `feat/tracey-lint-comments` merged for the gate to read `.rs` markers | this session | architect review; `_tooling` PR |

## Unowned

- Requirements for the rest of the crates: only `core/dsp`'s `linspace` and `WindowedMovingRms::process` are traced.

## Next action

Triage the architect's review of PR #1, then request the outside review.

## Standing risks / not verified

- `docs/backlog/EXPECTATIONS.md`: the sync from Meta has never run.
- The two skipped audio tests: `docs/backlog/DEBT.md`.
