# R7D2 Multi-OFE Lane Seed Authority Hold-Lift

Status: executed-held.

Package type: Array-native runtime defect-closure implementation package.

Objective: close `R7D-DIRECT-PRODUCTION-PUBLICATION-PARITY` by replacing
production direct's topology/area-only multi-OFE lane construction and
single-surface day-input seeding with lane-indexed typed direct constructor
authority.

Rationale: R7D proved the production direct consumer path already writes public
outputs from `DirectRunPublicationFrame`, not `execution.wb13_rows`. H2637
still fails HBP/WAT/PASS parity because `DirectProductionExecutor` seeds direct
lane frames from topology/area only and `DirectPublicationDayInputBuilder`
clones one aggregate `HillslopeWritebackSurface` for every lane. This package
must implement the lane-indexed producer authority needed to lift
`HOLD-R7D-MULTIOFE-DIRECT-LANE-SEED-AUTHORITY-ABSENT`.

Included scope:

- Build or expose a lane-indexed direct seed authority from parsed/static
  per-OFE soil, slope, management, PMET, snow, frost, layer, ET, transfer,
  geometry, and publication operands already prepared by runner intake.
- Convert that authority into `DirectLaneConstructorInputs` and production
  direct day-input producers before `DirectFrameExecutor` entry.
- Ensure multi-OFE production direct no longer clones a single aggregate
  runtime surface as lane seed authority.
- Add a multi-OFE anti-alias fixture proving at least lane 1 and lane 2 have
  intentionally different seed operands before execution.
- Iterate in-envelope blockers until focused and H2637 HBP/WAT/PASS/loss/
  manifest parity closes or a new named out-of-envelope blocker is reached.
- Keep default compatibility behavior unchanged and preserve explicit opt-in
  direct production selection.

Excluded scope:

- Default activation; compatibility remains the default mode.
- R7G performance/RSS closure unless a change is necessary to unblock parity.
- Deleting compatibility scheduler/runtime modules outside negative scans for
  production direct authority.
- New process-physics formulas or surrogate approximations. If parity requires
  missing science authority beyond moving existing parsed/static lane operands
  into typed direct state, close in `HOLD` with exact evidence and first
  action.

Intended write set:

- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260622-r7d2-multiofe-lane-seed-authority-001/**`

Dependencies:

- R7B typed direct constructor APIs.
- R7C production direct executor path.
- R7D executed-held producer-authority evidence.

Correction authority envelope:

- Defect: `R7D-DIRECT-PRODUCTION-PUBLICATION-PARITY`.
- Observed failure: H2637 direct production emits direct-source rows with zero
  compatibility-edge invocations but differs from compatibility for HBP, WAT,
  and PASS while focused one-OFE parity passes.
- In-scope corrections: lane-indexed direct seed authority, direct lane
  constructor input assembly, production direct day-input source selection,
  anti-alias fixtures, direct-publication producer/writer wiring needed for
  parity, and manifest/counter validation.
- Protected boundaries: do not use compatibility WB13 rows, compatibility
  public-output builders, aggregate runtime surfaces, stale logical state, or
  compatibility scheduler results as production direct authority. Existing
  parsed/static per-OFE surfaces may be used only as transitional extraction
  sources to create typed direct constructor/day inputs in this package; they
  may not remain the production direct day-input authority at closure.

Phase plan:

1. Scaffold package, catalog entry, prompt, and evidence placeholders.
2. Inventory current per-OFE lane state, direct constructor inputs, and
   production direct day-input authority.
3. Add failing/current-state anti-alias test that exposes lane seed aliasing.
4. Implement lane-indexed direct constructor/day-input authority and remove the
   production direct dependency on a single aggregate seed surface.
5. Re-run focused tests. If they expose the next in-envelope blocker, fix it in
   this package and keep iterating.
6. Re-run focused fixture and H2637 parity. If H2637 fails on a new
   in-envelope direct publication producer mismatch, fix it in this package and
   keep iterating.
7. Close complete only when R7D parity gates pass; otherwise close in `HOLD`
   only for a named out-of-envelope blocker with exact residual fields and the
   first implementation action.

Anti-premature-stop rule:

- Do not stop after a diagnostic run, a single failing field, a successful
  focused fixture, or one partial producer correction.
- Do not close as complete while any current-scope R7D gate is `FAIL`,
  `BLOCKED`, or unjustified `NOT RUN`.
- If a gate fails and the root cause is within the correction authority
  envelope, implement the next correction and rerun the gate.
- A hold is allowed only when the same blocker is out of envelope, lacks
  canonical authority, or requires a separate process-family contract. The hold
  must name the blocker, exact residual fields, and first code action.

Acceptance gates:

- Multi-OFE anti-alias fixture proves lane-indexed direct seed operands differ
  before execution.
- Production direct frame construction seeds each lane from lane-indexed typed
  direct constructor authority, not topology/area defaults.
- Production direct day-input production no longer clones one aggregate
  `HillslopeWritebackSurface` for every lane.
- Static scans prove production direct does not read `execution.wb13_rows`,
  compatibility public-output builders, or aggregate runtime surfaces as direct
  authority.
- Focused fixture HBP/WAT/PASS/loss/manifest parity passes.
- H2637 HBP/WAT/PASS/loss/manifest parity passes or the package closes in a
  named hold with exact residual fields and out-of-envelope blocker authority.
- Direct production counters remain nonzero with
  `compatibility_edge_invocations=0`.
- Default compatibility behavior remains unchanged.
- Rust closure gates pass: `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check`, unless the package closes
  in a named hold before full closure.
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

Execution summary:

- Implemented lane-indexed seed authority for production direct day-input
  construction. `DirectPublicationDayInputBuilder` now accepts lane seed
  surfaces, derives profile projection inputs per lane, and rejects missing
  lane/profile seed authority.
- Production direct frame construction now seeds each lane from
  `OfeLanePersistentStateSequence` when multi-OFE lane state exists, and fails
  closed for multi-OFE direct production when no lane-indexed seed authority is
  available.
- Added the anti-alias test
  `r7d2_direct_seed_authority_is_lane_indexed_for_multiofe_profiles`, proving
  lane 1 and lane 2 direct seed operands do not share profile depth, porosity
  capacity, field-capacity storage, or wilting-point storage.
- Focused one-OFE HBP/loss/PASS/WAT parity remains clean.
- H2637 direct production improved materially but still fails HBP/PASS/WAT
  parity. The residual is no longer aggregate seed/profile aliasing: direct day
  1 ET/profile/storage varies by lane after this package, but direct day 1
  `Q` remains equal to raw liquid input (`45.2 mm`) because direct R4K forwards
  zero infiltration/depression handoff inputs and direct R4A therefore computes
  runoff as liquid input.

Hold evidence:

- `SC-RUNOFFPART-001` and `SC-WATBAL-001` require baseline-authoritative WB14
  hyetograph/Green-Ampt infiltration production, same-pass WB12 infiltration
  and depression-storage handoff, and explicit storage-ingress lineage.
- The current direct runtime R4K phase has the span/operand shape but no
  producer: `DirectInfiltrationDepressionInputs::zero()` is the only production
  direct handoff source, and the only `wb12_infiltration` values visible in the
  runner are compatibility trace/runtime-surface observations.
- Wrapping those compatibility observations as direct authority would violate
  this package's forbidden-source rule and the runoff-partition anti-clone
  contracts.

Final disposition:
`HOLD-R7D2-DIRECT-WB14-R4K-INFILTRATION-PRODUCER-AUTHORITY-ABSENT`.

First follow-up action: scaffold and execute an R7D3 package that promotes a
baseline-authoritative direct WB14/R4K infiltration/depression producer into
`DirectFrameExecutor`, feeds R4A runoff, WB18 percolation same-pass
infiltration, ET same-pass infiltration, and direct publication projection, and
iterates H2637 parity until R7D closes or the next named out-of-envelope
process blocker is proven.
