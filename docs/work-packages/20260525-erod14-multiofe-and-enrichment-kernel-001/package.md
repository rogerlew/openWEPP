# 20260525-erod14-multiofe-and-enrichment-kernel-001

## Status
- state: completed
- date: 2026-05-25
- timezone: UTC

## Objective
Implement Wave-2 multi-OFE routing and enrichment kernel behavior in openWEPP
with contract-first sequencing, canonical `SC-*` authority, conservation-vector
validation, typed guard semantics, and no silent fallback/default transitions.

## Why This Package Exists
EROD10 defines `EROD14` as Wave-2 after Wave-1 (`EROD13`) completion. EROD13 is
now complete with a `GO` verdict, which authorizes entry into the multi-OFE and
enrichment scope required before Wave-3 routing-boundary coupling (`EROD15`).

This package executes the Wave-2 objective by extending erosion runtime
behavior from single-hillslope core closure outputs to multi-OFE transport and
enrichment semantics, with explicit class-fraction/class-mass conservation
checks and typed failures for invalid transitions.

## Scope
### Included
- Implement canonical contract amendments required for Wave-2 scope:
  `INV-SED-008..009` families and companion cross-domain guards.
- Implement contract-derived tests for OFE transition semantics,
  enrichment-state progression, and class-fraction/class-mass conservation.
- Record pre-implementation contract gate evidence before production kernel
  edits.
- Implement production runtime multi-OFE and enrichment behavior under typed
  error handling (no silent defaults/clamping).
- Publish explicit Wave-2 `GO`/`HOLD` verdict for EROD15 entry readiness.
- Complete dual review and dual verification artifacts for closure claims.

### Explicitly Out of Scope
- Routing-boundary payload export and downstream handoff (`EROD15`).
- Tiered comparator closeout governance (`EROD16`).
- Watershed channel/impoundment production-kernel scope beyond referenced
  dependencies.

## Deliverables
1. Contract implementation evidence:
   - `artifacts/erod14-contract-implementation-evidence.md`
2. Contract-test implementation evidence:
   - `artifacts/erod14-contract-test-implementation-evidence.md`
3. Pre-implementation contract gate:
   - `artifacts/erod14-preimplementation-contract-gate.md`
4. Implementation/test evidence:
   - `artifacts/erod14-implementation-and-test-evidence.md`
5. Kernel profile compliance checklist:
   - `artifacts/erod14-kernel-profile-compliance-checklist.md`
6. Wave-2 invariant coverage map:
   - `artifacts/erod14-wave2-invariant-coverage-map.md`
7. Multi-OFE transition and enrichment state map:
   - `artifacts/erod14-multiofe-transition-and-enrichment-map.md`
8. Conservation vector evidence:
   - `artifacts/erod14-conservation-vector-evidence.md`
9. Runtime phase integration plan:
   - `artifacts/erod14-runtime-phase-integration-plan.md`
10. Wave-2 entry verdict for EROD15:
    - `artifacts/erod14-wave2-go-no-go-verdict.md`
11. Package governance artifacts:
    - `artifacts/worker-handoff.md`
    - `artifacts/owned-file-manifest.md`
    - `artifacts/gate-results.md`
    - `artifacts/erod14_disposition.md`
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
4. Only then implement production kernel code edits.

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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod10-sediment-kernelization-intake-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod10-sediment-kernelization-intake-001/artifacts/erod10-wave-execution-plan.md`
- `/workdir/openWEPP/docs/work-packages/20260525-erod13-hillslope-core-erosion-kernel-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-erod13-hillslope-core-erosion-kernel-001/artifacts/erod13-wave1-go-no-go-verdict.md`
- `/workdir/openWEPP/docs/work-packages/20260525-erod13-hillslope-core-erosion-kernel-001/artifacts/erod13_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-erod13-hillslope-core-erosion-kernel-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260523-ws10-channel-impoundment-production-kernels-001/artifacts/ws10_disposition.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/**`
- `crates/openwepp-kernel-contract/**`
- `crates/openwepp-sim-contract/**`
- `tests/integration/**`
- `docs/work-packages/20260525-erod14-multiofe-and-enrichment-kernel-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase A - Intake and Wave-2 Entry Confirmation
- Confirm queue authorization, Wave-1 completion, and dependency readability.
- Confirm Wave-2 scope boundaries from EROD10 wave plan + EROD13 handoff.

### Phase B - Canonical Contract Amendments
- Implement required canonical `SC-*` Wave-2 invariant/guard amendments for
  multi-OFE and enrichment behavior (`INV-SED-008..009`).

### Phase C - Contract Tests and Pre-Implementation Gate
- Implement contract-derived tests for conservation vectors and transition
  semantics.
- Execute and record pre-implementation contract-gate evidence.

### Phase D - Production Wave-2 Runtime Implementation
- Implement production runtime OFE routing + enrichment behavior and typed
  guard failures.
- Preserve EROD13 guard posture and prohibit silent fallback/default paths.

### Phase E - Verification and Disposition
- Run required repository gates.
- Complete dual review + dual verification artifacts.
- Publish explicit Wave-2 EROD15 entry verdict (`GO`/`HOLD`).

## Exit Criteria
- Wave-2 scope (`INV-SED-008..009`) is implemented in canonical authority and
  production runtime behavior.
- Class-fraction and class-mass conservation vectors pass and are evidenced.
- No silent fallback/default branches remain in OFE transition paths.
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
- Wave-2 verdict for EROD15 entry is explicit and authority-backed.

## Security Impact and Review Gate
- security_impact: medium
- dedicated_security_review_required: no
- Rationale: production kernel/runtime logic updates are expected and validated
  through typed-guard/error and contract-derived test gates.
