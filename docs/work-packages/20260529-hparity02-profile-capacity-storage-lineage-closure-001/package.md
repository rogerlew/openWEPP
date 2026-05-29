# 20260529-hparity02-profile-capacity-storage-lineage-closure-001

## Status
- state: queued
- date: 2026-05-29
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute HPARITY02 to close profile-capacity and storage-lineage parity for
`ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, and `ProfileWPStore`
in hillslope `H.wat` outputs.

## Why This Package Exists
HPARITY01 establishes that the four profile-capacity columns fail semantic
tolerance for all 39 hillslopes with consistent residual signatures. These
columns are foundational for downstream soil-water and flux columns; they must
be closed first before RM/ET/snow and percolation/subsurface families.

## Scope
### Included
- Contract amendments for profile-depth and storage-capacity derivations from
  baseline-authoritative soil/state lineage.
- Contract-derived tests for profile-capacity publication invariants.
- Production runtime/publication changes needed to emit baseline-authoritative
  profile-capacity columns.
- Cohort rerun evidence on `unpalatable-rind` hillslopes for this column family.

### Explicitly Out of Scope
- RM/Ep/Es/Snow-Water physics closure (HPARITY03 scope).
- Dp/latqcc/SoilWaterTotal/Total-Soil closure (HPARITY04 scope).
- Watershed hold-lift closeout (HPARITY05 scope).

## Closure Measures (Required)
1. `MEASURE-HP02-001`: `ProfileDepth` fail count across the 39-hillslope
   cohort is reduced from `39` to `0`.
2. `MEASURE-HP02-002`: `ProfilePorosityCap`, `ProfileFCStore`,
   `ProfileWPStore` fail counts across the cohort are each reduced from `39` to
   `0`.
3. `MEASURE-HP02-003`: row-presence integrity remains intact (`1461` common
   rows per hillslope, zero baseline-only/candidate-only rows).
4. `MEASURE-HP02-004`: previously always-pass control columns
   (`Area`, `P`, `Q`, `Er`, `Tile`, `Irr`, `QOFE`, `SubRIn`, `UpStrmQ`,
   `frozwt`) remain passing.

## Deliverables
1. `artifacts/hparity02-profile-capacity-gap-matrix.md`
2. `artifacts/hparity02-contract-implementation-evidence.md`
3. `artifacts/hparity02-contract-test-implementation-evidence.md`
4. `artifacts/hparity02-preimplementation-contract-gate.md`
5. `artifacts/hparity02-implementation-and-test-evidence.md`
6. `artifacts/hparity02-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hparity02_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement required canonical contract/index amendments for profile-capacity
   lineage and invariants.
2. Implement contract-derived tests for profile-capacity publication surfaces.
3. Record pre-implementation contract-gate evidence before production edits.
4. Apply production runtime/publication edits for this column family only.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority lives in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Physics and symbol provenance must trace to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No surrogate/proxy profile-capacity equations are permitted.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hparity01-hillslope-wat-always-fail-gap-mapping-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/hillslope_semantic_summary.json`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-hparity02-profile-capacity-storage-lineage-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-output/src/hillslope_wat.rs`
- `tests/integration/hparity02_profile_capacity_parity_contract.rs`
- `tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HPARITY02 authorization from HPARITY01 handoff and freeze to profile
  capacity/storage columns only.

### Phase B - Contract/spec authority updates
- Amend canonical contract rows for profile-capacity derivations and invariants.
- Update science-contract index references for HPARITY02 scope.

### Phase C - Contract-derived tests
- Add profile-capacity lineage/parity tests with explicit message IDs and
  invariants.

### Phase D - Pre-implementation contract gate
- Record contract/test readiness before production edits.

### Phase E - Production implementation
- Implement runtime/publication lineage closure for the four profile-capacity
  columns.

### Phase F - Validation and parity rerun
- Execute:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Rerun 39-hillslope `unpalatable-rind` semantic comparison.

### Phase G - Dual review, dual verification, disposition
- Complete dual review/verification artifacts and publish disposition with
  explicit residual deltas for this family.

## Exit Criteria
- Closure measures `MEASURE-HP02-001..004` are satisfied and evidenced.
- Profile-capacity family no longer appears in always-fail column reports.
- Package handoff clearly identifies remaining families for HPARITY03/04.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contract/runtime/test updates only; no external auth/network
  changes.
