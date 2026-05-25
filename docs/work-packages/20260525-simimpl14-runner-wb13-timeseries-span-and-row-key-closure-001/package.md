# 20260525-simimpl14-runner-wb13-timeseries-span-and-row-key-closure-001

## Status
- state: completed
- date: 2026-05-25
- timezone: UTC

## Objective
Implement continuous hillslope simulation execution in the runner by advancing
climate day forcing across the full run span, carrying runtime state through
scheduler/kernel lifecycles, publishing replay-length WB13/H.wat trajectories,
and aligning candidate row-key semantics for promotable replay overlap.

## Why This Package Exists
SIMIMPL13 closed as `HOLD` with explicit replay/parity residuals and a
follow-on queue that marks SIMIMPL14 as the first implementation package.
SIMIMPL13 addendum findings (`SIMIMPL13-CONT-001..007`) confirm that current
runner behavior is single-day and projection-shaped, not a true continuous
simulation trajectory.

This package executes the first closure wave by implementing continuous
execution and publication behavior in the runner path so downstream comparator
alignment and hold-lift reruns can operate on meaningful multi-day candidate
surfaces.

## Scope
### Included
- Implement canonical contract amendments required for continuous-run and
  replay-span/key closure in canonical `SC-*` authority surfaces.
- Implement contract-derived tests that fail on one-day collapse and key-domain
  mismatch regressions relevant to SIMIMPL14 scope.
- Record pre-implementation contract gate evidence before production code edits.
- Implement production runner continuous day-index progression and lifecycle
  execution continuity.
- Implement replay-length WB13/H.wat publication continuity, including monotonic
  day indexing and row-key policy alignment.
- Implement continuity assertions/evidence for auxiliary outputs and manifest
  continuity metadata required for regression detection.
- Produce explicit SIMIMPL14 `GO`/`HOLD` verdict for SIMIMPL15/SIMIMPL16 entry.

### Explicitly Out of Scope
- Comparator tooling/schema alias policy closure (`SIMIMPL15` scope).
- Broad replay governance/test-matrix closeout beyond SIMIMPL14 implementation
  surfaces (`SIMIMPL16`/`SIMIMPL17` scope).
- Watershed replay/parity closure outside hillslope runner path.

## Deliverables
1. Contract implementation evidence:
   - `artifacts/simimpl14-contract-implementation-evidence.md`
2. Contract-test implementation evidence:
   - `artifacts/simimpl14-contract-test-implementation-evidence.md`
3. Pre-implementation contract gate:
   - `artifacts/simimpl14-preimplementation-contract-gate.md`
4. Implementation/test evidence:
   - `artifacts/simimpl14-implementation-and-test-evidence.md`
5. Kernel-profile compliance checklist:
   - `artifacts/simimpl14-kernel-profile-compliance-checklist.md`
6. Continuous execution driver design and evidence:
   - `artifacts/simimpl14-continuous-execution-driver-design-and-evidence.md`
7. WB13 timeseries publication closure map:
   - `artifacts/simimpl14-wb13-timeseries-publication-closure-map.md`
8. Row-key semantics alignment map:
   - `artifacts/simimpl14-row-key-semantics-alignment-map.md`
9. Auxiliary output and manifest continuity closure map:
   - `artifacts/simimpl14-aux-output-and-manifest-continuity-closure-map.md`
10. SIMIMPL14 entry verdict for follow-on waves:
    - `artifacts/simimpl14-go-no-go-verdict.md`
11. Governance artifacts:
    - `artifacts/worker-handoff.md`
    - `artifacts/owned-file-manifest.md`
    - `artifacts/gate-results.md`
    - `artifacts/simimpl14_disposition.md`
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
4. Only then implement production runner/runtime code edits.

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

## Physics and Provenance Posture
- Canonical authority for process invariants and closure rules remains in
  `docs/specifications/science-contracts/contracts/SC-*.md`; package-local
  artifacts are evidence, not authority replacement.
- Legacy baseline provenance defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified.
- Do not invent behavior semantics: continuity/key policy decisions must trace
  to canonical contract text plus explicit provenance rationale.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl11-tier-a-semantic-replay-recloseout-and-residual-classification-001/artifacts/simimpl11_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/simimpl13-pipeline-timeseries-span-audit.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/simimpl13-candidate-surface-comparability-gap-register.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/simimpl13-continuous-simulation-run-gap-assessment.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/simimpl13-replay-parity-full-closure-criteria.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/replay-implementation-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/simimpl13_disposition.md`
- `/workdir/wepp-forest_260430_baseline`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-runner/tests/**`
- `tests/integration/**`
- `docs/work-packages/20260525-simimpl14-runner-wb13-timeseries-span-and-row-key-closure-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase A - Intake and Entry Confirmation
- Confirm SIMIMPL14 queue authorization and prerequisite artifact readability.
- Confirm baseline contract/decision authority readiness.

### Phase B - Canonical Contract Amendments
- Implement required canonical `SC-*` amendments for continuity, span, and
  candidate key policy semantics.
- Update contract index cross-links when authority surfaces change.

### Phase C - Contract-Derived Tests and Pre-Implementation Gate
- Implement contract-derived tests covering continuity progression and
  publication/key invariants for SIMIMPL14 scope.
- Execute and record pre-implementation contract-gate evidence.

### Phase D - Production Runner/Publication Implementation
- Implement continuous day-indexed runner execution and carried state
  lifecycle behavior.
- Implement replay-length WB13/H.wat publication continuity and key alignment.
- Implement auxiliary output/manifest continuity assertions required for
  regression detection.

### Phase E - Verification and Disposition
- Run required repository gates.
- Complete dual review + dual verification artifacts.
- Publish SIMIMPL14 `GO`/`HOLD` verdict and final disposition.

## Exit Criteria
- Continuous forcing progression and lifecycle continuity are implemented and
  evidenced (`SIMIMPL13-CONT-001`, `SIMIMPL13-CONT-002`).
- Kernel/runtime path is no longer no-op for SIMIMPL14 execution lane
  acceptance claims (`SIMIMPL13-CONT-003`).
- WB13/H.wat publication emits replay-length trajectories with monotonic day
  indexing and continuity checks (`SIMIMPL13-CONT-004`).
- Candidate key policy is explicitly implemented and validated for comparator
  overlap readiness (`SIMIMPL13-CONT-005`).
- Auxiliary outputs and manifests are continuity-truthful or explicitly gated
  with typed constraints (`SIMIMPL13-CONT-006`, `SIMIMPL13-CONT-007`).
- Required repository gates are run and recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Contract-first sequence evidence is complete:
  1. contract implementation,
  2. contract-test implementation,
  3. pre-implementation contract gate,
  4. production implementation evidence.
- Dual review/disposition/verification artifacts are complete.

## Security Impact and Review Gate
- security_impact: medium
- dedicated_security_review_required: no
- Rationale: production runner/runtime code-path edits are expected; scope is
  local execution/publication behavior with typed-guard enforcement.
