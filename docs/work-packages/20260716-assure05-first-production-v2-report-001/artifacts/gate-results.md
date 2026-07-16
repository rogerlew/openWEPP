# ASSURE-05 Gate Results

Status: PASS — TERMINAL TECHNICAL CLOSURE

Evidence class: Ran

Frozen base and current `HEAD`:
`01ed70550a4e371e99afe35c4bdd4d9b667e812c`.

| Required gate | Latest disposition |
| --- | --- |
| `cargo fmt --check` | PASS — exit 0; 2.28 s |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS — exit 0; 0.91 s |
| Full Nextest | PASS — exit 0; run `f7960089-7439-420e-aa3b-293c7fa5d773`; 2,049 passed, 3 skipped, 4 slow; 576.031 s Nextest time |
| Deny | PASS — exit 0; 1.04 s |
| Fresh adjudicated CRAP | PASS — exit 0; raw 2, adjudicated 2, actionable 0, touched production files 0, touched maximum not applicable |
| Assurance terminal confirmation | PASS — validate/plan and two unrelated narrative-seeded build/check roots; complete trees byte-identical |
| Markdown validation | PASS — exit 0; 25 files, 0 errors, 0 warnings |
| `git diff --check` | PASS — exit 0; no output |

The initial strict-Clippy failure and the renewed full-Nextest publication-
fixture failure remain preserved as Attempts 1 and 2 in
`artifacts/heavy-gate-runner.md`. Attempt 3 records the complete passing
sequence, logs, timings, fresh CRAP identities, and staging roots. This is a
technical gate result only; it does not satisfy the package's named-human
approval boundary.
