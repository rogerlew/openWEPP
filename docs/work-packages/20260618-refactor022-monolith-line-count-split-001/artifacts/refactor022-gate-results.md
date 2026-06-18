# REFACTOR022 Gate Results

Evidence class: Ran.

## Required Gates

| Gate | Result |
|---|---|
| `cargo fmt --check` | pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass |
| `cargo test --workspace` | pass |
| `cargo deny check` | pass |
| `git diff --check` | pass |
| Section move parity check | pass, `REFACTOR022_SECTION_MOVE_PARITY_OK` |
| True HEAD bit-identity anchor | pass, `anchor_mismatches = 0` |

## Focused Checks Run During Implementation

These focused checks passed before the full closure gates:

- `cargo check -p openwepp-kernel-contract`
- `cargo check -p openwepp-watershed-orchestrator`
- `cargo check -p openwepp-hillslope-orchestrator`
- `cargo check -p openwepp-runner --bin openwepp-cli-hill`
- `cargo test -p openwepp-kernel-contract`
- `cargo test -p openwepp-watershed-orchestrator`
- `cargo test -p openwepp-hillslope-orchestrator`
- `cargo test -p openwepp-runner --bin openwepp-cli-hill`
- `cargo test -p openwepp-runner`

## Notes

Two clippy boundary lints appeared after the mechanical split because original local lint
allowances were left at the old section boundary. The original attributes were restored on the
moved functions:

- `#[allow(clippy::similar_names)]` on `derive_ws15_channel_sediment_scaffold`.
- `#[allow(clippy::too_many_lines)]` on `execute_scheduler_kernel_lifecycle`.

No production logic changed for those fixes; final section parity includes both original
attributes.
