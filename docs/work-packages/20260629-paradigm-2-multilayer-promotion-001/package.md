# PARADIGM-2 Multilayer Promotion

Status: `EXECUTED-COMPLETE-PRODUCTION-OPT-IN`
Date: `2026-06-29`
Contract: `SC-SNOWFREEZE-001` v112, `INV-SNOWFREEZE-082`,
`OBL-SNOWFREEZE-P-057`

## Objective

Ratify `layered_thermal_liquid_v1`, after the Stage 3-Decouple result, as a
production-supported opt-in water-temperature capability. The bulk snow model
remains the no-env default and rollback. The production value is the supported
hillslope meltwater-temperature source for later stream-temperature work, not a
new density, frost, or melt-mass default.

## Decisions

1. Activation posture: `OPT-IN`. Default-on is not justified because the arm is
   snow-neutral, adds layer-thermal overhead, and has no current default
   downstream consumer. The bulk default remains selected when the Stage 3
   selector is absent or empty.
2. Selector exposure: Production-supported internal selector. The existing
   `OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL=layered_thermal_liquid_v1` selector
   is promoted from package-bound diagnostic to supported production control.
   User-facing runfile/WEPPpy wiring is deferred to the stream-temperature
   program, where the first public snow water-temperature control can be designed
   with a downstream consumer.

## Scope

- Contract-first `SC-SNOWFREEZE-001` v112 amendment for production-supported
  opt-in status, fail-closed selector behavior, supported output field, default
  rollback, and protected boundaries.
- Publish `MeltwaterTemperature` as a nullable supported hillslope WAT parquet
  field in `degC`, populated from direct-production Stage 3 diagnostics only
  when routed meltwater temperature exists.
- HBP/watershed serialization remains deferred; keep HBP binary/watershed
  serialization and full in-stream routing out of scope because the
  stream-temperature program owns that consumer boundary.
- Reconfirm the real cross-SNOTEL/cancov snow guardrail with the promoted
  selector.
- Reconfirm focused conservation, temperature sanity, rollback/default behavior,
  and H2637 performance.
- Update work-package index, strategy note, and release-note artifact.

## Non-Scope

- No default-on activation.
- No density/frost behavior change.
- No Stage 1 per-layer densification requirement.
- No CoE melt-mass replacement.
- No HBP binary/watershed serialization.
- No full in-stream temperature routing.
- No fixture or site calibration.
- No user-facing runfile/WEPPpy selector wiring.

## Gates

1. Contract-first amendment exists and tests bind `INV-SNOWFREEZE-082`.
2. Selector fail-closed behavior and absent-selector rollback are verified.
3. `MeltwaterTemperature` is emitted as a supported nullable WAT parquet field
   with unit metadata and direct-production source lineage.
4. Snow-neutral real-run guardrail: promoted arm equals current bulk default
   (`15` robust fails / `179`) with `0` worse robust cells.
5. Conservation closes: mass, liquid, and energy ledgers remain within Stage 3
   tolerances.
6. Meltwater temperature is physically reasonable and supported-output evidence
   shows finite `<= 0 degC` values only when routed meltwater is present.
7. ADR-0025 H2637 performance remains within budget.
8. Production readiness: docs/release note present, no default-path public
   schema regression, no HBP/watershed serialization, and rollback preserved.
9. Rust/doc/dependency gates pass or the package closes `HOLD`.

## Evidence Artifacts

- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/operand-lineage.md`
- `artifacts/observed-guardrails.md`
- `artifacts/observed-guardrails.json`
- `artifacts/supported-output-evidence.md`
- `artifacts/performance-h2637.md`
- `artifacts/release-notes.md`
- `artifacts/review.md`
- `artifacts/verification.md`
- `artifacts/line-count-governance.md`

## Disposition

`EXECUTED-COMPLETE-PRODUCTION-OPT-IN`.

`layered_thermal_liquid_v1` is ratified as a production-supported internal
opt-in water-temperature capability. The no-env bulk snow default remains
unchanged. The supported output is nullable hillslope WAT parquet
`MeltwaterTemperature` in `degC`; default/rollback runs publish null values.
HBP/watershed serialization, full in-stream routing, and runfile/WEPPpy user
exposure remain deferred to the stream-temperature program.

Gates passed: real cross-SNOTEL/cancov snow guardrail `15` / `179` with `0`
worse robust cells; runoff/timing `0` worse robust cells; H2637 `70.65 s` /
`1153680 KiB`; supported output field present on real WAT files with finite
non-positive opt-in values; conservation guards closed on real runs.
