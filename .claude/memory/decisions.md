# Decisions — haptics-sdk

## 2026-11-04 — decision: outside review through OpenRouter is allowed for this repo

- what: `.claude/context/build-lead.md` carries `outside-review: allowed`, so `pr-review` may send this repo's PR diffs to OpenRouter when Copilot fails, and `external_refute.sh` may send evidence from it.
- why: the operator's decision, 2026-11-04, for this repo and `basslet/haptics-studio`. `pr-review` reads the line from the base branch, so it applies once PR #1 has merged.
- refs: `_tooling` `.claude/memory/decisions.md` "2026-11-01 — decision: `PR_REVIEWER=outside` does not override a repo's opt-in".

## 2026-11-04 — decision: the operator reviews and merges this repo's PRs

- what: agents prepare a PR and run its automated review; the operator reviews it and merges it.
- why: the operator's decision, 2026-11-04, given while allowing agents to merge `_tooling` PRs in the same session.
- refs: global rules, "Who decides".

## 2026-11-04 — decision: a private copy, not a fork, until the operator's colleagues report

- what: `basslet/haptics-sdk` is a private copy of `meta/meta-haptics-sdk`'s `main`, not a GitHub fork. Fork or standalone stays open until the colleagues report back.
- why: the operator's decision, 2026-11-04, after a public fork was created and then set aside. A copy can become public later; a fork's network cannot be rejoined once left.
- refs: `basslet/haptics-studio` `docs/backlog/DEBT.md` `DEBT-2026-11-04-no-public-fork`.

## 2026-11-04 — decision: the SDK is its own repo, traced at its own root

- what: `basslet/haptics-sdk` carries its own requirements, markers, suite and registers; `basslet/haptics-studio` consumes it as the submodule `third-party/haptics-sdk`.
- why: the operator's decision. A Rust requirement then fits one PR, and `tracey-lint` cannot see files inside a submodule. The cost is two PRs when the app must move its pointer.
- refs: `CLAUDE.md` "Sibling repo"; `_tooling` `scripts/checks/probe_tracey.sh` cases R1-R6.
