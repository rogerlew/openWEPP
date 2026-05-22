# Hillslope Consumer Ownership Wiring Contract (SR06)

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Consumer ownership mapping is explicitly encoded in hillslope orchestrator phase routing and required-symbol enforcement.
- Runtime seam symbol families are detected explicitly and required symbols are enforced with typed failure (`HS-CONSUMER-E-001`) when a seeded family is incomplete.

Ran:
- Contract behavior is exercised by SR06 integration and unit tests under `cargo test --workspace`.

## Canonical Phase to Consumer Adapter Mapping

| hillslope phase | consumer adapter | authority surface |
|---|---|---|
| `normalization` | `soil` | soil runtime seam state |
| `storage_bounds` | `soil` | soil runtime seam state |
| `evapotranspiration` | `watbal` | soil-driven water-balance inputs |
| `percolation_deep_seepage` | `perc` | percolation-required soil state |
| `lateral_transfer` | `watbal` | soil-driven water-balance inputs |
| `drainage` | `perc` | percolation-required soil state |
| `runoff_reconciliation` | `runoff` | slope + soil coupling boundary |
| `storage_reconciliation` | `watbal` | soil-driven water-balance inputs |
| `closure_diagnostics` | `watbal` | soil-driven water-balance inputs |

## Required Runtime Symbol Policy

Symbol requirements are activated by runtime family presence sentinels and validated per phase:

- Slope-family sentinels: `nelem`, `nwsofe`, `nslpts`, `slplen`, `avgslp`
- Soil-family sentinels: `nsl`, `solthk`, `dg`, `thetdr`, `thetfc`, `ssc`

Activated required symbol sets:

- `runoff` (slope family): `nslpts`, `slplen`, `avgslp`, `xinput_0001`, `slpinp_0001`
- `runoff` (soil family): `nsl`, `solthk`, `thetdr`, `thetfc`, `ssc`
- `soil`: `nsl`, `solthk`, `dg`, `thetdr`, `thetfc`, `ssc`
- `watbal`: `nsl`, `solthk`, `thetdr`, `thetfc`, `ssc`
- `perc`: `nsl`, `thetdr`, `thetfc`, `ssc`

Failure policy:

- Missing activated required symbol -> typed boundary error `HS-CONSUMER-E-001`
- Scheduler phase status -> `BoundaryClass::MissingRequiredInput`
- No fallback/default substitution is introduced

## Code Anchors

- Adapter enum + kernel request boundary field:
  - `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:335`
  - `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:361`
- Consumer requirement constants + typed boundary error:
  - `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:21`
  - `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:103`
- Phase->adapter mapping and required symbol resolution:
  - `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:143`
  - `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:165`
- Scheduler enforcement + typed missing-input status:
  - `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:731`
