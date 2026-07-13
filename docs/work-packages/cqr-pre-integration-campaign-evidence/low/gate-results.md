# Low Final Gate Results

Evidence class: **Ran + Static**

Status: `PASS`

The measurement source is `9145d288`; the ordinary closure source is
`8e0f7367`. The intervening delta is limited to the test-only lint corrections
documented in `final-metrics.md`.

| Gate | Result | Evidence |
| --- | --- | --- |
| `cargo fmt --check` | PASS | 2.09 s; `final/gates/fmt.log` |
| workspace all-target Clippy, warnings denied | PASS | 5.22 s; `final/gates/clippy.log` |
| PMET integration | PASS | 14/14; `final/gates/focused-pmet-integration.log` |
| L-10 private parser matrix | PASS | 5/5; `final/gates/focused-l10-private.log` |
| workspace full profile | PASS | 1,944/1,944, 3 skipped; `final/gates/nextest-full.log` |
| `cargo deny check` | PASS | 1.08 s; `final/gates/deny.log` |
| exact Markdown lint | PASS | 84 files, zero findings; `final/gates/markdown.log` |
| `git diff --check` | PASS | `final/gates/diff.log` |

The first all-target Clippy attempt failed only on four test `float_cmp`
diagnostics and one exhaustive-test `too_many_lines` diagnostic. Commit
`8e0f7367` corrects those findings; the passing rerun above supersedes the
retained initial-failure log.

At measurement commit, all six focused package gates also passed: input
contract 33, legacy bridge 15, meteorology 18, runner 213, sim contract 3, and
watershed orchestrator 129 tests, for 411 total. The measurement-source full
profile passed 1,944/1,944 as well.

No comparator, release, cohort, or anti-evasion command was invented: the Low
test-only changes do not alter external-authority suite posture and the
campaign contract names no concrete additional command. All required final
gates pass with no deferred finding.
