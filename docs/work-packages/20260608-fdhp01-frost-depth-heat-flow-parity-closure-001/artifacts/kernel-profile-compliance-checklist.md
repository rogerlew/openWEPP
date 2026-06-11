# Kernel Profile Compliance Checklist

Status: executed-hold

Evidence mode: Static + Ran

Date: 2026-06-11

## Static Checklist

- Contract-first order followed: `SC-SNOWFREEZE-001` was amended before the
  production frost-depth replacement.
- Contract-derived red tests were added and recorded in
  `pre-implementation-contract-gate.md` before production edits.
- Production frost-depth state now fails closed through existing typed
  orchestrator errors for invalid/non-finite runtime values and profile-bound
  violations.
- Frozen-water exchange now fails closed when newly frozen storage would exceed
  available liquid `wb11_soil_water`; no clamp-to-zero liquid storage is used.
  Warm thaw returns prior frozen storage to liquid `wb11_soil_water`.
- No `.unwrap()` or `.expect()` was added to production paths.
- No `unsafe` code was added.
- No fallback wrapper silently masks a missing required dependency. Runtime
  `frost.runtime_frdp_m` is now required for WAT publication.
- The retired `0.20 m` cap is removed from model-depth publication and
  writeback bounds. The remaining `0.20 m` usage is the CLIM06 tilled-layer
  conductivity scale documented in `SC-SNOWFREEZE-001`.
- `frdp` publication is covered by WAT schema metadata and unit registry alias
  authority, uses dataset version `1.4`, and is bounded by physical profile
  depth at publication.

## Ran

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

All listed Rust gates passed on 2026-06-11. The later post-review
`algebraic-radium` cohort gate failed (`p2` no WAT, annual closure residuals up
to `2.4798612273409617 mm` after the D1 `SoilWaterTotal` fix), so
kernel-profile closure is held pending FDHP01 correction.
