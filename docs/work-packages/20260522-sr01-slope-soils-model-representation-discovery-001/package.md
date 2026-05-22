# 20260522-sr01-slope-soils-model-representation-discovery-001

## Status
- state: active
- date: 2026-05-22
- timezone: UTC

## Objective
Establish exactly how slope and soil model representations are implemented in
`/workdir/wepp-forest_260430_baseline`, map all representation consumers and
ownership boundaries, and produce an openWEPP architecture-fit proposal that
defines whether slope+soil is the correct immediate boundary or whether a
larger extension boundary should be executed as a sequenced series.

## Why This Package Exists
After CLIM04, climate-runtime integration requires an explicit pause until
slope/soil representation semantics and consumer ownership are unambiguous.
Without this, downstream climate-consumer integration risks coupling to
incorrect geometry/layer/state abstractions.

## Scope
### Included
- Baseline source investigation of slope representation semantics in
  `/workdir/wepp-forest_260430_baseline`.
- Baseline source investigation of soil representation semantics in
  `/workdir/wepp-forest_260430_baseline`.
- Consumer inventory and ownership mapping for slope/soil representations
  across hydrology/erosion/climate-coupled kernels.
- Architecture-fit analysis for openWEPP typed seam/state constraints.
- Boundary decision: confirm slope+soil as current implementation boundary, or
  define a larger logical extension boundary and sequenced follow-on packages.
- Dual review and dual verification artifacts.

### Explicitly Out of Scope
- Implementing slope/soil runtime code changes.
- Editing CLIM01 artifacts or climate queue sequencing in this package.
- Single-storm climate/modeling support.

## Deliverables
1. Baseline slope representation map:
   - `artifacts/wepp-forest-slope-representation-map.md`
2. Baseline soil representation map:
   - `artifacts/wepp-forest-soil-representation-map.md`
3. Slope/soil consumer ownership map:
   - `artifacts/slope-soil-consumer-ownership-map.md`
4. openWEPP architecture-fit analysis:
   - `artifacts/openwepp-slope-soil-architecture-fit-analysis.md`
5. Boundary decision record:
   - `artifacts/slope-soil-boundary-decision-record.md`
6. Follow-on series queue:
   - `artifacts/slope-soil-follow-on-wp-queue.md`
7. Worker/governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/sr01_disposition.md`
8. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md`
- `/workdir/wepp-forest_260430_baseline/`
- `/workdir/wepppy/`
- `/workdir/wepppyo3/`

## Intended Write Set
- `docs/work-packages/20260522-sr01-slope-soils-model-representation-discovery-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Inventory baseline slope/soil representation sources and current openWEPP
  parser/contract surfaces.

### Phase 1 - Representation Reconstruction
- Reconstruct canonical slope and soil data models, layer/profile semantics,
  and initialization/normalization rules from baseline source.

### Phase 2 - Consumer and Architecture Mapping
- Map all primary consumers and ownership boundaries.
- Evaluate fit against openWEPP typed seam/state architecture.

### Phase 3 - Boundary Decision and Queue
- Decide boundary scope: slope+soil only vs logical extension boundary.
- Author sequenced follow-on work-package queue.
- Complete dual review + dual verification and final disposition.

## Exit Criteria
- Slope and soil baseline representation semantics are documented with direct
  evidence links.
- Consumer ownership map is complete and actionable.
- Boundary decision record is explicit and non-ambiguous.
- Follow-on queue is dependency-ordered and implementation-ready.
- Dual review and verification artifacts are complete.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: discovery/specification package only.
