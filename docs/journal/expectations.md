# Expectations

What is not yet proven, and what would prove it wrong.

| Expectation | What would prove it wrong | How to check |
| --- | --- | --- |
| The **Report a vulnerability** button that [`SECURITY.md`](../../SECURITY.md#reporting-a-vulnerability) describes works once the repository is public and the setting is on. | An account with no role in the repository opens the Security tab and sees no button, or the button does not open a private report form. | Open the Security tab from such an account. |
| `just` and the C and C++ tests pass on a clean checkout. This project's maintainers have no record of running them since it split from Meta's original repository, and no workflow runs them. | After the [Quick Start](../../README.md#quick-start) steps, `just` exits with a non-zero status. | Run `just` from the repository root, with `just`, CMake, a C/C++ compiler and Rust installed. |
| The README banner shows `resources/images/logo/Banner-SDK-dark.png` in GitHub's dark theme and `resources/images/logo/Banner-SDK-light.png` in the light theme, 800 pixels wide. The two files look alike, except that the wordmark is a lighter grey in the dark one. | In a window wider than 800 pixels, the banner is another width, shows no image, or loads the other theme's file. | Open the repository page on GitHub in each theme, signed in while the repository is private. Open the banner image in a new tab and read its file name, and check its width. |
