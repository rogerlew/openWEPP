# Gate Results

Evidence class: `Ran`

All commands ran from `/home/workdir/openWEPP` on the exact working tree unless
stated otherwise.

## Passing Direct Gates

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | PASS, exit 0, repeated after review disposition |
| `cargo clippy --test assurance_v2_publication_contract -- -D warnings` | PASS, exit 0 after review disposition |
| `markdown-doc lint --path docs/work-packages/20260809-assurance-draft-publication-defect-closure-001 --format plain` | PASS, 6 files, 0 errors, 0 warnings before later evidence additions |
| `TMPDIR=/home/workdir/openwepp-task-tmp cargo nextest run --test assurance_v2_publication_contract draft_subject_root_is_stable_but_cannot_publish --profile quick` | PASS before review; PASS again after review disposition, 1/1 in 3.401 s |

## Diagnostic And Superseded Runs

| Command | Result | Disposition |
| --- | --- | --- |
| Same isolated test with `TMPDIR=/home/workdir/openWEPP/target/task-tmp` | FAIL, exit 100: `staging and repository roots must be unrelated` | Expected confinement rejection; invocation is invalid because scratch is below the repository. |
| Full `assurance_v2_publication_contract` binary under profile `quick` and external scratch | TIMEOUT, exit 100 after 832.533 s; 5 passed, one negative-matrix test exceeded the 600 s quick-profile timeout, 31 not run | Performance/profile limitation under concurrent fixture-heavy execution, not a test assertion failure. The required full-profile exact-workspace gate supersedes this run. |
| `cargo nextest run -p openwepp-assurance --profile quick` under external scratch | INTERRUPTED after 123.829 s to remove contention from the required full-workspace run; 30 passed, 2 active tests received SIGINT | Not correctness evidence and not represented as a failure. The exact-workspace full-profile gate covers the crate. |

## Required Full-Workspace Gate

Delegated to `comparator_suite_runner` with external
`TMPDIR=/home/workdir/openwepp-task-tmp`. The first run compiled the pre-review
test blob and was explicitly interrupted as superseded evidence after the
review finding changed the test. The terminal rerun compiled reviewed blob
`07e65f289049cfa6a96617a9922f70a06d8f5165` and passed all 2,325 selected tests
in 3,300.706 seconds; 33 declared full-profile tests were skipped. See
`full-workspace-gate.md` and `full-workspace-gate-run.log`.
