# 20260522-pl08-comparator-confidence-tier-review-001

## Status
- state: hold
- date: 2026-05-22
- timezone: UTC

## Objective
Run single-OFE daily water-balance plus plant/residue parity comparator review
after PL05/PL06 integration and record confidence-tier delta disposition.

## Why This Package Exists
PL01 follow-on sequencing defines PL08 as the comparator closeout checkpoint
after PL05/PL06 scaffold integration and PL07 parser-to-runtime coverage
closure. Per ADR-0011 confidence-tier policy and repo numerics posture, this
package provides high-confidence acceptance-direction evidence on Tier-A
surfaces while treating broader divergences as investigation signals.

## Scope
### Included
- Execute Tier-A comparator review for single-OFE daily water-balance surfaces
  with PL growth/decomposition-relevant outputs in scope.
- Investigate plant/residue parity direction after PL05/PL06 scaffolding.
- Classify and document comparator deltas with explicit confidence-tier
  semantics and typed disposition rationale.
- Record comparator input set, baseline provenance, command traces, and output
  evidence for reproducibility.

### Explicitly Out of Scope
- Tier-B hourly/watershed comparator closeout as a hard rejection rule.
- Bit-for-bit parity certification across platforms.
- New parser/runtime kernel rewiring beyond PL05/PL06/PL07 validated surfaces.

## Deliverables
1. Tier-A comparator delta report:
   - `artifacts/single-ofe-daily-water-balance-comparator-delta-report.md`
2. Plant/residue parity investigation report:
   - `artifacts/plant-residue-parity-investigation.md`
3. Comparator run/provenance manifest:
   - `artifacts/comparator-run-provenance-manifest.md`
4. Comparator confidence-tier disposition:
   - `artifacts/comparator-confidence-tier-disposition.md`
5. Semantic-parity direction assessment:
   - `artifacts/semantic-parity-direction-assessment.md`
6. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/pl08_disposition.md`
7. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl05-growth-kernel-surface-scaffolding-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl05-growth-kernel-surface-scaffolding-001/artifacts/pl05-growth-surface-test-evidence.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl06-decomposition-resup-kernel-surface-scaffolding-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl06-decomposition-resup-kernel-surface-scaffolding-001/artifacts/pl06-decomposition-surface-test-evidence.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl07-parser-to-runtime-integration-tests-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl07-parser-to-runtime-integration-tests-001/artifacts/pl07-parser-to-runtime-integration-evidence.md`
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/home/workdir/openWEPP/docs/decisions/0003-parity-semantic-not-bit.md`
- `/home/workdir/openWEPP/docs/numerics/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260520-arch01-subsystem-map-and-contract-spine/artifacts/comparator-confidence-tier-policy.md`
- `/workdir/wepp-forest_260430_baseline`

## Intended Write Set
- `docs/work-packages/20260522-pl08-comparator-confidence-tier-review-001/**`
- `docs/work-packages/README.md`
- `tests/integration/**` (only if comparator-run harness assertions require)
- `crates/openwepp-comparator-metadata/**` (only if typed disposition metadata refinement is required)

## Phase Plan
### Phase 0 - Intake
- Confirm PL05/PL06/PL07 closure claims and comparator target surfaces.
- Confirm baseline provenance anchors and comparator harness entrypoints.

### Phase 1 - Comparator Execution
- Run Tier-A single-OFE daily water-balance comparator lane.
- Capture raw deltas and plant/residue direction signals.

### Phase 2 - Delta Disposition
- Classify deltas using confidence-tier policy and semantic-parity standards.
- Separate acceptance-direction signals from unresolved blockers/investigation
  items.

### Phase 3 - Verification
- Execute required gates (when code changes are in scope) and record evidence.

### Phase 4 - Disposition
- Finalize review/verification artifacts and package disposition.

## Exit Criteria
- Tier-A comparator review is executed with reproducible evidence.
- Plant/residue parity direction is explicitly assessed and source-backed.
- Confidence-tier disposition clearly distinguishes acceptance signals vs
  unresolved blockers.
- Baseline comparator provenance (commit/hash/binary identity) is recorded.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: comparator evidence/disposition package with optional metadata/test
  refinements only.

## Execution Result

- Tier-A comparator replay executed for single-OFE fixture `p5` using pinned
  baseline authority and persisted JSON evidence for `H5.wat.dat` and
  `H5.plot.dat`.
- `H5.wat.dat` comparator reports `structure_diff`; shared 20-column daily rows
  align exactly by `(OFE,J,Y)` key, while baseline-only trailing columns drive
  structural mismatch.
- Plant/residue surrogate signal is positive (`Ep`, `Es`, `Er` keyed parity and
  `H5.plot.dat` strict pass), but openWEPP-vs-legacy Tier-A candidate output
  surface remains unavailable in this workspace.
- Per Tier-A policy, unresolved blocker keeps PL08 disposition in `HOLD`.
