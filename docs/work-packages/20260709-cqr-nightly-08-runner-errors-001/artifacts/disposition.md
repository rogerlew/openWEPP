# Disposition

Evidence label: Static/Ran.

Status: `EXECUTED-COMPLETE-CQR-NIGHTLY`

## Objective Disposition

Static/Ran:

- Target module:
  `crates/openwepp-runner/src/errors.rs`.
- Production code changed: no.
- Characterization changed:
  `tests/integration/cli01_runner_contract_derived_tests.rs`.
- Baseline target rows above CRAP `30`: `6`.
- After target rows above CRAP `30`: `0`.
- Max target CRAP after characterization: `20.0`.

Disposition:

- Objective is met for the target module.
- No ADR-0021 hold is required for target production functions.
- Full workspace coverage/CRAP could not be produced because unrelated
  `laned_shadow_h2637` coverage-instrumented tests failed before LCOV emission.
  Package Phase D permits the documented targeted equivalent used here.

## Review Finding Disposition

Static/Ran:

| Source | Finding | Disposition | Resolution |
|---|---|---|---|
| Review A | `SidecarInvalid` characterization did not require nested `RELMD-E-*` code in display text. | accepted | Fixed by asserting `RELMD-E-004` and `sha256` fragments. |
| Review B | Final gates were pending. | accepted | Resolved by delegated runner evidence and documented targeted coverage/CRAP substitution. |
| Review B | Metric provenance was ephemeral. | accepted | Resolved by recording size and SHA-256 for targeted metric files. |

## Gate Disposition

Ran:

- Focused runner characterization tests passed: `13/13`.
- `cargo fmt --check` passed.
- `git diff --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo nextest run --workspace --profile full` passed:
  `1573` passed, `3` skipped.
- `cargo deny check` passed.

Final disposition:

- Dual verification passed.
- The closeout commit containing this artifact satisfies the package completion
  boundary before CQR Nightly target #9 starts.
