# Expectations

What is not yet proven, and what would prove it wrong.

| Expectation | What would prove it wrong | How to check |
| --- | --- | --- |
| The **Report a vulnerability** button that [`SECURITY.md`](../../SECURITY.md#reporting-a-vulnerability) describes works once the repository is public and the setting is on. | An account with no role in the repository opens the Security tab and sees no button, or the button does not open a private report form. | Open the Security tab from such an account. |
| GitHub still lists a Code of Conduct when `CODE_OF_CONDUCT.md` is a pointer to [`CONTRIBUTING.md`](../../CONTRIBUTING.md#code-of-conduct) rather than the full text. | With the pointer file on `main`, `.files.code_of_conduct` is `null`. | Run `gh api repos/basslet/haptics-sdk/community/profile --jq .files.code_of_conduct`. A key such as `other` means GitHub no longer recognises the text but still lists the entry, so it does not disprove this. The separate `.files.code_of_conduct_file` field does not count. |
| `just` and the C and C++ tests pass on a clean checkout. This project's maintainers have no record of running them since it split from Meta's original repository, and no workflow runs them. | After the [Quick Start](../../README.md#quick-start) steps, `just` exits with a non-zero status. | Run `just` from the repository root, with `just`, CMake, a C/C++ compiler and Rust installed. |
