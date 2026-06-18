# Gate Results

Evidence class: Ran

Status: complete.

Required gates:

| Gate | Result |
|---|---|
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `cargo test --workspace` | PASS |
| `cargo deny check` | PASS |
| `markdown-doc lint --path docs/work-packages/20260618-basecond01-ssc-harmonic-normalization-defect-closure-001 --format plain` | PASS, 22 files validated, 0 errors, 0 warnings |
| `git diff --check` | PASS |

Focused gates:

| Gate | Result |
|---|---|
| `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests::soil_runtime_surface_contains_canonical_state_symbols -- --nocapture` | PASS after production edit |
| `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests -- --nocapture` | PASS, 79 passed |
| `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests::soil_runtime_surface_projects_harmonic_vertical_ssc_below_top_interval -- --nocapture` | PASS |
| `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests::soil_runtime_surface_rejects_non -- --nocapture` | PASS, 2 passed |
| `cargo test -p openwepp --test parser_runtime_seam_integration -- --nocapture` | PASS, 49 passed |

Notes:

- `cargo test --workspace` includes
  `tests/integration/auth11_required_suite_obligation_guards_contract.rs`.
- Anti-evasion release guard script was not run because this package did not
  change external-authority suite posture, cohort fixtures, or required-case
  bindings.
