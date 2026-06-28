# Verification A

Evidence mode: Ran.

## Commands

- `cargo fmt --check` -> PASS.
- `cargo clippy --workspace --all-targets -- -D warnings` -> PASS after
  refactoring the line-count failure.
- `cargo deny check` -> PASS.
- `cargo test --test snowdensity10_3_16_open_surface_ablation_stage_a` -> PASS,
  3 passed.
- `cargo test --workspace` -> PASS after the final line-count refactor.

## Package Gate Check

The package has current direct evidence for each current-scope gate. Because the
under-persistence non-worsening gate failed, the only valid closure state is
non-promotion/HOLD, not activation.
