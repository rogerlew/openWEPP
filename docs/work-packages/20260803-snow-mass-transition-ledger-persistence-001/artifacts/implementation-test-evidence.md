# Implementation And Test Evidence

Status: `implementation complete / remediated focused gates pass`

Evidence mode: `Static + Ran`

Intentional architecture deltas:

- introduced `DirectSnowDiagnosticCapture`, two compact ledger types,
  `DirectSnowStage3Outcome`, and boxed `DirectSnowVerboseDiagnostics` in the
  bounded `snow_mass_transition.rs` module;
- replaced duplicated transition scalars throughout direct runtime with the
  same two ledger values;
- moved meltwater temperature and Stage-3 sublimation from the research
  diagnostic into the compact production outcome;
- retained `compute_direct_snow_liquid_partition_from_typed` as a verbose
  compatibility entry point and added the runner-facing capture-aware entry;
- made the schema-v4 writer consume the optional payload explicitly and fail
  closed when a selected row lacks it.
- preserved exact typed ledger validation sources through the hydrology and
  direct-runtime seams;
- retained the original by-value downstream constructor without a deep clone;
- boxed the constructor-input bundle and already-optional live-frame shadow
  record to retain the pre-existing layout ceilings.

Mechanical migrations changed repository-owned field call sites and v123
contract pins to v124. The crate is `publish = false`; no CLI, runfile, or
public output schema changed.

Ran on the remediated source:

- orchestrator/runner all-target check and warnings-denied Clippy: PASS.
- focused seven-binary science/contract set: `37/37` PASS.
- `cargo nextest run -p openwepp-runner`: `228/228` PASS.
- ledger/capture contract: `8/8` PASS, including signed raw melt, signed
  retained-liquid delta, Stage-3-enabled Disabled-vs-Verbose parity, and typed
  failure categories.
- focused typed-error tests: `2/2` PASS; capture selector tests: `2/2` PASS.
- type-layout guard: PASS; `DirectDayConstructorInputs=4112 B` and
  `DirectDayFrame=15552 B`, each 48 B below its retained ceiling.
- footprint test: partition `656 B`, compact ledger/outcome total `112 B`,
  optional payload handle `8 B`.

The trace contains `3844` rows where raw signed melt differs materially from
authoritative SWE loss, so the primary rejected upstream alias is exercised.
The Stage-3 trace retains the previously adjudicated non-alias rows and exact
downstream operand closure. Exact release comparison receipts are recorded in
`performance-storage-evidence.md` and `real-consumer-reconstruction.md`.
