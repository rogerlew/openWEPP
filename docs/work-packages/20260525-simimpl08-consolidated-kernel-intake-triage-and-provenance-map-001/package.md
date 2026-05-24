# 20260525-simimpl08-consolidated-kernel-intake-triage-and-provenance-map-001

## Status
- state: complete
- date: 2026-05-24
- timezone: UTC

## Objective
Execute SIMIMPL08 end-to-end by building a per-kernel consolidated intake map
(`wbk*` family) from `/workdir/wepp-forest/fpm-src` to baseline and canonical
contract authority, classifying each candidate element as `adopt`, `defer`, or
`reject` with explicit provenance rationale.

## Why This Package Exists
SIMIMPL01 and SIMIMPL03 established that consolidated architecture intake must
be selective and contract-governed. SIMIMPL08 closes `GAP-SIMCONS-001` at the
planning/authority layer by producing a bounded adoption set that SIMIMPL09+ can
implement without uncontrolled behavior drift.

## Scope
### Included
- Inventory consolidated watbal candidate surfaces (`wbk*` kernels,
  daily/hourly adapters, policy modules) from `/workdir/wepp-forest/fpm-src`.
- Map each candidate surface to:
  - baseline legacy provenance anchors (`wepp-forest_260430_baseline`),
  - canonical contract authority surfaces (`SC-WATBAL-001`, `SC-SYSTEM-001`,
    `SC-INFILE-WEPPUI-001`).
- Classify each candidate item as `adopt`, `defer`, or `reject` with rationale,
  risk class, and dependency notes.
- Publish a bounded adoption recommendation for SIMIMPL09 hourly foundation
  planning.
- Complete governance/review/verification/disposition artifacts.

### Explicitly Out of Scope
- Production Rust/F90 code integration of consolidated kernels.
- Contract-derived runtime test implementation.
- Replay execution or parity acceptance decisions.

## Deliverables
1. Contract/authority evidence:
   - `artifacts/simimpl08-contract-implementation-evidence.md`
2. Consolidated kernel inventory:
   - `artifacts/simimpl08-consolidated-kernel-inventory.md`
3. Provenance triage matrix (`adopt`/`defer`/`reject`):
   - `artifacts/simimpl08-provenance-triage-matrix.md`
4. Adoption boundary recommendation:
   - `artifacts/simimpl08-adoption-boundary-recommendation.md`
5. Contract-test implementation evidence:
   - `artifacts/simimpl08-contract-test-implementation-evidence.md`
6. Pre-implementation contract gate:
   - `artifacts/simimpl08-preimplementation-contract-gate.md`
7. Implementation/test evidence log:
   - `artifacts/simimpl08-implementation-and-test-evidence.md`
8. Kernel-profile compliance checklist:
   - `artifacts/simimpl08-kernel-profile-compliance-checklist.md`
9. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl08_disposition.md`
   - `artifacts/worker-handoff.md`
10. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
11. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
SIMIMPL08 is an authority/triage package and must preserve contract-first
constraints for downstream implementation packages:
1. canonical contract amendments (SIMIMPL03),
2. contract-derived tests + pre-implementation gate (SIMIMPL04),
3. production integrations only after gate closure.

No SIMIMPL08 recommendation may bypass this ordering.

## Autonomous Execution Intent (Required)
This package is execution-ready and self-contained. Assigned agents must execute
all phases through disposition without requesting additional user direction
unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label evidence mode using `Static:` and/
or `Ran:` sections.

## Physics and Authority Posture
- Baseline comparator/provenance authority remains:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Consolidated candidate intake source is `/workdir/wepp-forest/fpm-src`.
- No physics invention is permitted.
- Policy/clamp modules are non-authorized by default unless explicitly
  contract-governed and dispositioned.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simulation-implementation-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simimpl01-watbal-authority-source-comparison.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001/artifacts/simimpl02-routine-contract-invariant-crosswalk.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03-contract-amendment-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl07-wepp-ui-hourly-branch-propagation-and-closure-001/artifacts/simimpl07_disposition.md`
- `/workdir/wepp-forest_260430_baseline`
- `/workdir/wepp-forest`

## Intended Write Set
- `docs/work-packages/20260525-simimpl08-consolidated-kernel-intake-triage-and-provenance-map-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase A - Intake and Authority Alignment
- Confirm dependency readability and queue authority.
- Confirm baseline/candidate authority posture and triage constraints.

### Phase B - Consolidated Kernel Inventory
- Enumerate candidate consolidated kernel/adapter/policy surfaces.
- Record deterministic source anchors and grouping.

### Phase C - Provenance and Contract Triage
- Map candidate items to baseline and contract invariants.
- Classify each item as `adopt`, `defer`, or `reject` with rationale.

### Phase D - Adoption Boundary Recommendation
- Produce bounded adoption recommendation for SIMIMPL09+.
- Record residual risk and deferred-item handling requirements.

### Phase E - Review, Verification, Disposition
- Complete dual review/disposition + dual verification.
- Finalize gate results, owned-file manifest, and worker handoff.

## Exit Criteria
- Per-kernel consolidated intake map is complete and evidence-linked.
- Every candidate item is dispositioned as `adopt`, `defer`, or `reject`.
- Adoption boundary is explicit, bounded, and consumable by SIMIMPL09+.
- Governance/review/verification artifacts are complete with no queued
  placeholders.
- Required repository gates are run and recorded only if non-doc code changes
  are introduced:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: authority/triage package only; no production execution-path edits.
