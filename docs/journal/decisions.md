# Decisions

- No Contributor License Agreement. Contributions are licensed under the [LICENSE](../../LICENSE) file ([`CONTRIBUTING.md`, License](../../CONTRIBUTING.md#license)), because this is an independent project and Meta's License Agreement does not apply to it.
- Doc pages link directly across the three repositories (Haptics SDK, Haptics Studio, Haptics Studio Examples), which read as one suite. The navigation block at the top of every documentation page does this.
- The navigation block and its lint come from four files in `scripts/` that are the same in all three repositories: [`doc_nav.js`](../../scripts/doc_nav.js), [`check_doc_links.js`](../../scripts/check_doc_links.js) and their tests. A change is made in one repository and copied to the other two, and `node scripts/doc_nav.js --compare` names any repository whose copy on `main` differs ([basslet/haptics-studio#32](https://github.com/basslet/haptics-studio/issues/32)).
- The repository and its issues are treated as public while the repository is still private, because they are meant to be open to anyone.
