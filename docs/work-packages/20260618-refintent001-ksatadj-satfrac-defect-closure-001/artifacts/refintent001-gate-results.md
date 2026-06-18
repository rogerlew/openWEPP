# REFINTENT001 Gate Results

Evidence class: Ran

## Rust gates

| Gate | Result | Notes |
|---|---:|---|
| `cargo fmt --check` | PASS | rerun after clippy cleanup |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | initial fail on `struct_field_names` / `too_many_lines`; remediated |
| `cargo test --workspace` | PASS | full workspace and doctests passed |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok` |
| `cargo build -p openwepp-runner --bin openwepp-cli-hill --release` | PASS | release binary used for H2637 and ladder reruns |
| `git diff --check` | PASS | clean after code and artifact edits |

## Focused gates

| Gate | Result |
|---|---:|
| `cargo test -p openwepp-hillslope-orchestrator wb14_ksatadj -- --nocapture` | PASS, 2 tests |
| `cargo test --test wb14_infiltration_hyetograph_kernel_contract -- --nocapture` | PASS, 16 tests |

## Fixture gates

| Fixture | Result |
|---|---:|
| H2637 without UI | PASS, exit 0 |
| H2637 with UI | PASS, exit 0 |
| OFE1-OFE5 ladder | PASS, all 5 exit 0 |

## Initial non-pass events

- First clippy run failed on style/size lints introduced during the first cut.
  The accumulator fields were renamed and validation was split; clippy then
  passed.
- First OFE-ladder command used relative run-file paths and failed before kernel
  execution. The rerun used absolute paths and all five cases exited 0.

## Documentation checks

- `wctl doc-lint --path docs/work-packages/20260618-refintent001-ksatadj-satfrac-defect-closure-001`
  exited 0, reporting `0 files validated, 0 errors, 0 warnings`.
- `uk2us` preview reported no differences for the new package artifacts. It did
  report broad differences for existing `docs/work-packages/README.md` and
  `docs/ROADMAP.md`; no repo-wide spelling rewrite was applied under this
  scoped package.

No known gate remains open.
