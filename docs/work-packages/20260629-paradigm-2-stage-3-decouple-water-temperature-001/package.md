# PARADIGM-2 Stage 3-Decouple Snow-Neutral Water-Temperature Arm

Status: `EXECUTED-COMPLETE-OPT-IN-CAPABILITY`
Date: `2026-06-29`
Contract: `SC-SNOWFREEZE-001` v111, `INV-SNOWFREEZE-081`,
`OBL-SNOWFREEZE-P-056`
Selector:

- `OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL=layered_thermal_liquid_v1`

## Objective

Execute the Paradigm 2 deliverable as an opt-in water-temperature capability:
keep the current bulk snow-density/frost behavior, carry a private
bulk-equivalent layer geometry for the Stage 3 thermal/liquid solver, and emit
typed meltwater-flux temperature for the future stream-temperature program.
The no-env default, CoE melt mass path, rollback selector, frost behavior, and
public output schemas remain intact.

## Read-First Basis

- `docs/planning/snow-frost-fidelity-strategy.md` §10.3 step 10
- `docs/planning/paradigm2-multilayer-snow-specification.md` §1.1, §4 reqs
  3/4, and §6 Stage 3
- Prior Stage 3 package:
  `docs/work-packages/20260629-paradigm-2-stage-3-liquid-routing-meltwater-temperature-001/`
- Stage 1 layer state in `crates/openwepp-hillslope-orchestrator/src/winter_column.rs`
- Stage 0 surface-energy primitives in `crates/openwepp-meteorology`
- Bulk density model in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs`
- ADR-0029, ADR-0028, ADR-0026, ADR-0025, ADR-0011
- `docs/backlog/20260627-stream-water-temperature-surface-energy-balance.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`

## Scope

- Contract-first v111 amendment for the decoupled opt-in arm.
- Remove the Stage 3 runtime requirement for
  `physics_bulk_multilayer_density_v1`.
- Preserve the current `layered_thermal_liquid_v1` internal selector with
  disabled default and fail-closed unsupported values.
- When Stage 3 is selected without Stage 1 density, synthesize or carry a
  persistent layer stack whose per-layer densities all equal the selected bulk
  aggregate density and whose aggregate SWE/depth/density exactly reconstruct
  the bulk outcome.
- Keep the existing CoE snow/rain melt mass authoritative for routed melt,
  downstream liquid forcing, and WAT/public water-balance surfaces.
- Reuse the Stage 3 thermal/liquid/refreeze solver and typed meltwater-flux
  temperature diagnostic.
- Prove no public schema, fixture, default, frost, cap, parser, runfile, user
  CLI, `.run`, Qwet/frzftp, compatibility-runtime, or full stream-temperature
  routing change.

## Non-Scope

- No default activation.
- No Stage 1 per-layer densification requirement for the water-temperature arm.
- No replacement of CoE melt with energy-balance melt.
- No snow-density promotion claim.
- No public output schema change beyond internal diagnostic state/trace.
- No full in-stream water-temperature routing.
- No fixture/site calibration.

## Gates

1. Contract-first amendment exists and tests bind `INV-SNOWFREEZE-081`.
2. Opt-in selector is fail-closed; absent selector preserves the disabled path.
3. Stage 3 runs with `physics_bulk_density_compaction_v1` and does not require
   `physics_bulk_multilayer_density_v1`.
4. Snow-density cross-SNOTEL/cancov rubric exactly matches the current bulk
   default (`15` robust fails / `179` score) with no worse robust cells.
5. Mass, liquid, and energy ledgers close from produced operands.
6. Meltwater temperature is typed and physically reasonable: ripe melt near
   `0 degC`, no layer or flux temperature above freezing, and cold-content/
   refreeze behavior captured.
7. Runoff/melt-timing forcing-robust signatures are no-worse.
8. ADR-0025 H2637 performance evidence is recorded.
9. Rust/doc gates pass or the package closes `HOLD` with explicit blockers.

## Evidence Artifacts

- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/authority-provenance.md`
- `artifacts/decoupled-conservation-temperature.md`
- `artifacts/paradigm2-stage3-decouple-observed-guardrails.md`
- `artifacts/paradigm2-stage3-decouple-observed-guardrails.json`
- `artifacts/performance-h2637.md`
- `artifacts/review.md`
- `artifacts/verification.md`
- `artifacts/line-count-governance.md`

## Disposition

`EXECUTED-COMPLETE-OPT-IN-CAPABILITY`.

The decoupled opt-in arm is implemented and gated. With only
`OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL=layered_thermal_liquid_v1` set, the
Stage 3 thermal/liquid solver now runs over a bulk-equivalent layer stack and
does not require `physics_bulk_multilayer_density_v1`. The current no-env bulk
default remains unchanged.

Gate disposition:

- Contract-first amendment: passed. `SC-SNOWFREEZE-001` v111 records
  `INV-SNOWFREEZE-081` and `OBL-SNOWFREEZE-P-056`.
- Snow-density observed guardrail: passed. Current default and decoupled arm
  both score `15` robust fails / `179`; decoupled-vs-default robust cells are
  `0` better, `90` equal, `0` worse.
- Runoff/melt-timing guardrail: passed. Decoupled-vs-default timing/runoff
  cells are `0` better, `40` equal, `0` worse.
- Conservation and meltwater temperature: passed by focused Stage 3-Decouple
  and Stage 3 liquid-routing tests.
- H2637 performance: passed at `70.68 s` / `1150612 KiB`, within the ADR-0025
  `<=10x` budget (`91.2 s`).
- Rust, dependency, anti-evasion, and Markdown gates: passed as recorded in
  `artifacts/verification.md`.

No default activation, Stage 1 per-layer densification requirement, CoE
melt-mass replacement, public output schema change beyond the diagnostic
meltwater-temperature flux, full in-stream temperature routing, fixture change,
site calibration, frost behavior change, density-cap change, parser/runfile
change, user CLI change, `.run` control, Qwet/frzftp change, or
compatibility-runtime change was made.
