# worker handoff

Status: executed-hold

Evidence mode: Static + Ran

Date: 2026-06-11

## Handoff

FDHP01 is not complete. No branch was created and no commit was made.

Primary landed behavior:

- Frost depth now uses hourly signed heat flow and latent-heat increments
  instead of the retired freeze-index proxy.
- Depth state is bounded by `solthk`/physical profile depth, while the remaining
  `0.20 m` constant is only the CLIM06 tilled-layer conductivity scale.
- Frozen-water exchange is bidirectional: freezing withdraws liquid water and
  fails closed on overdraw; thawing returns prior frozen storage to liquid
  `wb11_soil_water`.
- WAT output now publishes `frdp` in `mm` from runtime
  `frost.runtime_frdp_m`; dataset version is `1.4`.

Validation status before post-review cohort validation:

- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass.

Residual note:

- Post-review cohort run root:
  `/tmp/fdhp01_closure_20260611T041333Z`.
- `42/43` frost-on prefixes exited clean; `p2` failed before WAT publication
  at `HKERNEL-WB11-PERC-E-003`, `1990-308`.
- Emitted-prefix annual closure max abs residual is
  `75.43917280313423 mm`; closure-under-frost is broken.
- Emitted-prefix depth metrics overreach legacy heat-flow range
  (`open max depth mean 1782.2670980346527 mm`; median correlation
  `-0.10301692862035305`).
- `SC-SNOWFREEZE-001` v55 reopens `GAP-SNOWFREEZE-002`.

First actionable item: close defect `FDHP01-FROST-DEPTH-HEATFLOW-001` on the
current attempted heat-flow implementation. Do not advance to MOFE until the
full 43-prefix cohort runs clean, annual closure returns to numerical noise,
and depth/duration evidence materially closes the FDMC01 gap without comparator
tuning.
