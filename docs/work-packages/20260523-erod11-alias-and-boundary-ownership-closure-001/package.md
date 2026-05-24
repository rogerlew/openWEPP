# 20260523-erod11-alias-and-boundary-ownership-closure-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Close Wave-0 erosion-lane alias and boundary ownership ambiguity by ratifying
explicit canonical-to-runtime symbol ownership maps and contract authority
updates across sediment-coupled companion contracts.

## Why This Package Exists
EROD10 ratified `EROD11` as the first mandatory Wave-0 gate before any
production erosion-kernel implementation package. Current companion contracts
retain non-promotable alias/ownership gaps (`GAP-SED-002`, `GAP-HYD-002`,
`GAP-ROUTE-002`, `GAP-WATBAL-003`, `GAP-RUNOFFPART-002`) that keep erosion-lane
execution in `HOLD` until explicit ownership closure is completed.

This package is governance/contracts scoped. It does not implement production
erosion kernel physics; it establishes authoritative alias and ownership closure
required to release Wave-1 kernel authoring. Scaffolded/placeholder physics
posture is explicitly non-acceptable for closure claims and must remain
gated as `HOLD` until authority-backed implementation evidence exists.

## Scope
### Included
- Author canonical-to-runtime alias mapping authority for erosion-lane boundary
  symbols across required `SC-*` contracts.
- Author explicit producer/consumer ownership register for erosion-coupled
  boundaries.
- Amend canonical contracts to close alias-specific non-promotable gaps where
  closure is supportable by authority.
- Add explicit governance guards that keep erosion-lane follow-ons in `HOLD`
  when scaffolded/placeholder physics behavior is detected or undispositioned.
- Produce dual review/disposition/verification artifacts for alias-ownership
  closure claims.
- Publish Wave-0 gate verdict for `EROD12` readiness.

### Explicitly Out of Scope
- Production erosion kernel implementation (`EROD13+`).
- OFE/enrichment runtime authoring (`EROD14`).
- Routing production-kernel implementation (`WS10`) beyond referenced
  dependency state.
- Tier-A hold-lift closeout disposition changes beyond EROD11 scope.

## Deliverables
1. Alias-ownership authority evidence:
   - `artifacts/erod11-alias-ownership-authority-evidence.md`
2. Canonical-to-runtime alias matrix:
   - `artifacts/erod11-canonical-runtime-alias-matrix.md`
3. Cross-contract boundary ownership register:
   - `artifacts/erod11-cross-contract-boundary-ownership-register.md`
4. Contract implementation evidence:
   - `artifacts/erod11-contract-implementation-evidence.md`
5. Contract-test implementation evidence:
   - `artifacts/erod11-contract-test-implementation-evidence.md`
6. Pre-implementation contract gate:
   - `artifacts/erod11-preimplementation-contract-gate.md`
7. Implementation/test evidence:
   - `artifacts/erod11-implementation-and-test-evidence.md`
8. Wave-0 gate verdict artifact:
   - `artifacts/erod11-wave0-gate-verdict.md`
9. Kernel profile compliance checklist:
   - `artifacts/erod11-kernel-profile-compliance-checklist.md`
10. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/erod11_disposition.md`
11. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement canonical contract updates in `SC-*` files.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Only then apply production code edits.

This sequence is mandatory. Any deviation keeps package disposition in `HOLD`.

## Truthfulness Labeling Requirement
All evidence artifacts in this package must explicitly declare evidence class
using `Static:` and/or `Ran:` sections. Claims without explicit evidence mode
labeling are non-compliant and block closure.

## Physics Authority and Provenance Requirement
- Canonical physics/equation authority for migration/parity claims must live in
  `docs/specifications/science-contracts/contracts/SC-*.md`; package-local
  notes are not authority.
- Legacy provenance defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` per ADR-0012 unless explicitly
  justified.
- No invented physics is allowed: equations/constants/guards/invariants must
  trace to canonical contract citations.
- Variable naming continuity with legacy WEPP symbols is required; differing
  runtime names must be documented by explicit alias maps in canonical
  contracts.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod10-sediment-kernelization-intake-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod10-sediment-kernelization-intake-001/artifacts/erod10-intake-decision-and-scope.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod10-sediment-kernelization-intake-001/artifacts/erod10-contract-authority-mapping.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod10-sediment-kernelization-intake-001/artifacts/erod10-wave-execution-plan.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod10-sediment-kernelization-intake-001/artifacts/erod10_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/pl15-pl08-hold-lift-decision-record.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/claude-pl15-pre-closeout-physics-review.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb16-peak-runoff-kernel-001/artifacts/wb16_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-ws10-channel-impoundment-production-kernels-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-ws10-channel-impoundment-production-kernels-001/artifacts/ws10_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/**`
- `docs/work-packages/20260523-erod11-alias-and-boundary-ownership-closure-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm EROD10 Wave-0 gate scope and current non-promotable alias gap set.

### Phase 1 - Alias Authority Mapping
- Build canonical-to-runtime symbol alias matrix and producer/consumer
  ownership table across required companion contracts.

### Phase 2 - Contract/Test/Gate Before Code
- Implement canonical `SC-*` alias/ownership amendments.
- Implement contract-derived tests.
- Execute and record pre-implementation contract gate evidence.
- Execute dual review + disposition + verification workflow.

### Phase 3 - Wave-0 Gate Verdict
- Publish explicit `GO`/`HOLD` verdict for EROD12 entry criteria.

## Exit Criteria
- EROD10 alias-ambiguity gate (`EROD10-AH-001`) is closed or explicitly
  authority-dispositioned with `HOLD` retention rationale.
- Alias/ownership mapping for erosion-lane boundaries is explicit in canonical
  contracts (not only work-package artifacts).
- Non-promotable alias-gap rows are updated with concrete closure posture.
- Contract-first sequence is evidenced in artifacts:
  1. contract implementation,
  2. contract-test implementation,
  3. pre-implementation contract gate,
  4. production edit evidence (if any).
- Dual review/disposition/verification artifacts are complete and evidence
  labeled (`Static:` / `Ran:`).
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: governance/contracts package; no direct production kernel
  implementation in intended scope.
