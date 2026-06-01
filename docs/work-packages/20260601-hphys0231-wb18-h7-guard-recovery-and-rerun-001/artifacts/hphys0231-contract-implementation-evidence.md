# HPHYS0231 Contract Implementation Evidence

Status: completed  
Evidence mode: mixed (`Ran` + `Static`)

## Contract Updates (Ran)

Updated canonical authority:
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`

Applied amendments:
1. `contract_version` advanced to `16`; `last_reviewed` retained at
   `2026-06-01`.
2. Replaced strict `0 < FC/UL < 1` hard-fail posture with branch-conditioned
   authority matching baseline semantics:
   - active branch (`stz < 0.95`) and `FC/UL <= 0` uses explicit
     legacy-degenerate `Bi = 0`,
   - active branch (`stz < 0.95`) and `FC/UL > 0` uses
     `Bi = -2.655/log10(FC/UL)` with typed domain guards,
   - saturated branch (`stz >= 0.95`) bypasses ratio-domain evaluation with
     `fx = 1`.
3. Updated test-vector obligations to require coverage of:
   - positive-ratio dynamic-`Bi` path,
   - non-positive-ratio `Bi=0` path,
   - saturated-branch bypass behavior.

## Production Implementation Mapping (Ran)

Updated runtime implementation:
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`

Mapping:
1. WB18 percolation now evaluates `FC/UL` ratio only inside the active
   conductivity branch (`stz < 0.95`).
2. Non-positive ratio handling is explicit and authoritative (`Bi = 0`) rather
   than typed failure.
3. Saturated branch (`stz >= 0.95`) now bypasses ratio-domain failure path.
4. Runner diagnostics capture per-layer WB18 guard terms plus lineage symbols
   (`thetfc/thetdr/dg/por/cpm`) for concrete failure triage.

## Measure Mapping

- `MEASURE-HP231-002`: satisfied.
- `MEASURE-HP231-004`: satisfied.
