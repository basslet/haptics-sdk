# Decisions

Decisions in force, and why.

| Decision | Why |
| --- | --- |
| No Contributor License Agreement. Contributions are licensed under the [LICENSE](../../LICENSE) file ([`CONTRIBUTING.md`, License](../../CONTRIBUTING.md#license)). | One license in and out is enough for an MIT project run by volunteers, with no extra step for contributors. |
| Direct links across the three repositories, in the navigation line of [`CONTRIBUTING.md`](../../CONTRIBUTING.md) and [`SECURITY.md`](../../SECURITY.md). | Haptics SDK, Haptics Studio and Haptics Studio Examples should read as one suite. |
| [`AGENTS.md`](../../AGENTS.md) sets the journal rules: `docs/journal/` holds the current state, not history; findings go in `debt.md`; a row becomes an issue only when a maintainer approves it, and is then deleted. | Git keeps the history, so the journal stays short and true, and issues hold only work a maintainer has accepted. |
| Everything in the repository and its issues is treated as public, including while the repository is private. Anything sensitive stays off GitHub. | The repository, its history and its issues are meant to be open to anyone. |
