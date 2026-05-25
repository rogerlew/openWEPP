# 20260525-simimpl16-replay-contract-derived-test-coverage-closure-001

## Status
- state: completed
- date: 2026-05-25
- timezone: UTC

## Objective
Close replay/parity contract-derived test blind spots by implementing explicit
coverage for trajectory span overlap, row-key semantic alignment,
parquet-alias continuity, strict-lane governance compensation, and
native-vs-conversion candidate provenance safeguards.

## Why This Package Exists
SIMIMPL13 classified five test blind spots (`SIMIMPL13-TEST-001..005`) that
permit replay/parity blockers to persist without deterministic automated gate
failures. SIMIMPL14 and SIMIMPL15 have completed with downstream `GO` verdicts,
so SIMIMPL16 is now the required test-closure wave before SIMIMPL17 hold-lift
reruns.

This package converts residual closure requirements into enforceable,
contract-derived tests that fail on known regression modes and pass only when
runner/comparator behavior satisfies replay closure semantics.

## Scope
### Included
- Implement canonical contract amendments required for test-gate semantics
  covering replay span/key/tooling/provenance closure assertions.
- Implement contract-derived tests that close `SIMIMPL13-TEST-001..005`.
- Record pre-implementation contract gate evidence before test/runtime harness
  edits.
- Implement test harness/provenance assertions needed to distinguish native
  candidate dat emission from conversion-derived surrogate surfaces.
- Produce explicit SIMIMPL16 `GO`/`HOLD` verdict for SIMIMPL17 entry.

### Explicitly Out of Scope
- New runner continuous execution/publication implementation (SIMIMPL14 scope).
- New comparator production behavior beyond alignment already closed in
  SIMIMPL15.
- Final replay rerun + hold-lift disposition (`SIMIMPL17` scope).

## Deliverables
1. Contract implementation evidence:
   - `artifacts/simimpl16-contract-implementation-evidence.md`
2. Contract-test implementation evidence:
   - `artifacts/simimpl16-contract-test-implementation-evidence.md`
3. Pre-implementation contract gate:
   - `artifacts/simimpl16-preimplementation-contract-gate.md`
4. Implementation/test evidence:
   - `artifacts/simimpl16-implementation-and-test-evidence.md`
5. Kernel-profile compliance checklist:
   - `artifacts/simimpl16-kernel-profile-compliance-checklist.md`
6. Blind-spot closure traceability map:
   - `artifacts/simimpl16-test-blind-spot-closure-traceability-map.md`
7. Span/key overlap test matrix evidence:
   - `artifacts/simimpl16-span-key-overlap-test-matrix-evidence.md`
8. Alias/provenance test coverage evidence:
   - `artifacts/simimpl16-alias-and-provenance-test-coverage-evidence.md`
9. Strict-lane governance compensation test evidence:
   - `artifacts/simimpl16-strict-lane-governance-compensation-test-evidence.md`
10. SIMIMPL16 entry verdict for downstream wave:
    - `artifacts/simimpl16-go-no-go-verdict.md`
11. Governance artifacts:
    - `artifacts/worker-handoff.md`
    - `artifacts/owned-file-manifest.md`
    - `artifacts/gate-results.md`
    - `artifacts/simimpl16_disposition.md`
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
4. Only then implement production/runtime harness edits if still required for
   test execution closure.

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

## Provenance and Authority Posture
- Canonical replay closure authority remains in
  `docs/specifications/science-contracts/contracts/SC-*.md` and companion
  runner/comparator contract docs.
- Legacy baseline provenance defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified.
- Do not invent closure criteria: all test assertions must trace to canonical
  contract invariants and SIMIMPL13 residual definitions.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md`
- `/workdir/openWEPP/docs/contracts/openwepp-hillslope-runfile-contract.md`
- `/workdir/openWEPP/docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/simimpl13-contract-test-blind-spot-assessment.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/simimpl13-replay-parity-full-closure-criteria.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/replay-implementation-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/simimpl13_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl14-runner-wb13-timeseries-span-and-row-key-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl14-runner-wb13-timeseries-span-and-row-key-closure-001/artifacts/simimpl14-go-no-go-verdict.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl14-runner-wb13-timeseries-span-and-row-key-closure-001/artifacts/simimpl14_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl15-replay-comparator-tooling-alignment-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl15-replay-comparator-tooling-alignment-001/artifacts/simimpl15-go-no-go-verdict.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl15-replay-comparator-tooling-alignment-001/artifacts/simimpl15_disposition.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/pl14_tier_a_candidate_replay_contract.rs`
- `tests/integration/pl14r_tier_a_replay_rerun_contract.rs`
- `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
- `tests/integration/comparator_tier_routing_metadata.rs`
- `tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`
- `crates/openwepp-runner/tests/simimpl04_runner_kernel_execution_contract.rs`
- `crates/openwepp-runner/tests/simimpl04_wb13_publication_contract.rs`
- `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
- `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `docs/work-packages/20260525-simimpl16-replay-contract-derived-test-coverage-closure-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase A - Intake and Entry Confirmation
- Confirm SIMIMPL16 queue authorization and prerequisite completion
  (`SIMIMPL14` and `SIMIMPL15` `GO`).
- Confirm dependency readability and baseline evidence readiness.

### Phase B - Canonical Contract Amendments
- Implement required canonical `SC-*` amendments for test-gate semantics across
  span/key/alias/provenance closure assertions.
- Update contract index cross-links when authority surfaces change.

### Phase C - Contract-Derived Tests and Pre-Implementation Gate
- Implement contract-derived tests for `SIMIMPL13-TEST-001..005` closure.
- Execute and record pre-implementation contract gate evidence.

### Phase D - Test Harness Closure Implementation
- Implement/adjust test harness and provenance assertions required to execute
  and enforce the new closure tests.
- Verify failure-before / pass-after behavior for targeted residual modes.

### Phase E - Verification and Disposition
- Run required repository gates.
- Complete dual review + dual verification artifacts.
- Publish SIMIMPL16 `GO`/`HOLD` verdict and final disposition.

## Exit Criteria
- `SIMIMPL13-TEST-001` closure: tests assert replay candidate span overlap
  against baseline semantics and fail on one-row collapse.
- `SIMIMPL13-TEST-002` closure: tests enforce candidate row-key policy
  alignment before comparator promotion assertions.
- `SIMIMPL13-TEST-003` closure: parquet alias continuity tests guard required
  investigation-column mappings (`Total-Soil` lineage).
- `SIMIMPL13-TEST-004` closure: strict-lane skip governance compensation is
  test-enforced for promotable replay claims.
- `SIMIMPL13-TEST-005` closure: provenance tests distinguish native dat emission
  from conversion-derived surrogate surfaces and gate claims accordingly.
- Required repository gates are run and recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Contract-first sequence evidence is complete:
  1. contract implementation,
  2. contract-test implementation,
  3. pre-implementation contract gate,
  4. implementation/test evidence.
- Dual review/disposition/verification artifacts are complete.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: test/comparator/harness governance scope; no new external
  connectivity or privileged runtime surface required.
