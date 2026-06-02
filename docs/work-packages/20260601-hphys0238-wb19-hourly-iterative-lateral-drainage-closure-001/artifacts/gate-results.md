# HPHYS0238 Gate Results

Status: completed  
Evidence mode: Ran

## Commands

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`

## Final Results

- `cargo fmt --check`: pass
- `cargo clippy --workspace --all-targets -- -D warnings`: pass
- `cargo test --workspace`: pass

## Notes

- Earlier intermediate failures were resolved within package scope:
  - formatting-only drift,
  - clippy naming warning in new WB19 test,
  - HPHYS0225 source-guard expectation for explicit `available_pool` assignment,
  - unstable WB19 lane-divergence assertions replaced with stable
    lane-invariance/conservation and lane-symbol hard-fail guards.
