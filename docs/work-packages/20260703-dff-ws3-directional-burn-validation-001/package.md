# DFF WS-3 Directional Burn Validation + Peakflow Magnitude Adjudication

Status: `executed-hold`

Date opened: `2026-07-03`

Date held: `2026-07-03`

Package type: disturbed-forest validation fixture, direct-runtime verification,
and sediment-production hold package.

## Objective

Build the openWEPP WS-3 disturbed-burn validation substrate and run the first
direct-runtime checks for the McKenzie Bridge burn matrix:

- install a full 80-cell matrix fixture: 4 textures x 5 vegetation classes x 4
  burn severities;
- verify representative burn-direction laws for runoff and peakflow on the
  McKenzie Bridge clay-loam forest pair;
- adjudicate that openWEPP does not reproduce the legacy river-scale peakflow
  artifact on the representative high-burn cell;
- hold sediment ordering until real contract-backed Wave-1/Wave-2 sediment
  production exists in the production direct runtime.

## Disposition

Result: `EXECUTED-HOLD-DFF-WS3-SEDIMENT-PRODUCTION`

WS-3 is held before sediment ordering. The package proved the fixture catalog
and representative runoff/peak direction, but current production direct
execution publishes zero `tdet`, `tdep`, and `sedcon_*` for the representative
cells because the typed direct production seed authority still disables Wave-1
and supplies `DirectErod13Inputs::zero()`.

This package does not implement a runtime selector workaround. Proper Wave-1
and Wave-2 sediment production is queued in
`../20260703-dff-ws3a-wave1-wave2-sediment-production-001/package.md`.

## In Scope

- Fixture and test substrate under `tests/fixtures/disturbed_burn/` and
  `tests/integration/`.
- Package evidence and catalog updates under `docs/work-packages/`.
- Representative p1 unburned and p4 high-burn direct-runtime execution.
- Runoff, peakflow, and current sediment-publication characterization.

## Out of Scope

- Production runtime erosion/sediment implementation.
- Runfile-level activation switches that enable Wave-2 without real Wave-1
  production operands.
- Surrogate, proxy, or fixture-only sediment values.
- Full 80-cell sediment ordering verdicts before the Wave-1/Wave-2 hold is
  lifted.

## Write Set

- `Cargo.toml`
- `tests/integration/dff_ws3_directional_burn_validation.rs`
- `tests/fixtures/disturbed_burn/forest_high_severity_clay_loam/`
- `tests/fixtures/disturbed_burn/mckenzie_bridge_80_cell_matrix/`
- `docs/work-packages/20260703-dff-ws3-directional-burn-validation-001/`
- `docs/work-packages/20260703-dff-ws3a-wave1-wave2-sediment-production-001/`
- `docs/work-packages/README.md`

No production Rust runtime, parser, kernel, or science-contract files are in
this package's final write set.

## Exit Criteria

| Criterion | Status | Evidence |
| --- | --- | --- |
| Full matrix fixture catalog covers 80 cells | `PASS` | `tests/fixtures/disturbed_burn/mckenzie_bridge_80_cell_matrix/matrix.csv`; catalog test |
| Fixture checksums verify | `PASS` | `sha256sum -c SHA256SUMS` in the matrix fixture |
| Representative p1/p4 cells execute through direct production | `PASS` | `dff_ws3_representative_clay_loam_documents_runoff_peak_and_sediment_hold` |
| High-burn runoff and peak exceed matched unburned | `PASS` | Representative integration test and gate artifact |
| Legacy peakflow artifact is not reproduced | `PASS` | p4 max `peakro` remains near `8.24e-6 m3/s`, not legacy `380150 m3/s` |
| Sediment ordering can be asserted | `BLOCKED` | `HOLD-DFF-WS3-SEDIMENT-PRODUCTION`; Wave-1 disabled in production seed authority |
| Runtime/kernel code changes | `NOT APPLICABLE` | Package held before production implementation |

## Hold Boundary

The hold is a production-consumer boundary, not a missing test. The sediment
ordering law needs real downstream output from the direct runtime. Current
production direct execution does not provide that evidence because Wave-1 is
disabled in the typed erosion authority and its input surface is zeroed.

The next package must close the production sediment path with contract-backed
Wave-1/Wave-2 operands, then return to WS-3 to assert sediment ordering across
the matrix.

## Required Reading

- `HANDOFF.md`
- `docs/planning/disturbed-forest-fidelity-strategy.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/work-packages/20260703-dff-ws3a-wave1-wave2-sediment-production-001/package.md`
- `docs/work-packages/20260702-wshedw7dc01-hillslope-sediment-production-hold-lift-001/package.md`

## Subagent Authorization

No delegated subagent work was used for this held package. Any follow-on
implementation package must authorize review and verification subagents in its
own package text if delegation is required.
