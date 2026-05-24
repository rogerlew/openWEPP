# 20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001

## Status
- state: complete
- date: 2026-05-24
- timezone: UTC

## Objective
Execute the full SIMIMPL02 assessment wave by producing a complete
legacy-to-openWEPP hillslope routine inventory and an evidence-linked
owner-surface gap-closure map that drives contract-first implementation
sequencing.

## Why This Package Exists
SIMIMPL01 identified high-impact runtime closure gaps and authored the
simulation implementation queue, but routine inventory and owner-surface mapping
remain incomplete (`GAP-SIMINV-001`).

SIMIMPL02 resolves that inventory gap so downstream contract and code packages
can execute against a complete, auditable routine map rather than sampled
anchors.

## Scope
### Included
- Enumerate full hillslope routine set for SIMIMPL scope from
  `/workdir/wepp-forest_260430_baseline/src` with explicit evidence anchors.
- Map each routine to openWEPP owner surface(s): `runner`, `orchestrator`,
  `kernel`, `output`, `contract`, or `unowned-gap`.
- Classify each routine as `mapped`, `partial`, `gap`, or `deferred` with
  rationale and evidence links.
- Link routine families to canonical contract surfaces and invariant families.
- Publish a deterministic routine gap-closure map for SIMIMPL03+ sequencing.
- Complete review/verification/gate/disposition artifacts for this package.

### Explicitly Out of Scope
- Production Rust/F90 kernel implementation changes.
- Canonical `SC-*` contract amendments (unless blocked by contradiction that
  must be documented for SIMIMPL03 intake).
- Replay rerun execution and parity acceptance decisions.

## Deliverables
1. Contract/authority execution evidence:
   - `artifacts/simimpl02-contract-implementation-evidence.md`
2. Full routine inventory:
   - `artifacts/simimpl02-full-hillslope-routine-inventory.md`
3. Owner-surface gap closure map:
   - `artifacts/simimpl02-routine-owner-surface-gap-closure-map.md`
4. Contract linkage and invariant crosswalk:
   - `artifacts/simimpl02-routine-contract-invariant-crosswalk.md`
5. Contract-test execution evidence:
   - `artifacts/simimpl02-contract-test-implementation-evidence.md`
6. Pre-implementation contract gate record:
   - `artifacts/simimpl02-preimplementation-contract-gate.md`
7. Implementation/test evidence log:
   - `artifacts/simimpl02-implementation-and-test-evidence.md`
8. Kernel-profile compliance checklist:
   - `artifacts/simimpl02-kernel-profile-compliance-checklist.md`
9. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl02_disposition.md`
   - `artifacts/worker-handoff.md`
10. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
11. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
For downstream code-authoring packages that consume SIMIMPL02 outputs, sequence
must remain:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

No downstream execution order may violate this sequence.

## Autonomous Execution Intent (Required)
This package is execution-ready and self-contained. Assigned agents are expected
to execute all package phases through disposition without requesting additional
user direction unless hard-blocked by missing local authority or contradictory
canonical requirements.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label evidence mode using `Static:` and/
or `Ran:` sections.

## Physics and Authority Posture
- Comparator/provenance baseline authority remains:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Consolidation architecture intake reference remains:
  `/workdir/wepp-forest/fpm-src` (selective intake only; no wholesale import).
- No physics invention is permitted.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simimpl01-hillslope-routine-gap-register.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simimpl01-pipeline-gap-audit.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simimpl01-watbal-authority-source-comparison.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simimpl01-watbal-consolidation-and-timestep-architecture.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simulation-implementation-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simimpl01_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-pl14s-tier-a-openwepp-candidate-emission-and-replay-001/artifacts/pl14s-tier-a-semantic-parity-delta-report.md`
- `/workdir/wepp-forest_260430_baseline`
- `/workdir/wepp-forest`

## Intended Write Set
- `docs/work-packages/20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase A - Intake and Authority Alignment
- Confirm dependency readability and SIMIMPL01 handoff assumptions.
- Confirm baseline/candidate provenance posture and evidence constraints.

### Phase B - Full Legacy Routine Inventory
- Enumerate complete SIMIMPL hillslope routine set and called surfaces.
- Record deterministic routine list with source anchors.

### Phase C - Owner-Surface and Contract Mapping
- Map each routine to openWEPP owner surface status and closure classification.
- Link routine families to canonical contract invariant surfaces.

### Phase D - Gap Closure Map and Queue Handoff
- Produce deterministic gap-closure map for SIMIMPL03+ queue consumers.
- Record unresolved/deferred items with ownership and sequencing rationale.

### Phase E - Review, Verification, Disposition
- Complete review/verification artifacts.
- Update gate results, manifest, and disposition.

## Exit Criteria
- Full routine inventory is complete and evidence-linked.
- Every routine has owner-surface classification and closure status.
- Contract/invariant crosswalk is present for covered routine families.
- Gap-closure map is deterministic and directly consumable by SIMIMPL03+.
- Governance/review/verification artifacts are complete with no placeholder
  state.
- Required repository gates are run and recorded if non-doc code changes are
  introduced:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Execution Record
- 2026-05-24: Phase A completed; authority intake aligned to required
  governance, canonical contract, and dependency artifact set.
- 2026-05-24: Phase B completed; deterministic transitive routine inventory
  extracted from baseline roots (`202` routines, `326` edges) with source
  anchors and unresolved-symbol disclosure.
- 2026-05-24: Phase C completed; per-routine owner-surface and closure-status
  mapping published with deterministic rationale coding.
- 2026-05-24: Phase D completed; contract/invariant crosswalk and queue-driving
  closure map finalized for SIMIMPL03+.
- 2026-05-24: Phase E completed; review, verification, gate, manifest,
  handoff, and disposition artifacts closed.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: SIMIMPL02 is an assessment/mapping package and does not require
  production execution-path mutation.
