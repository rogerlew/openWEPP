# Gate results

Status: PASS
Evidence mode: Ran

| Gate | Result | Evidence |
| --- | --- | --- |
| Focused parser contract | PASS | final 27/27, 0 skipped |
| `cargo fmt --check` | PASS | final state exit 0, 1.98s |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | exit 0, 16.19s; production source unchanged afterward |
| `cargo nextest run --workspace --profile full` | PASS | exact final state exit 0, 586.14s command / 585.064s nextest; 1,730 passed, 0 failed, 3 skipped, 4 slow |
| `cargo deny check` | PASS | exit 0, 1.02s; advisories/bans/licenses/sources all ok |
| Target coverage | PASS | 97.397% lines / 98.065% regions; 25/25 functions; minimum function 85.366% |
| Target CRAP | PASS | zero eligible rows above 30; maximum 17 |
| Markdown docs | PASS | 37 files, 0 errors, 0 warnings |
| `git diff --check` | PASS | final state exit 0, 0.02s |

The delegated runner also completed a pre-final full run (1,730 pass / 3 skip)
before the exhaustive fixture assertions were added. The table accepts only the
subsequent exact-final-state rerun. No gate failed and no failure attribution is
required. The three nextest skips are existing configured skips and are not in
the fixed-date suite.
