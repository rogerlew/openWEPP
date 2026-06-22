# R7D3 Direct WB14/R4K Infiltration Producer

Status: executed-held.

Package type: Array-native runtime defect-closure implementation package.

Objective: close `R7D-DIRECT-PRODUCTION-PUBLICATION-PARITY` by resolving
`HOLD-R7D2-DIRECT-WB14-R4K-INFILTRATION-PRODUCER-AUTHORITY-ABSENT`.

Rationale: R7D2 lifted multi-OFE lane seed/profile aliasing, but H2637 still
fails HBP/PASS/WAT parity because direct R4K forwards zero
infiltration/depression handoff inputs and direct R4A consequently computes
runoff as raw liquid input. `SC-RUNOFFPART-001`, `SC-WATBAL-001`, and
`SC-PERC-001` require a baseline-authoritative WB14/WB12 infiltration,
depression-storage, and same-pass layer-ingress producer, not compatibility
runtime-surface observations wrapped as direct operands.

Included scope:

- Implement a typed direct WB14/R4K producer for hyetograph infiltration,
  depression-storage handoff, and reconciled runoff inputs.
- Feed the produced same-pass infiltration into R4A runoff partition, WB18
  percolation/storage ingress, ET same-pass infiltration consumers, and direct
  publication projection.
- Preserve existing direct runtime phase-span identity: inputs, direct compute,
  state mutation, downstream operands, and shadow projection.
- Add contract-derived focused tests proving nonzero infiltration can make
  runoff less than liquid input, and malformed required inputs fail closed.
- Add or update production direct H2637 evidence and keep iterating through
  in-envelope parity blockers until HBP/WAT/PASS/loss/manifest parity closes
  or the next named out-of-envelope process blocker is proven.
- Keep default compatibility behavior unchanged and keep production direct
  explicit opt-in.

Excluded scope:

- Default activation, rollback-policy selection, or compatibility deletion.
- Using compatibility `execution.wb13_rows`, compatibility public-output
  builders, or compatibility scheduler/runtime-surface `wb12_infiltration` as
  direct production authority.
- Provisional runoff/infiltration formulas not anchored in
  `SC-RUNOFFPART-001`, `SC-WATBAL-001`, `SC-PERC-001`, or pinned baseline
  provenance.
- Sediment-coupled erosion `qin` producer closure unless it is the next
  blocker after hydrology parity is otherwise closed.

Intended write set:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260622-r7d3-direct-wb14-r4k-infiltration-producer-001/**`

Dependencies:

- R7D2 lane-indexed seed/profile authority.
- `SC-RUNOFFPART-001` WB14 infiltration and subdaily hyetograph authority.
- `SC-WATBAL-001` WB12/WB14 runoff/storage closure authority.
- `SC-PERC-001` WB18 same-pass infiltration ingress authority.

Correction authority envelope:

- Defect: `R7D-DIRECT-PRODUCTION-PUBLICATION-PARITY`.
- Observed failure: H2637 direct production emits direct-source rows with
  `compatibility_edge_invocations=0` but differs from compatibility for HBP,
  WAT, and PASS. After R7D2, direct day-1 `Q` remains raw liquid input across
  multi-OFE lanes because direct R4K has no infiltration/depression producer.
- In-scope corrections: direct WB14/R4K input structures, hyetograph and
  conductivity-domain validation, same-pass infiltration/depression production,
  downstream operand wiring into R4A/WB18/ET/publication, tests, and parity
  artifact updates.
- Protected boundaries: do not use compatibility WB13 rows, compatibility
  scheduler results, aggregate runtime surfaces, stale logical state, or
  compatibility `wb12_infiltration`/`wb12_depression_storage_delta` as direct
  production authority.

Phase plan:

1. Scaffold package, catalog entry, prompt, and evidence artifacts.
2. Inventory required WB14/R4K inputs already available in lane seed surfaces
   and identify missing typed direct inputs.
3. Add focused direct-runtime tests for nonzero infiltration, runoff below
   liquid input, malformed hyetograph failure, and direct counter shape.
4. Implement direct WB14/R4K producer inputs, compute, state mutation,
   downstream operands, and shadow projection.
5. Wire same-pass infiltration into R4A runoff, WB18 percolation ingress, ET
   same-pass infiltration, and direct publication projection.
6. Run focused tests and H2637 parity. If a new in-envelope blocker appears,
   implement the next correction and rerun. Repeat until R7D closes or a named
   out-of-envelope blocker is proven.
7. Complete reviews, verification, line-count disposition, and final
   complete-or-hold disposition.

Anti-premature-stop rule:

- Do not stop after adding only input structs, tests, a diagnostic trace, or a
  single producer output.
- Do not stop after the first H2637 improvement if HBP/WAT/PASS parity still
  fails and the next failure is in this package's correction envelope.
- Do not close as complete while any current-scope R7D gate is `FAIL`,
  `BLOCKED`, or unjustified `NOT RUN`.
- A hold is allowed only when the next blocker is outside this package's
  process family, lacks canonical authority, requires a separate contract
  amendment not authorized here, or is an invalid upstream input correctly
  fail-closed. The hold must name exact residual fields and the first code
  action for the follow-up.

Acceptance gates:

- Direct R4K has typed inputs, direct compute, state mutation, downstream
  operands, shadow projection, and nonzero direct counters.
- Direct R4K no longer forwards only zero constructor defaults for valid
  precipitation/hyetograph inputs.
- Focused test proves positive direct infiltration can make direct `Q` less
  than liquid input without compatibility surfaces.
- Focused tests prove missing/non-finite/malformed hyetograph or conductivity
  domains fail closed with typed direct errors.
- Same-pass infiltration is consumed by R4A, WB18 percolation ingress, ET
  same-pass infiltration, and direct publication projection.
- Static scans prove production direct does not read `execution.wb13_rows`,
  compatibility public-output builders, or compatibility
  `wb12_infiltration`/`wb12_depression_storage_delta` as direct authority.
- Focused fixture HBP/WAT/PASS/loss/manifest parity passes.
- H2637 HBP/WAT/PASS/loss/manifest parity passes or the package closes in a
  named hold with exact residual fields and an out-of-envelope blocker.
- Direct production counters remain nonzero with
  `compatibility_edge_invocations=0`.
- Default compatibility behavior remains unchanged.
- Rust closure gates pass before `complete`: `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check`, unless the package closes
  in a named hold before full R7D closure.
- Scoped Markdown lint and `git diff --check` pass.

Security-impact gate:

- No secrets, tokens, credentials, or machine-local absolute paths are committed
  as normative config.
- Direct production remains explicit opt-in and fail-closed.

Review requirements:

- Dual local reviews with explicit finding disposition.
- Verification artifact labels `Static:` and `Ran:` evidence.
- Conservation/publication acceptance anti-tautology is reviewed before
  claiming HBP/WAT/PASS parity.
- `.rs` line-count governance: `2000+` lines is `WARN`; non-exempt `3000+`
  files block closure.

Final disposition:
`HOLD-R7D3-DIRECT-MOFE-DYNAMIC-CARRY-TRANSFER-ABSENT`.

Execution summary:

- Implemented typed direct WB14/R4K infiltration/depression producer inputs,
  hyetograph validation, Green-Ampt-style cumulative infiltration, top-two-layer
  storage capping, depression-storage handoff, and direct fail-closed malformed
  input guards.
- Wired R4K output into R4A runoff partition, WB18 same-pass infiltration
  ingress, R4N ET same-pass infiltration inputs, and direct publication day
  inputs sourced from lane-indexed seed surfaces.
- Reordered direct R4 spans so R4K executes before WB18/ET consumers.
- Added direct R4C storage-input override so scalar storage closure uses the
  same direct liquid supply as R4A/R4K on production direct runs.
- Added R4L direct hourly saturation-addback consumption from R4O
  `ui_SCrunf` carry arrays; this cleared the H2637 day-1213 storage closure
  failure after R4K made infiltration nonzero.
- Focused R4K and R4L tests pass; H2637 production direct now completes with
  zero compatibility-edge invocations and writes HBP/WAT/PASS/loss/manifest
  artifacts.
- H2637 full parity remains blocked. Loss JSON is byte-identical, but WAT,
  PASS, HBP, and manifest are not parity-clean. The dominant next root cause is
  missing dynamic MOFE same-day lane-to-lane carry transfer: direct manifest
  `mofe_hourly_carry.current_carry_total_m` and
  `upstream_carry_total_m` are `0.0`, while default compatibility reports
  `0.2205447764353141`. Direct downstream lanes therefore start with zero
  `UpStrmQ`/`SubRIn` and zero `QOFE`/PASS `runvol` despite upstream lanes
  producing saturation/lateral carry arrays.

Hold evidence:

- Same-binary H2637 default: `637.63 s / 227352 KiB`, exit 0.
- Same-binary H2637 direct production: `192.90 s / 643724 KiB`, exit 0.
- `H2637.loss.json` is byte-identical.
- WAT row/schema parity shape holds (`235961` rows, `34` columns), but values
  diverge materially: max absolute deltas include `Q=72.25917534435557 mm`,
  `Dp=29.488664492756087 mm`, `UpStrmQ=1316.4645543910933 mm`,
  `SubRIn=89.89968491102034 mm`, `Total-Soil=511.2314284277249 mm`, and
  `Interception=0.891459703930619 mm`.
- PASS row/schema parity shape holds (`12419` rows, `17` columns), but values
  diverge materially: `runvol` first row default `107.13682236123434` vs direct
  `0.0`; max absolute `runvol=14402.354720112891`; `sbrunv` max absolute
  `465.62385852543673`.
- Static executor review shows `DirectFrameExecutor` iterates day then lane and
  commits each lane only to itself. R4O/R4L publish hourly carry in
  per-lane shadows, but no same-day transfer mutates the downstream lane's
  `DirectTransferBuffers`, and R4J consumes constructor
  `DirectRunonCarryInputs` rather than those dynamic transfer buffers.

First follow-up action: scaffold and execute R7D4 to close
`HOLD-R7D3-DIRECT-MOFE-DYNAMIC-CARRY-TRANSFER-ABSENT` by adding a typed
same-day upstream-to-downstream MOFE carry producer in `DirectFrameExecutor`:
publish current-lane `ui_SCrunf`/`ui_LfCrf` arrays from R4L/R4O, copy them
with area scaling into the next lane's `ui_SUrunf`/`ui_LfUrf` transfer buffers
before that lane's R4J/R3A spans, make R4J consume the typed dynamic transfer
authority, and rerun H2637 parity until R7D closes or the next named
out-of-envelope blocker is proven.
