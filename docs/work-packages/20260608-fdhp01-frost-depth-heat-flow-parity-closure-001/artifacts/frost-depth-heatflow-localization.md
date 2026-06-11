# Frost Depth Heat-Flow Localization

Status: complete

Evidence mode: Static + package-authorized subagent report

## Scope

FDHP01 localizes to the single-OFE frost depth/duration publication path. The
authorized write set is the frost runtime coupling, WAT publication surface,
contract/test surfaces, and package evidence. Multi-OFE and MOFE compensation
are out of scope.

## Baseline Defect

Static: `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`
still uses the retired freeze-index proxy:

- prior frost-depth state is rejected above `WB14_FROST_MAX_DEPTH_M = 0.20 m`;
- daily freeze/thaw update is `0.20 * clamp(mean-temperature / 6 C)`;
- `Dfrost`, `Dthaw`, `frdp_m`, `thdp_m`, `tfrdp_m`, and `tthawd_m` are
  hard-bounded to `0.20 m`;
- frozen-soil conductivity fraction also uses the proxy cap as the depth scale.

Static: `crates/openwepp-hillslope-output/src/hillslope_wat.rs` and the runner
WAT assembly path publish `frozwt` but do not publish `frdp`.

Package evidence: the comparator worker found no FDMC01 owcmp suite manifest to
rerun exactly through `tools/owcmp`, but confirmed the FDMC01 characterized gap:
openWEPP proxy depth is hard-capped at `200.0 mm` for all `43/43` prefixes while
the pinned legacy baseline ranges `240.0..503.2 mm` with mean `414.22093 mm`.

## Legacy Provenance

Static: `/workdir/wepp-forest_260430_baseline/src/frostn.for` drives frost
front update from hourly signed heat flow through snow/residue/frozen soil
layers and clamps frost depth to the physical soil profile, not to `0.20 m`.
`frzng.for` converts available freezing energy to depth increments with latent
heat of fusion. `mlttp.for`/`mltbtm.for` handle thaw routing. `frsoil.for`
preserves conductivity coupling through the frozen profile.

FDHP01 will implement the contract-required phase boundary: an hourly
energy-balance frost-depth update with explicit `Qsrf`/`Quf` publication and a
profile-depth bound. Full fine-layer water redistribution remains outside this
package unless required by focused validation.

## Owned Seams

- `compute_active_frost_coupling`: replace freeze-index depth proxy with hourly
  signed heat-flow depth update and profile-depth bounds.
- `hydrology_phase_runoff_reconciliation`: write frost depth runtime fields
  against the dynamic profile-depth bound.
- `HillslopeWatRow` and runner WAT row assembly: publish `frdp` in millimetres.
- Unit registry and WAT schema tests: declare/verify `hillslope_wat.frdp`.
- `SC-SNOWFREEZE-001`: amend `INV-SNOWFREEZE-006`/GAP-002 to describe the
  executable FDHP01 boundary.

## Non-Regression Anchors

- FROSTVAL01 rerun, 2026-06-11: `43/43` frost-active, closure-under-frost at
  `3.2e-11 mm`; frost-off paired runs `43/43` clean.
- FQ4 activation behavior must remain active when default frost controls have
  `wintRed=1`, independent of `frost_file_present` provenance.
