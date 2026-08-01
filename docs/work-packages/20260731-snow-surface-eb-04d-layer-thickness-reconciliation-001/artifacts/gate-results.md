# Gate Results

Status: `PASS`

Evidence class: `Ran`

| Gate | Result | Evidence |
| --- | --- | --- |
| Formatting | PASS | `cargo fmt --all -- --check` |
| Strict Clippy | PASS | `cargo clippy --workspace --all-targets -- -D warnings` |
| Diff hygiene | PASS | `git diff --check` |
| Scoped Markdown | PASS | package 34 files plus contract, index, roadmaps, and catalog; 0 errors/warnings |
| Authority anti-evasion | PASS | `check_authority_suite_antievasion.sh` |
| Required-suite obligations | PASS | 3/3, run `727f871d-0343-4472-96df-70586b163e25` |
| Unit registry | PASS | 21/21 plus check |
| EB-04D contract/runtime | PASS | 27/27, run `30610ae3-17d3-41fa-8ad1-5e63c2b2800a` |
| Partial-sublimation/target-trim | PASS | 3/3, run `06f547f1-cac4-48d2-9a84-eb8e66a90430` |
| Exact replay | PASS | two 16,437-day trajectories; `acceptance_passes=true` |
| Workspace quick | PASS | 2,128/2,128; 36 profile skips; run `791467e8-8f07-42b8-9712-5c60c69fd709`; 2,195.935 s |
| Workspace frost | PASS | 329/329; 1,889 profile skips; run `61486d3b-760a-43d9-914c-f49252e282d6`; 522.442 s |
| Critical full workspace | PASS | 2,177/2,177; 29 profile skips; run `47b5394b-5943-4a25-b5f4-627c7e390240`; 2,223.163 s |

Two earlier quick/full attempts were deliberately terminated without a
disposition when primary review identified new conservation defects; only the
final-source runs above and the final full gate are admissible. The
suite runner's first stale log was moved from an unauthorized root `artifacts/`
directory to `target/eb04d-suite-logs/` and is not closure evidence.

A terminal repository-wide Markdown observation found 15 unrelated pre-existing
broken links in historical/synthetic records. Every EB-04D-owned and directly
updated documentation surface passes the canonical linter; the ambient findings
are outside this package's write set and disposition.
