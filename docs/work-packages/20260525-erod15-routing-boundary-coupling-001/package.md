# 20260525-erod15-routing-boundary-coupling-001

## Status
- state: queued
- date: 2026-05-25
- timezone: UTC

## Objective
Implement Wave-3 routing-boundary sediment coupling in openWEPP with
contract-first sequencing, canonical `SC-*` authority, typed handoff
semantics, and explicit `INV-SED-010` payload export validation.

## Why This Package Exists
EROD10 defines `EROD15` as Wave-3 after `EROD14` completion and after upstream
watershed production-kernel readiness (`WS10`) is no longer placeholder-only.
EROD14 is now complete with a `GO` verdict and explicit EROD15 entry signal.

This package executes Wave-3 by wiring sediment-routing boundary exports,
ensuring route handoff completeness checks align with canonical `SC-ROUTE-001`,
and proving typed seam non-regression across erosion-to-routing boundaries.

## Scope
### Included
- Implement canonical contract amendments required for Wave-3 routing-boundary
  coupling (`INV-SED-010` family and companion route/hydraulics guards).
- Implement contract-derived tests for boundary payload completeness,
  producer/consumer ownership, and typed failure semantics.
- Record pre-implementation contract gate evidence before production code
  edits.
- Implement production routing-boundary coupling behavior and typed handoff
  failures for missing/invalid payload surfaces.
- Publish explicit Wave-3 `GO`/`HOLD` verdict for EROD16 entry readiness.
- Complete dual review and dual verification artifacts for closure claims.

### Explicitly Out of Scope
- Tiered comparator/governance closeout (`EROD16`).
- New watershed physics-equivalence authoring beyond coupling integration.
- Non-erosion domains not required for routing boundary handoff closure.

## Deliverables
1. Contract implementation evidence:
   - `artifacts/erod15-contract-implementation-evidence.md`
2. Contract-test implementation evidence:
   - `artifacts/erod15-contract-test-implementation-evidence.md`
3. Pre-implementation contract gate:
   - `artifacts/erod15-preimplementation-contract-gate.md`
4. Implementation/test evidence:
   - `artifacts/erod15-implementation-and-test-evidence.md`
5. Kernel profile compliance checklist:
   - `artifacts/erod15-kernel-profile-compliance-checklist.md`
6. `INV-SED-010` payload export evidence:
   - `artifacts/erod15-inv-sed-010-payload-export-evidence.md`
7. Routing-boundary handoff map:
   - `artifacts/erod15-routing-boundary-handoff-map.md`
8. Typed seam non-regression evidence:
   - `artifacts/erod15-typed-seam-nonregression-evidence.md`
9. Runtime route-integration plan:
   - `artifacts/erod15-runtime-route-integration-plan.md`
10. Wave-3 entry verdict for EROD16:
    - `artifacts/erod15-wave3-go-no-go-verdict.md`
11. Package governance artifacts:
    - `artifacts/worker-handoff.md`
    - `artifacts/owned-file-manifest.md`
    - `artifacts/gate-results.md`
    - `artifacts/erod15_disposition.md`
12. Dual review artifacts:
    - `artifacts/review_agent_a.md`
    - `artifacts/review_agent_b.md`
13. Dual verification artifacts:
    - `artifacts/verification_agent_a.md`
    - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement canonical contract updates in `SC-*` files.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Only then implement production code edits.

Any sequencing violation keeps package disposition in `HOLD`.

## Autonomous Execution Intent (Required)
This package is execution-ready and self-contained. Assigned agents must
execute all phases through disposition and artifact updates without requesting
additional user direction unless hard-blocked by missing local authority,
unreadable dependencies, or contradictory canonical requirements.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label evidence mode using `Static:`
and/or `Ran:` sections. Claims without explicit evidence-mode labeling are
non-compliant.

## Physics Authority and Provenance Requirement
- Canonical physics/equation authority for migration/parity claims must live
  in `docs/specifications/science-contracts/contracts/SC-*.md`; package-local
  notes are evidence, not authority.
- Legacy provenance defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified.
- Do not invent physics: every equation/constant/guard/invariant must trace to
  canonical authority plus explicit provenance citations.
- Preserve canonical variable naming continuity; when runtime symbols differ,
  record explicit alias mappings in canonical contracts.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod10-sediment-kernelization-intake-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod10-sediment-kernelization-intake-001/artifacts/erod10-wave-execution-plan.md`
- `/workdir/openWEPP/docs/work-packages/20260525-erod14-multiofe-and-enrichment-kernel-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-erod14-multiofe-and-enrichment-kernel-001/artifacts/erod14-wave2-go-no-go-verdict.md`
- `/workdir/openWEPP/docs/work-packages/20260525-erod14-multiofe-and-enrichment-kernel-001/artifacts/erod14_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-erod14-multiofe-and-enrichment-kernel-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260523-ws10-channel-impoundment-production-kernels-001/artifacts/ws10_disposition.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/**`
- `crates/openwepp-watershed-orchestrator/**`
- `crates/openwepp-kernel-contract/**`
- `crates/openwepp-sim-contract/**`
- `tests/integration/**`
- `docs/work-packages/20260525-erod15-routing-boundary-coupling-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase A - Intake and Wave-3 Entry Confirmation
- Confirm queue authorization, Wave-2 completion, and dependency readability.
- Confirm Wave-3 coupling scope from EROD10 wave plan + EROD14 handoff.

### Phase B - Canonical Contract Amendments
- Implement required canonical `SC-*` Wave-3 coupling invariants and companion
  guard semantics for route-boundary payload handoff (`INV-SED-010`).

### Phase C - Contract Tests and Pre-Implementation Gate
- Implement contract-derived tests for payload completeness, seam ownership,
  and typed handoff failure semantics.
- Execute and record pre-implementation contract-gate evidence.

### Phase D - Production Wave-3 Runtime Implementation
- Implement production route-boundary payload export and coupling integration.
- Wire typed failures for missing/invalid route-boundary sediment payload
  surfaces.

### Phase E - Verification and Disposition
- Run required repository gates.
- Complete dual review + dual verification artifacts.
- Publish explicit Wave-3 EROD16 entry verdict (`GO`/`HOLD`).

## Exit Criteria
- Wave-3 scope (`INV-SED-010`) is implemented in canonical authority and
  production route-coupling behavior.
- Route-boundary payload handoff completeness checks are evidenced and passing.
- Cross-lane typed seam non-regression evidence is complete.
- No silent fallback/default branches exist in route-boundary coupling paths.
- Contract-first sequence evidence is complete:
  1. contract implementation,
  2. contract-test implementation,
  3. pre-implementation contract gate,
  4. production implementation evidence.
- Required repository gates are run and recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Dual review/disposition/verification artifacts are complete.
- Wave-3 verdict for EROD16 entry is explicit and authority-backed.

## Security Impact and Review Gate
- security_impact: medium
- dedicated_security_review_required: no
- Rationale: production kernel/runtime coupling updates are expected and gated
  through typed failure semantics plus contract-derived tests.
