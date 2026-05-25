# 20260525-erod13-hillslope-core-erosion-kernel-001

## Status
- state: completed
- date: 2026-05-25
- timezone: UTC

## Objective
Implement Wave-1 core hillslope erosion kernel behavior in openWEPP with
contract-first sequencing, canonical science-contract authority, typed
error/guard semantics, and no silent fallback behavior.

## Why This Package Exists
EROD10 established Wave-1 (`EROD13`) as the first production erosion-kernel
implementation stage after Wave-0 governance closure. EROD11 and EROD12 are
completed, and the PL08 queue reassessment marks
`EROD13-hillslope-core-erosion-kernel-001` as `NEXT` with prerequisite
packages satisfied (`EROD12`, `WB14`, `WB15`, `WB16`).

This package executes the authorized Wave-1 scope by implementing core
hillslope erosion kernel behavior aligned to canonical `SC-*` authority and
producing implementation evidence suitable for Wave-2 (`EROD14`) entry.

## Scope
### Included
- Implement canonical contract amendments required for Wave-1 core hillslope
  erosion behavior (`INV-SED-001..007` family coverage and companion guards).
- Implement contract-derived tests for Wave-1 invariants and typed failure
  semantics.
- Record pre-implementation contract-gate evidence before production kernel
  edits.
- Implement production hillslope erosion kernel path and typed guard/error
  propagation for Wave-1 scope.
- Produce explicit Wave-1 `GO`/`HOLD` verdict artifact for EROD14 entry
  readiness.
- Complete dual review and dual verification artifacts for closure claims.

### Explicitly Out of Scope
- Multi-OFE enrichment physics (`EROD14`).
- Routing-boundary sediment coupling (`EROD15`).
- Erosion comparator/closeout package work (`EROD16`).
- Watershed production kernel scope outside Wave-1 hillslope erosion objective.

## Deliverables
1. Contract implementation evidence:
   - `artifacts/erod13-contract-implementation-evidence.md`
2. Contract-test implementation evidence:
   - `artifacts/erod13-contract-test-implementation-evidence.md`
3. Pre-implementation contract gate:
   - `artifacts/erod13-preimplementation-contract-gate.md`
4. Implementation/test evidence:
   - `artifacts/erod13-implementation-and-test-evidence.md`
5. Kernel profile compliance checklist:
   - `artifacts/erod13-kernel-profile-compliance-checklist.md`
6. Core kernel invariant coverage map:
   - `artifacts/erod13-core-kernel-invariant-coverage-map.md`
7. Typed error/guard surface map:
   - `artifacts/erod13-typed-error-guard-surface-map.md`
8. Runtime phase integration plan and execution evidence:
   - `artifacts/erod13-runtime-phase-integration-plan.md`
9. Wave-1 entry verdict for EROD14:
   - `artifacts/erod13-wave1-go-no-go-verdict.md`
10. Package governance artifacts:
    - `artifacts/worker-handoff.md`
    - `artifacts/owned-file-manifest.md`
    - `artifacts/gate-results.md`
    - `artifacts/erod13_disposition.md`
11. Dual review artifacts:
    - `artifacts/review_agent_a.md`
    - `artifacts/review_agent_b.md`
12. Dual verification artifacts:
    - `artifacts/verification_agent_a.md`
    - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement canonical contract updates in `SC-*` files.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Only then implement production kernel code edits.

Any sequencing violation keeps package disposition in `HOLD`.

## Autonomous Execution Intent (Required)
This package must remain self-contained and executable end-to-end. Assigned
agents must progress through all declared phases and update artifacts through
final disposition without requesting additional user direction unless
hard-blocked by missing local authority, unreadable dependencies, or
contradictory canonical requirements.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label evidence mode using `Static:`
and/or `Ran:` sections. Claims without explicit evidence-mode labeling are
non-compliant.

## Physics Authority and Provenance Requirement
- Canonical physics/equation authority for migration/parity claims must live
  in `docs/specifications/science-contracts/contracts/SC-*.md`; package-local
  notes are not authority.
- Legacy provenance defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified.
- Do not invent physics: every equation/constant/guard/invariant must trace to
  canonical authority plus explicit provenance citations.
- Preserve legacy WEPP variable naming continuity in canonical tables; when
  runtime names differ, record explicit alias mappings.

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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod10-sediment-kernelization-intake-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod10-sediment-kernelization-intake-001/artifacts/erod10-wave-execution-plan.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod11-alias-and-boundary-ownership-closure-001/artifacts/erod11-wave0-gate-verdict.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod12-cross-domain-contract-closure-001/artifacts/erod12-wave0-release-verdict.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod12-cross-domain-contract-closure-001/artifacts/erod12_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001/artifacts/wb14_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb15-canopy-interception-kernel-coupling-001/artifacts/wb15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb16-peak-runoff-kernel-001/artifacts/wb16_disposition.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/**`
- `crates/openwepp-kernel-contract/**`
- `crates/openwepp-sim-contract/**`
- `tests/integration/**`
- `docs/work-packages/20260525-erod13-hillslope-core-erosion-kernel-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase A - Intake and Wave-1 Entry Confirmation
- Confirm queue authorization and prerequisite package closure state.
- Confirm dependency readability and authority baseline.

### Phase B - Canonical Contract Amendments
- Implement required canonical `SC-*` Wave-1 erosion invariants and companion
  guard semantics.
- Update alias mappings where runtime names diverge.

### Phase C - Contract-Derived Tests and Pre-Implementation Gate
- Implement contract-derived tests for Wave-1 invariant/guard coverage.
- Execute and record pre-implementation contract-gate evidence.

### Phase D - Production Kernel Implementation
- Implement Wave-1 production hillslope erosion kernel behavior and typed error
  propagation.
- Wire required runtime integration path for erosion-phase execution.

### Phase E - Verification and Disposition
- Run required repository gates for touched code paths.
- Complete dual review + dual verification artifacts.
- Publish Wave-1 EROD14 entry verdict and final package disposition.

## Exit Criteria
- Wave-1 core erosion scope (`INV-SED-001..007`) is implemented in canonical
  contract authority and production runtime path.
- Typed guard/error families are explicit; no silent default/clamp fallback is
  introduced for domain violations.
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
- Wave-1 verdict for EROD14 entry is explicit (`GO`/`HOLD`) and authority
  backed.

## Security Impact and Review Gate
- security_impact: medium
- dedicated_security_review_required: no
- Rationale: production kernel/runtime code edits are expected, but the scope
  remains local model logic and typed-guard enforcement.
