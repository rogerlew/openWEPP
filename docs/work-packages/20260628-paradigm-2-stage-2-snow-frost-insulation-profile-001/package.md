# PARADIGM-2 Stage 2 Snow-Frost Insulation Profile

Status: `HOLD-GATE-FAILURE-NON-PROMOTION`
Date: `2026-06-28`  
Contract: `SC-SNOWFREEZE-001` v109, `INV-SNOWFREEZE-079`,
`OBL-SNOWFREEZE-P-054`  
Selectors:

- `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=physics_bulk_multilayer_density_v1`
- `OPENWEPP_SNOWFROST_STAGE2_INSULATION_MODEL=layered_resistance_v1`

## Objective

Execute Paradigm 2 Stage 2 as a minimal frost-first opt-in candidate: keep the
Stage 1 snow layer stack density-only, verify the layer-density gradient exists,
then replace the bulk snow-to-frost insulation handoff with a layer-stack
thermal-resistance equivalent. The current no-env default and bulk frost
handoff remain intact.

## Read-First Basis

- `docs/planning/paradigm2-multilayer-snow-specification.md` §1.1, §4 reqs
  3/5/6, §6 Stage 2, §9
- ADR-0029, ADR-0028, ADR-0026, ADR-0025, ADR-0011
- Stage 1 package and review:
  `docs/work-packages/20260628-paradigm-2-stage-1-layered-snow-density-001/`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- Frost handoff and solver:
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs`
  and `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs`

## Scope

- Contract-first amendment for opt-in layered insulation-profile coupling.
- Instrument real direct-production traces with top and basal layer densities.
- Entry gate: prove the Stage 1 candidate develops a basal-denser-than-surface
  density gradient before frost coupling is evaluated.
- Add a package-bound opt-in frost-insulation selector that composes with
  `physics_bulk_multilayer_density_v1`.
- Use the existing snow conductivity relation and frost surface-resistance path
  by computing an equivalent bulk density whose `depth / k(rho)` reproduces the
  layer-stack resistance `sum(layer_depth / k(layer_density))`.
- Preserve the prior-day snow-to-frost timing decoupling.

## Non-Scope

- No default activation.
- No per-layer thermal solve, surface-energy-balance melt, per-layer melt,
  liquid routing, or meltwater temperature.
- No public output schema, fixture, density-cap, parser, runfile, user CLI,
  `.run`, Qwet/frzftp, compatibility-runtime, or frost output change.

## Implementation Summary

Stage 2 is implemented as an internal opt-in snow-to-frost insulation candidate.
The selector
`OPENWEPP_SNOWFROST_STAGE2_INSULATION_MODEL=layered_resistance_v1` composes with
`OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=physics_bulk_multilayer_density_v1`; the
absent/default selector remains `bulk_depth_density`.

The implementation chooses the rollback-compatible effective-density option:
it computes prior-day snow-layer thermal resistance as
`sum(layer_thickness_m / k(layer_density_kg_m3))`, using the Sturm et al. 1997
snow conductivity relation already mirrored by the WEPP frost heat path, then
inverts the same relation to pass an insulation-equivalent density through the
existing `DirectFrostThermalInputs` bulk snow field. This preserves the existing
frost solver interface, prior-day snow-to-frost timing, public WAT schema, and
bulk rollback.

The snow trace now records surface and basal layer density before and after the
snow step. This is diagnostic-only; no public output schema changed.

## Gates

1. Contract-first amendment exists and tests bind `INV-SNOWFREEZE-079`.
2. Stage 1 gradient entry gate passes on a real direct-production run.
3. Opt-in selector is fail-closed; absent selector preserves the bulk handoff.
4. Frost observation-corpus primary gate improves forcing-robust frost-depth
   signatures versus the bulk handoff: onset, deepening, thaw, and frozen
   duration.
5. Cross-SNOTEL snow rubric does not worsen.
6. Snow/frost conservation closes.
7. ADR-0025 H2637 performance evidence is recorded.
8. Rust/doc gates pass or the package closes `HOLD`/non-promotion with explicit
   blockers.

## Gate Disposition

The real gradient entry gate and paired frost observation-corpus run completed.
The opt-in candidate is executable and preserves protected boundaries, but it
fails the primary promotion gate because forcing-robust frost signatures do not
improve relative to the bulk handoff:

- gradient entry gate: pass; `56831` multi-layer trace rows, `49548` positive
  basal-minus-surface density-gradient rows, max gradient
  `446.5207296110246 kg m^-3`;
- paired frost corpus: bulk handoff `3` robust fails / `49` score; layered
  resistance `3` robust fails / `49` score;
- forcing-robust frost cell deltas: `0` improved, `0` worsened;
- limited report-only frost-depth cells are mixed: Sleepers South field
  `frost_depth_timeseries` improves by one ordinal, while Morris
  `frost_max_depth_bias` worsens by one ordinal.

| Gate | Status | Evidence |
|---|---|---|
| Contract-first amendment and selector binding | Pass | `SC-SNOWFREEZE-001` v109; `tests/integration/paradigm2_stage2_snow_frost_insulation_profile.rs`. |
| Stage 1 density-gradient entry gate | Pass | `artifacts/paradigm2-stage2-gradient-entry-gate.json`. |
| Opt-in selector fail-closed; bulk default preserved | Pass | Source guard and integration test; absent selector maps to `bulk_depth_density`. |
| Frost observation-corpus primary gate improves forcing-robust signatures | Fail | `artifacts/paradigm2-stage2-frost-rubric.json`: `3/49` vs `3/49`, no improved robust cells. |
| Cross-SNOTEL snow no-regression guard | Pass-static | Selector is consumed only inside frost thermal-input construction; it does not feed snow partition, snow density, or WAT snow publication. |
| Snow/frost conservation | Pass-static for implementation, not promotion-decisive | Stage 1 layer SWE/depth closure remains enforced; Stage 2 now fails closed if layer SWE/depth do not reconstruct runtime snow state before computing resistance. |
| ADR-0025 H2637 performance evidence | Not promotion-decisive | The primary frost gate failed first; no activation/performance promotion claim is made. Real paired frost corpus elapsed `319.179 s` for two model arms across five sites. |
| Protected boundaries | Pass | No default, rollback, fixture, public schema, frost output, density-cap, melt, phase, canopy, radiation, parser, runfile, user CLI, `.run`, Qwet/frzftp, compatibility-runtime, or site-calibration change. |

## Evidence Artifacts

- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/paradigm2-stage2-gradient-entry-gate.json`
- `artifacts/paradigm2-stage2-gradient-entry-gate.md`
- `artifacts/paradigm2-stage2-frost-rubric.json`
- `artifacts/paradigm2-stage2-frost-rubric.md`
- `artifacts/authority-k-rho-provenance.md`
- `artifacts/frost-rubric-results.md`
- `artifacts/snow-no-regress-results.md`
- `artifacts/performance-h2637.md`
- `artifacts/review.md`
- `artifacts/verification.md`
- `artifacts/line-count-governance.md`

## Disposition

`HOLD-GATE-FAILURE-NON-PROMOTION`.

No activation is authorized. The Stage 2 candidate remains available only through
the package-bound opt-in selector for diagnosis. The current no-env default,
bulk snow-to-frost handoff, and explicit rollback behavior remain unchanged.
