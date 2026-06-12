# Implementation Test Evidence

Status: executed-hold

Evidence mode: Static + Ran

Date: 2026-06-11

## Implementation Summary

Static:

- Replaced the retired freeze-index depth proxy in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`
  with an hourly heat-flow update. Surface heat loss through the
  snow/residue/frozen path is published separately from lower unfrozen-soil
  heat flow (`Quf`); their signed balance advances or thaws `Dfrost` through
  latent-heat increments. Depth state is bounded by the physical soil profile
  depth from `solthk`, not by the former `0.20 m` model cap.
- Preserved the `0.20 m` CLIM06 tilled-layer conductivity scale as a
  conductivity input only. It no longer bounds model frost depth.
- Added a fail-closed guard so newly frozen-water storage cannot exceed
  available liquid `wb11_soil_water`.
- Added thaw-side liquid credit so reductions in `frost.runtime_ws_frz`
  restore prior frozen storage to `wb11_soil_water`.
- Added profile-depth propagation through `FrostCouplingOutcome` so downstream
  frost writeback validates against the same physical bound.
- Published runtime `frost.runtime_frdp_m` to the WAT surface as `frdp` in
  millimetres, with schema metadata, unit-registry aliases, profile-bound
  validation, and WAT interchange dataset version `1.4`.
- Bound WAT `frozwt` publication to
  `frost.runtime_frwatc_frozen_water_after_m`, added initial projection of
  the `frost.runtime_frwatc_*` diagnostics, and added a guard so missing
  exchange-store publication fails closed.
- Replaced the scalar `frdp * theta_active` frozen-water store with per-layer
  `wb18_perc_frozen_depth_####` and `wb18_perc_frzw_####` state. WAT `frozwt`
  now resolves the legacy `Σ soilf(i)` store from active layer state:
  `Σ(wb18_perc_frzw_#### + thetdr_#### * wb18_perc_frozen_depth_####)`.
- Added D3 Increment A shadow fine-state diagnostics for the legacy
  `fgfrst`/`slfsd`/`slsic`/`slsw`/`sltime`/`yst`/`nwfrzz` handoff seam. The
  shadow state is written only as diagnostics and does not drive active depth,
  conductivity, publication, or freeze/thaw behavior.
- Made WAT parquet physical bytes deterministic by writing a sorted
  `ARROW:schema` footer. Required Arrow field metadata is still preserved for
  file readers.
- Removed stale runner/orchestrator constants that represented the retired
  model-depth cap.

## Focused Post-Implementation Tests

Ran:

- `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture`
  - Result: passed, 22 tests, including layered-store rejection of scalar
    frozen-water equivalence, per-layer freeze update coverage, and Increment A
    shadow-state round-trip/identity/non-driving tests.
- `cargo test --test cli04_runner_wat_parquet_contract_derived_tests -- --nocapture`
  - Result: passed, 2 tests.
- `cargo test --test sim_contract_boundary_unit_registry -- --nocapture`
  - Result: passed, 14 tests.
- `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract -- --nocapture`
  - Result: passed, 8 tests.
- `cargo test -p openwepp-runner --lib fdhp01_wb13 -- --nocapture`
  - Result: passed, 3 tests, including the missing exchange-store guard.
- `cargo test -p openwepp-runner --lib
  hphys0203_wb13_soil_water_total_preserves_watcon_alias -- --nocapture`
  - Result: passed, proving WAT `frozwt` follows
    `frost.runtime_frwatc_frozen_water_after_m` when it differs from
    `runtime_ws_frz`.
- `cargo test -p openwepp-hillslope-output schema_includes_required_dataset_metadata_keys -- --nocapture`
  - Result: passed, 1 test.
- `cargo test -p openwepp-hillslope-output hillslope_wat -- --nocapture`
  - Result: passed, 4 tests, including deterministic WAT bytes and file-level
    field metadata preservation.
- `cargo clippy -p openwepp-hillslope-output --all-targets -- -D warnings`
  - Result: passed.

## Workspace Gates

Ran:

- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed.

## Notes

The landed tests prove narrow contract-critical implementation boundaries:
retired cap removed, separate `Qsrf`/`Quf` publication, warm heat-flow thaw
accepted for deep prior frost, frozen-water overdraw fail-closed, WAT `frdp`
value publication/profile bound/versioning, and unit registry authority
present. The layered continuation adds tests proving the published frozen store
is no longer the scalar `frdp * theta` quantity and that layer `frzw` changes
with freeze/thaw.

Post-review cohort validation supersedes the earlier "unavailable FDMC01
manifest" caveat: direct measurement was available and was run. The layered
cohort gate at `/tmp/fdhp01_layered_store_20260611T080722Z` produced `43/43`
clean exits, closed the annual `Total-Soil + frozwt` identity to numerical
noise (`1.2683574368566042e-07 mm` max abs residual), and broke the exact
`frozwt/frdp` scalar relation. This implementation remains held only on the D3
depth/duration parity gate: max-depth mean `1782.0379909380451 mm` versus
legacy `414.22093023255815 mm`, median depth correlation
`-0.27756218032931956`.

Dd diagnostic continuation: a temporary env-gated hook forced only the frost
snow-depth/density inputs from legacy `H*.winter.dat`, ran `43/43` clean, and
was removed before rebuilding production. Years 2-6 additive storage closure
remained at WAT-publication texture (`6.726058817130287e-07 mm` max abs), but
legacy snow forcing did not certify D3 closure: mean max depth stayed
`856.817674502367 mm`, `0/43` prefixes were inside the `240..503.2 mm`
envelope, and median frozen duration residual stayed `+502` days. The
production rebuild after hook removal restored binary SHA
`95491b24f36065c28f90ca7e55bfceb39cf14ac2c270ddfd207eb750a2e4a536`.

De implementation continuation: Codex landed the content-dependent lower-front
`Qdry` conductivity correction from legacy `frostn.for:430-458`, including the
parser-derived `wb19_bulk_density_kg_m3_####` seam and the same conductivity
path for bottom thaw. The full Rust closure loop passed, plus authority
anti-evasion guards. The final native production cohort at
`/tmp/fdhp01_increment_de_native_cohort_final_20260612T171358Z` ran `43/43`
clean with years 2-6 additive storage closure at
`5.474257917248426e-07 mm`. A temporary forced-snow hook was then used only for
diagnosis and removed before the final production rebuild. The corrected
forced-snow cohort at
`/tmp/fdhp01_increment_de_forced_snow_cohort_20260612T171017Z_proper` ran
`43/43` clean with years 2-6 closure at `4.355148297552347e-07 mm`; forced
legacy snow plus De improved mean max depth to `655.9890274782282 mm` and
median duration residual to `+186` days, but did not certify D3 because `0/43`
prefixes entered the `240..503.2 mm` envelope. Production binary SHA after hook
removal is `981da203d9ced9b1d73f049fa3a4b227710862a3dbecaad9d4619f03ae7dd2d5`.

Df diagnostic continuation: Codex added a temporary env-gated p1/p2 forced-snow
hourly trace, joined it to legacy `H*.winter.dat`, and removed the hook before
the clean-source production rebuild. The clean rebuild returned the De
production binary SHA
`981da203d9ced9b1d73f049fa3a4b227710862a3dbecaad9d4619f03ae7dd2d5`, and a
source marker search found no `OPENWEPP_FDHP01_DF`/`fdhp01_df` traces under
`crates/`. Compact evidence is in
`fdhp01_increment_df_localization_summary_20260612.json`,
`fdhp01_increment_df_term_attribution_20260612.csv`, and
`fdhp01_increment_df_paired_hourly_excerpt_20260612.csv`. Df did not land
production physics; it localizes the next implementation to residue-depth frost
resistance plus the legacy shallow-front `dpfsfl` minimum conduction distance.

Dj implementation continuation: Codex landed the legacy `hr_tmp`/`tmpadj`
surface-temperature synthesis into active frost top heat flow. The runtime now
publishes `frost.hourly.surface_temp_c_####`, fails closed on missing required
hourly radiation/cloud/air forcing, preserves the positive-under-snow cap, and
projects winter hourly forcing when frost is enabled by `frost.options.wintRed`
or runtime frost state even on warm/no-snow days. Focused Dj tests passed for
non-raw-air surface-temperature synthesis, the positive snow cap, and missing
radiation fail-closed behavior. Native and forced-snow cohorts both ran
`43/43` clean with years 2-6 additive closure at WAT-publication texture, but
the forced-snow maximum-depth gate did not improve, so FDHP01 remains
`executed-hold`.
