# Gate Evidence

Status: `ALL REQUIRED GATES PASS`

Evidence class: `Ran`

| Gate | Result | Evidence |
|---|---|---|
| Daymet custody | `PASS` | `sha256sum -c SHA256SUMS`; 10 entries OK |
| package Python controls | `PASS` | 38/38 tests |
| executor validator | `PASS` | 9,261 candidates; 27,783 saturation rows; 18 commands |
| package Rust tests | `PASS` | 34/34 tests |
| Rust formatting | `PASS` | `cargo fmt --check` |
| strict package Clippy | `PASS` | all targets; warnings denied |
| dependency policy | `PASS` | advisories, bans, licenses, and sources OK; unmatched allowances warned |
| Markdown | `PASS` | 46 package files plus catalog and roadmap; 0 errors, 0 warnings |
| diff hygiene | `PASS` | `git diff --check` |
| large-ledger storage | `PASS` | typed failure ledger uses package-scoped Git LFS |
| full correctness profile | `PASS` | exact-terminal-tree nextest run `89776593-82b0-42cb-820e-d4c02ed79498`; 2,101 passed, 29 skipped |
| direct calibration prefix | `PASS` | orders 1–11; `calibration-complete.json` |
| freeze | `PASS` | digest `6066be76...e7349d`; 177 transitive members |
| independent freeze verifiers | `PASS` | verifier A and B checksum-bound receipts |
| formal freeze barrier | `PASS` | receipt summary matches both direct receipts |
| one-time Harvard execution | `PASS_SCORED_NO_REFIT` | 37 candidates; 12,053 days |
| terminal result validator | `PASS` | 9,261 calibration, 37 accepted, 37 holdout |
| terminal exact-rank aggregate audit | `PASS` | 1,598 finite; step histogram `{0: 986, 1: 576, 2: 35, 3: 1}` |
| dual terminal scientific review | `PASS AFTER CORRECTION` | `review-agent-a.md`; `review-agent-b.md` |
| dual terminal verification | `PASS AFTER CORRECTION` | `verification-agent-a.md`; `verification-agent-b.md` |

Harvard execution success is not a scientific fit claim. Its 43.48–72.46 day
errors, 34/37 zero-coverage candidates, and maximum positive interval coverage
of 18.81% are retained as an independent-validation limitation.
