# Refute: test filter in `tests/run_all.sh`, record 1

Head: `d7da3a1`  <-  `git rev-parse --short HEAD`
Status of this verdict: complete. "Claude verdict" judged the old filter, "External verdict" the exact-name filter, and "Who judged what" says which claim each pass covers.

## Claim

`cargo test ... -- --skip audio_loading::tests::load_test_audio_file:: --skip audio_loading::tests::multichannel_behavior::` skips only the fixture-dependent test cases that cannot pass in this copy (they read audio fixture files that Meta's repo does not ship, and fail with NotFound), and cannot hide any other test that exists or is added later outside those two groups.

Prior claims reversed on this object: (1) a bare `--skip load_test_audio_file` name selects just those tests; (2) the form without a trailing `::` is exact.

## Claude verdict

**REFUTED**, on the clause "cannot hide any other test ... added later outside those two groups". The clauses about the tests that exist at this head stand.

Deciding artifact: 7 planted tests outside the two groups were removed by the literal filter, across 3 crates, and all 7 failed when the filter was off. 6 of 6 controls in the same filtered run were listed and failed.

Command: `bash p2.sh` in a clone of `d7da3a1` with `plant.py` applied, using its own Cargo target directory. It diffs `cargo test --workspace --locked --no-fail-fast --jobs 8 -- --list` with and without the two `--skip` arguments.

| Planted test, full name | Crate and binary | Under the literal filter |
|---|---|---|
| `my_audio_loading::tests::multichannel_behavior::p_hidden_a_prefixed_module_same_crate` | `haptic_audio_analysis` lib | hidden |
| `nested::audio_loading::tests::load_test_audio_file::p_hidden_b_nested_path_same_crate` | `haptic_audio_analysis` lib | hidden |
| `audio_loading::tests::multichannel_behavior::p_hidden_c_same_path_other_crate` | `haptic_dsp` lib | hidden |
| `wav_audio_loading::tests::load_test_audio_file::p_hidden_d_prefixed_other_crate` | `haptic_renderer` lib | hidden |
| `case_audio_loading::tests::load_test_audio_file::p_hidden_e_test_case_macro_other_module` | `haptic_audio_analysis` lib | hidden |
| `audio_loading::tests::load_test_audio_file::p_hidden_f_integration_binary` | `haptic_dsp` `tests/planted_it.rs` | hidden |
| `core/dsp/src/lib.rs - planted_doc::audio_loading::tests::multichannel_behavior::p_hidden_g_doctest (line 49)` | `haptic_dsp` doc tests | hidden |
| `audio_loading::tests::load_test_audio_file_p_ctl_sibling` (control) | `haptic_audio_analysis` lib | ran, failed |
| `audio_loading::tests::multichannel_behavior_p_ctl_sibling` (control) | `haptic_audio_analysis` lib | ran, failed |
| `audio_loading::tests::p_ctl_unrelated` (control) | `haptic_audio_analysis` lib | ran, failed |
| `dsp_ctl::p_ctl_other_crate` (control) | `haptic_dsp` lib | ran, failed |
| `p_ctl_integration` (control) | `haptic_dsp` `tests/planted_it.rs` | ran, failed |
| `core/dsp/src/lib.rs - planted_doc_ctl::p_ctl_doctest (line 55)` (control) | `haptic_dsp` doc tests | ran, failed |

Tests `c` and `f` carry the exact group text in a different crate, so they are outside the groups only if a group means the one in `haptic_audio_analysis`. Tests `a`, `b`, `d`, `e` and `g` are outside the groups on any reading. If a group is defined as "any name containing the pattern", the clause is true by definition and says nothing.

### Check 1: Coverage

Evidence spans `n` = 173 listed tests at one head, one toolchain (`cargo 1.97.1`, `rustc 1.97.1`), default features, plus 15 planted tests (9 hidden, 6 controls). 173 <- `grep -E ': test$' c_noskip.list.log | sort | wc -l`. No existing test covers a name outside the groups that contains a pattern, so evidence from the existing suite could never test the "added later" clause. Only planted cases reach it.

Which tests the line runs: 5 workspace members, 6 test binaries, 4 doc-test targets  <-  `grep -E '^(     Running|   Doc-tests)' c_skip.run.log`. No `Cargo.toml` sits outside the workspace  <-  `find . -name Cargo.toml -not -path './third-party/*'`. `--all-features` lists the same 173, 0 extra  <-  `comm -13 c_noskip.ids c_allfeat.ids | wc -l`. The arguments reach every binary and the doc tests; test `g` shows the doc-test case.

### Check 2: Circularity

Not circular. The baseline is measured with no filter, and the filter is measured as a difference from it. Each result below comes from a build that does not depend on the filter, because `--skip` acts at run time.

### Check 3: Derivation (unmodified clone, own target directory)

| Fact | Value | Command |
|---|---|---|
| Listed, no filter | 173 | `wc -l < c_noskip.ids` |
| Listed, literal filter | 161 | `wc -l < c_skip.ids` |
| Removed by the filter | 12, all under the two group names | `comm -23 c_noskip.ids c_skip.ids \| grep -cE '^audio_loading::tests::(load_test_audio_file\|multichannel_behavior)::'` |
| Failures, no filter | 12 | `grep -cE '^test .*\.\.\. FAILED$' c_noskip.run.log` |
| `kind: NotFound` lines, no filter | 12 | `grep -c 'kind: NotFound' c_noskip.run.log` |
| Panic sites | 10 at `audio_loading.rs:199`, 2 at `:220`, both `fs::read(path).unwrap()` | `grep -oE 'panicked at [^ ]+' base_noskip.log \| sort \| uniq -c` |
| Failures, literal filter | 0, exit 0 | `grep -cE '^test .*\.\.\. FAILED$' c_skip.run.log` |
| Audio files tracked here | 0 | `git ls-files \| grep -cE '\.(wav\|flac\|mp3\|ogg\|aiff)$'` |
| Audio files upstream, `meta/meta-haptics-sdk` HEAD, `truncated=false` | 0 of 233 paths, 0 under `resources/audio`; control: 35 under `resources/` | `gh api 'repos/meta/meta-haptics-sdk/git/trees/HEAD?recursive=1' --jq '.tree[].path'` |

Clauses that stand at this head: every existing skipped case fails for the stated reason, and the filter removes nothing else that exists.

Control on cause: with synthetic WAV fixtures written to `resources/audio`, 4 of the 12 pass with no filter (2 `load_test_audio_file` WAV cases, 2 `multichannel_behavior`)  <-  `grep ... fix_noskip.log`. The other 8 (flac, mp3, ogg, aiff) were not given a file. Their evidence is the NotFound panic only, so whether they pass with real fixtures is unknown.

### Check 4: Feasibility

The filter fires on 12 cases, not zero. Meta's fixtures would be needed to pass all of them.

### Check 5: The prior two failures

The current claim shares their mechanism. Prior claim 2 failed because `--skip` is an unanchored substring match, at the right edge of the pattern. The trailing `::` anchors the right edge against sibling names, which the two `_p_ctl_sibling` controls confirm. The left edge stays open, and the name carries no crate: tests `a`, `d`, `e` extend the pattern on the left, `b` and `g` nest it, `c` and `f` repeat it in another crate. Each correction closed the case last seen and then asserted the whole class closed.

### "Added later", inside the groups

The claim's "outside" wording makes no statement here, but its first clause ("skips only the fixture-dependent test cases that cannot pass") does. Two new `#[test_case]` cases added to the real groups are hidden. With fixtures present, `p_inside_group_multichannel` fails on `assertion failed: mean_data.abs() < 1e-10`, not NotFound, and the filter still hides it; the 4 passing WAV cases are hidden too. Under the filter neither group produces a line and the only sign is `17 filtered out`  <-  `grep -E '^test result' fix_skip.log`. So the clause is true only while `resources/audio` is absent.

### Size of the exposure

[Likely] The refuted case is narrow in practice: a hidden test needs a module path ending in `audio_loading::tests::load_test_audio_file::` or `...multichannel_behavior::`, and 0 such tests exist today outside the groups. The claim said "cannot", and that is what falls.

[Certain] The sentences "cannot hide" and "skips only" do not appear in the repo's files  <-  `grep -nE 'cannot hide|only the fixture|skips only' CLAUDE.md docs/backlog/DEBT.md .claude/STATUS.md tests/run_all.sh tmp/pr_sdk.md` (rc=1, no lines). Those files say "the cases under `audio_loading::tests::...::`" and "the trailing `::` is what keeps a sibling test", and the sibling statement held in this run. Whether "the cases under" reads as one module in one crate is the lead's call.

### Limits of this pass

- One shared-target-directory error, found and removed: an `--all-features` listing of the base clone showed planted tests because `clone` and `plant` shared a Cargo target directory. Experiments 1 and 2 were rerun in separate directories (`target_clean`, `target_plant`), and every number above comes from those reruns except the panic sites (`base_noskip.log`, which ran before any plant existed) and the fixture control.
- The fixture control (`fix_noskip.log`, `fix_skip.log`) ran in the shared directory on the planted tree. Its output contains the planted names, so it ran the planted build, but it was not repeated in a separate directory.
- `libtest`'s source was not read. The substring behaviour is established from observed results, on this toolchain only.
- Stable `--exact` and other run forms were not tried, since the claim concerns this line.

## External verdict

The operator allowed the send on 2026-11-04 and chose the exact-name filter (`--exact` with the twelve full names) over keeping the trailing `::`. The bundle sent holds that new filter's claim and two-clone comparison, followed by this pass's evidence on the old filter.

`external_refute.sh <bundle>` with no model failed: HTTP 404, `No endpoints found for openai/o3-mini` (`_tooling` `DEBT-2026-11-04-external-refute-default-model`). `external_refute.sh <bundle> openai/gpt-5.6-sol`, the model `bin/outside-review` uses by default, answered rc 0.

Comparison run before the send, names read from `tests/run_all.sh`: unmodified clone 173 listed, 161 with the filter, 12 removed, 0 outside the twelve, filter run rc 0. Planted clone 188 listed, 176 with the filter, 12 removed, 0 outside the twelve, 15 of 15 plants kept, filter run rc 101 with 15 FAILED, all plants.

Verdict, in full:

> 1. **Coverage:** The experiments establish the claim for ordinary libtest and rustdoc harnesses in the tested workspace. They cover unit tests, integration tests, doctests, and `test_case`-generated names. They do **not** cover `harness = false` targets.
>
> 2. **Counterexample:** Add this target:
>
> ```toml
> [[test]]
> name = "custom_tests"
> path = "tests/custom_tests.rs"
> harness = false
> ```
>
> ```rust
> // tests/custom_tests.rs
> fn main() {
>     // A custom harness may ignore all arguments forwarded after `cargo test --`.
>     eprintln!(
>         "running audio_loading::tests::load_test_audio_file::_sine_16_44100_aiff_44100_1_0_expects"
>     );
>     panic!("the nominally skipped test ran");
> }
> ```
>
> Cargo forwards `--exact --skip ...` to this executable, but `harness = false` means libtest is not interpreting them. The executable may ignore those arguments, reject them, or give them unrelated semantics. Thus even a case assigned one of the twelve exact names can run.
>
> 3. **Circularity and derivation:** For libtest-managed targets, the experiment is persuasive rather than circular: the removed-name comparison and planted failures independently show that `--exact` makes the skip comparisons exact. The reported counts derive correctly: 173−161=12 and 188−176=12; all 15 planted near-matches remained runnable and failed.
>
> 4. **Feasibility:** In the current measured standard-harness targets, the rule fires on exactly twelve listed cases. That does not establish the unrestricted “any other test added later” claim once custom harnesses are allowed.
>
> VERDICT: REFUTED, A later `harness = false` test target can ignore the forwarded libtest arguments and run a case having one of the twelve supposedly skipped names.

Disposition: the refutation holds for a `harness = false` target only, and none exists in the workspace (`git grep -c 'harness *= *false' -- '*Cargo.toml'`). Such a target could run a test the list names, or read the arguments another way. `tests/run_all.sh` carries that limit as a `shortcut:` line. For libtest and rustdoc targets the verdict confirms the filter.

## Who judged what

- The Claude pass judged the old filter, two `--skip` group prefixes: refuted, planted tests hidden.
- The external model judged the exact-name filter, `--exact` with the twelve full names: refuted only for a `harness = false` target.
- The architect's pass on the exact-name filter agrees for libtest and rustdoc names, and adds one case: a test with one of the twelve full names in another crate is also skipped, because a name carries no crate. A test that fails when run, in its own binary: `./b --exact --skip audio_named` gives `1 passed; 0 failed; 1 filtered out`, and `./b` alone gives `1 passed; 1 failed` (`rustc --test b.rs -o b`, 2026-11-04).
