# worker handoff

Status: executed-hold

Evidence mode: Static + Ran

Date: 2026-06-11

## Handoff

FDHP01 is not complete. No branch was created for this closure pass.

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
- WAT `SoilWaterTotal` is now the hydout-equivalent `Total-Soil` alias again;
  `frozwt` remains separately published to avoid frozen-storage double
  counting.
- `SC-WATBAL-001` v150 pins the legacy `frwatc.for`/`watbalprint.for`
  definition: `Total-Soil`/`SoilWaterTotal` exclude frozen water, and
  frost-active storage audits use `Total-Soil + frozwt`.
- Active frost exchange now publishes `frost.runtime_frwatc_*` diagnostics
  proving liquid/frozen before/after state, freeze debit, thaw credit, and
  signed liquid delta.

Validation status before post-review cohort validation:

- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass.

Residual note:

- Post-review cohort run root:
  `/tmp/fdhp01_closure_after_d1_restored_20260611T053545Z`.
- `42/43` frost-on prefixes exited clean; `p2` failed before WAT publication
  at `HKERNEL-WB11-PERC-E-003`, `1990-308`.
- Emitted-prefix annual closure max abs residual is
  `2.4798612273409617 mm` after D1, down from the pre-D1 post-review
  `75.43917280313423 mm`; closure-under-frost is still not restored to
  numerical noise.
- Emitted-prefix depth metrics overreach legacy heat-flow range
  (`open max depth mean 1782.2670980346527 mm`; median correlation
  `-0.10301692862035305`).
- Focused D2 diagnostics now prove symmetric freeze/thaw exchange algebra at
  the WB14/WB11 seam. Addendum 2c then localized the WAT contradiction to
  publication: emitted `frozwt` is `0.149 * frdp` over measured frost-active
  days, not the exchanged frozen store.
- `SC-SNOWFREEZE-001` v55 reopens `GAP-SNOWFREEZE-002`.

First actionable item: close defect `FDHP01-FROST-DEPTH-HEATFLOW-001` on the
current attempted heat-flow implementation. Sequence the next pass as: fix
`frozwt` publication to source the true exchanged store tracked by the
`frwatc` diagnostics and retire the `0.149 * frdp` scale; rerun the additive
identity on the cohort to confirm whether D2 dissolves; then address D3 depth
runaway against the corrected storage gate. Keep the `p2`
`HKERNEL-WB11-PERC-E-003` fail-closed defect tracked separately. Do not advance
to MOFE until the full 43-prefix cohort runs clean, frost-active annual closure
returns to numerical noise under the ratified storage term, and depth/duration
evidence materially closes the FDMC01 gap without comparator tuning.
