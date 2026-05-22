# 20260522-pl01-plant-landuse-growth-decomposition-model-representation-discovery-001

## Status
- state: queued
- date: 2026-05-22
- timezone: UTC

## Objective
Establish exactly how plant/landuse/growth/decomposition model
representations are implemented downstream of management (`.man`) surfaces,
map consumer ownership boundaries, and produce an openWEPP architecture-fit
boundary proposal for sequenced implementation follow-ons.

## Why This Package Exists
openWEPP now has slope/soil seam closure through SR07 and a full typed
management datamodel path, but downstream plant/landuse/growth/decomposition
representation authority and consumer ownership are not yet consolidated in a
single discovery package. Without this discovery checkpoint, implementation
work risks coupling to ambiguous state abstractions and mixed ownership
boundaries.

## Scope
### Included
- Baseline source investigation of plant, landuse-management coupling, growth,
  and decomposition representation semantics in
  `/workdir/wepp-forest_260430_baseline`.
- Reconstruction of canonical symbol/state surfaces downstream of `.man`
  sections and scenario references.
- Consumer inventory and ownership mapping across ET/water-balance/soil/
  residue/erosion-coupled pathways.
- Architecture-fit analysis for openWEPP typed seam/state boundaries.
- Boundary decision: confirm immediate implementation boundary and publish a
  dependency-ordered follow-on queue.
- Dual review and dual verification artifact workflow.

### Explicitly Out of Scope
- Implementing plant/landuse/growth/decomposition runtime kernels.
- Re-scoping completed SR/CLIM package dispositions.
- Broad comparator campaign execution beyond discovery evidence needs.

## Deliverables
1. Baseline plant representation map:
   - `artifacts/wepp-forest-plant-representation-map.md`
2. Baseline landuse/management-coupling representation map:
   - `artifacts/wepp-forest-landuse-management-representation-map.md`
3. Baseline growth representation map:
   - `artifacts/wepp-forest-growth-representation-map.md`
4. Baseline decomposition representation map:
   - `artifacts/wepp-forest-decomposition-representation-map.md`
5. Consumer ownership map:
   - `artifacts/plant-landuse-growth-decomposition-consumer-ownership-map.md`
6. openWEPP architecture-fit analysis:
   - `artifacts/openwepp-plant-landuse-growth-decomposition-architecture-fit-analysis.md`
7. Boundary decision record:
   - `artifacts/plant-landuse-growth-decomposition-boundary-decision-record.md`
8. Follow-on series queue:
   - `artifacts/plant-landuse-growth-decomposition-follow-on-wp-queue.md`
9. Worker/governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/pl01_disposition.md`
10. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md`
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl09-management-full-typed-datamodel-001/package.md`
- `/workdir/wepp-forest_260430_baseline/`
- `/workdir/wepppy/`
- `/workdir/wepppyo3/`

## Intended Write Set
- `docs/work-packages/20260522-pl01-plant-landuse-growth-decomposition-model-representation-discovery-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Inventory baseline plant/management-downstream representation sources and
  current openWEPP parser/contract surfaces.

### Phase 1 - Representation Reconstruction
- Reconstruct canonical plant, landuse-management coupling, growth, and
  decomposition data/state models from baseline source and contract authority.

### Phase 2 - Consumer and Architecture Mapping
- Map primary consumers and ownership boundaries across downstream domains.
- Evaluate fit against openWEPP typed seam/state architecture.

### Phase 3 - Boundary Decision and Queue
- Decide boundary scope and sequencing strategy for follow-on implementation
  packages.
- Author dependency-ordered follow-on queue.
- Complete dual review + dual verification and final disposition.

## Exit Criteria
- Plant/landuse/growth/decomposition baseline representation semantics are
  documented with direct evidence links.
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
