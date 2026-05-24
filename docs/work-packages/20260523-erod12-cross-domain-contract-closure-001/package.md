# 20260523-erod12-cross-domain-contract-closure-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Close Wave-0 cross-domain erosion companion-contract ownership and guard
semantics so remaining non-promotable erosion-lane governance blockers are
authoritatively dispositioned before EROD13 production kernel authoring.

## Why This Package Exists
EROD10 ratified `EROD12-cross-domain-contract-closure-001` as the second
mandatory Wave-0 gate after EROD11 alias-ownership closure. EROD11 is now
closed and WB19/WB20 are completed, but cross-domain non-promotable companion
gaps remain open and continue to enforce erosion-physics `HOLD`.

This package is contract/governance scoped. It must disposition cross-domain
ownership/guard semantics in canonical `SC-*` contracts and produce dual
review/disposition/verification evidence. It does not implement production
erosion kernel physics.

## Scope
### Included
- Close cross-domain ownership and guard semantics for erosion-lane companion
  contracts in canonical authority files.
- Disposition remaining non-promotable cross-domain gaps or explicitly retain
  them with authority-backed `HOLD` rationale.
- Reconcile cross-contract guard ownership matrix across sediment,
  hydraulics, routing, runoff partition, and water-balance boundaries.
- Implement contract-derived tests that assert final cross-domain
  promotability posture and blocker retention rules.
- Record pre-implementation contract-gate evidence before any production code
  edits (if any become in-scope by explicit approval).
- Publish explicit EROD13 entry verdict (`GO`/`HOLD`) with rationale.

### Explicitly Out of Scope
- Production erosion kernel implementation (`EROD13+`).
- OFE/enrichment runtime authoring (`EROD14`).
- Routing production-kernel implementation beyond governance closure.
- Comparator closeout package work (`EROD16`).

## Deliverables
1. Cross-domain closure authority evidence:
   - `artifacts/erod12-cross-domain-contract-closure-evidence.md`
2. Cross-domain ownership/guard matrix:
   - `artifacts/erod12-cross-domain-ownership-and-guard-matrix.md`
3. Companion-gap disposition register:
   - `artifacts/erod12-companion-gap-disposition-register.md`
4. Contract implementation evidence:
   - `artifacts/erod12-contract-implementation-evidence.md`
5. Contract-test implementation evidence:
   - `artifacts/erod12-contract-test-implementation-evidence.md`
6. Pre-implementation contract gate:
   - `artifacts/erod12-preimplementation-contract-gate.md`
7. Implementation/test evidence:
   - `artifacts/erod12-implementation-and-test-evidence.md`
8. Wave-0 release verdict artifact:
   - `artifacts/erod12-wave0-release-verdict.md`
9. Kernel profile compliance checklist:
   - `artifacts/erod12-kernel-profile-compliance-checklist.md`
10. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/erod12_disposition.md`
11. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement canonical contract updates in `SC-*` files.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Only then apply production code edits (if explicitly in-scope).

Any deviation keeps package disposition in `HOLD`.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label evidence mode using `Static:`
and/or `Ran:` sections. Claims without explicit evidence labeling are
non-compliant.

## Physics Authority and Provenance Requirement
- Canonical physics/equation authority for migration/parity claims must live in
  `docs/specifications/science-contracts/contracts/SC-*.md`; package-local
  notes are not authority.
- Legacy provenance defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified.
- No invented physics is allowed: equations/constants/guards/invariants must
  trace to canonical citations.
- Preserve legacy WEPP canonical variable naming continuity; when runtime names
  differ, use explicit alias mappings in canonical contracts.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod10-sediment-kernelization-intake-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod10-sediment-kernelization-intake-001/artifacts/erod10-intake-decision-and-scope.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod10-sediment-kernelization-intake-001/artifacts/erod10-contract-authority-mapping.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod10-sediment-kernelization-intake-001/artifacts/erod10-wave-execution-plan.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod11-alias-and-boundary-ownership-closure-001/artifacts/erod11_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod11-alias-and-boundary-ownership-closure-001/artifacts/erod11-wave0-gate-verdict.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb19-lateral-drainage-physics-equivalence-port-001/artifacts/wb19_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb20-forward-water-balance-solver-lane-001/artifacts/wb20_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/pl15-pl08-hold-lift-decision-record.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/**`
- `docs/work-packages/20260523-erod12-cross-domain-contract-closure-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm EROD10 Wave-0 cross-domain gate scope and current blocker set.

### Phase 1 - Cross-Domain Contract Closure
- Implement canonical `SC-*` cross-domain ownership/guard closure amendments.
- Reconcile promotability posture for remaining Wave-0 blocker rows.

### Phase 2 - Contract-Test + Gate (Pre-Implementation)
- Implement contract-derived tests for blocker posture and guard ownership.
- Execute and record pre-implementation contract-gate evidence.

### Phase 3 - Review/Disposition/Verification
- Execute dual independent review workflow and disposition findings.
- Execute dual verification workflow and publish final package verdict.

### Phase 4 - Wave-0 Verdict
- Publish explicit EROD13 entry verdict (`GO`/`HOLD`) with rationale.

## Exit Criteria
- Remaining Wave-0 cross-domain blocker rows are explicitly dispositioned in
  canonical contracts with authority-backed rationale.
- Cross-domain ownership/guard matrix is explicit and canonicalized.
- Contract-derived tests enforce closed vs non-promotable posture as ratified.
- Contract-first sequence evidence is complete and truthfully labeled.
- Dual review/disposition/verification artifacts are complete.
- Production erosion-physics implementation remains `HOLD` unless all gates are
  explicitly satisfied and approved.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: governance/contracts package; no direct production erosion kernel
  implementation in intended scope.
