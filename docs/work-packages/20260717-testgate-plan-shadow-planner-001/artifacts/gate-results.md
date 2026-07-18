# Gate Results

Evidence class: `Ran`

## Focused Remediation Gates

- Planner/verifier focused suite: PASS, five directly affected tests, run
  `2f59c4bd-eb94-49f8-b015-00f8a1717564`.
- Final deletion/reconciliation regression: PASS, run
  `6e4d8e2c-b69d-4f99-b04d-87c9a3351191`.
- Execution-matrix schema fixture: PASS, run
  `583ee3ff-ed44-4aae-a12a-ddfb47055444`.
- Production gate-definition admission: PASS, run
  `8fadce07-0f01-4f31-b66a-64f7ef7be459`.
- Planner crate Clippy with warnings denied: PASS on the final source.

## Single Terminal Conservative Sequence

The package-authorized closure runner executed these commands serially. No
successful command was repeated.

| Gate | Result | Evidence |
| --- | --- | --- |
| `cargo fmt --check` | PASS | 3 s |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | 10 s |
| `cargo nextest run --workspace --profile full` | PASS | run `14ea7c1c-9c9d-4a2d-a4b4-9417e88ad98d`; 2,115 passed, 5 profile-skipped, 26 slow; 570.681 s |
| `cargo deny check` | PASS | advisories, bans, licenses, and sources all OK; 1 s |
| `bash tools/release/run_adjudicated_crap_gate.sh --base-ref 0873bdae960f7f8c76401845acb476750fdd020e` | FAIL | 14 raw, 2 adjudicated, 12 actionable; 2,673 s |

The CRAP run's before/after/final source manifests were identical: 244 Rust
sources, SHA-256
`a4a45f0d32db35f3369c351f517da8a41ec929860e5ab12eacde1c10e995527f`.
The report is retained locally at
`target/adjudicated-crap/adjudicated-crap-report.json` with SHA-256
`d52a7bb7ec11f6db563b094fab95aaac53e5ee5815c5fef3e968dd4f3e91d8ff`.

Actionable rows are four ledger verifiers, plan output confinement, three
planner paths, and four receipt/trust/reuse paths. This `FAIL` blocks package
completion. It is not waived, deferred, or converted to PASS.
