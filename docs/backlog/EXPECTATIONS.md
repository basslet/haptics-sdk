# Validation expectations — haptics-sdk

What was not verified against a real run before merge. One row per claim. A
row is closable only by an artifact that could have falsified it. Never delete
a row — retire it by strike-through naming the PR/date that closed it.

| ID | What the next real run must show | Artifact and field | Source | Status |
|---|---|---|---|---|
| VE-2026-11-04-upstream-sync | The first sync from Meta merges with conflicts only beside `// r[...]` markers, and `tests/run_all.sh` is green on the merge. | That sync PR's diff and the suite's exit code | `CLAUDE.md` "Sync from Meta", never run | OPEN |
