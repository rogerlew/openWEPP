# 20260525-simimpl15-replay-comparator-tooling-alignment-001

## Status
- state: completed
- date: 2026-05-25
- timezone: UTC

## Objective
Close replay comparator/tooling alignment gaps after SIMIMPL14 by reconciling
strict/semantic lane policy behavior, parquet alias mapping drift, diagnostic
surface consistency, and candidate-source provenance classification for
promotable parity evidence.

## Why This Package Exists
SIMIMPL13 identified comparator/tooling residuals (`SIMIMPL13-TOOL-001..004`,
`SIMIMPL13-COMP-003`, `SIMIMPL13-COMP-005`) that block promotable parity
claims even when candidate execution/publication span is corrected.

SIMIMPL14 is wrapping up and is expected to close `GO` (green) for downstream
dependency usage.
SIMIMPL15 is the next queued closure wave and focuses exclusively on
comparison-suite and replay-tooling alignment so SIMIMPL16/SIMIMPL17 can rely
on deterministic comparator signals.

## Scope
### Included
- Implement canonical contract/authority amendments required for comparator
  lane policy and investigation-column alignment closure.
- Implement contract-derived tests for strict/semantic lane behavior,
  parquet-alias continuity, and provenance-source classification.
- Record pre-implementation contract gate evidence before production tooling
  edits.
- Implement replay tooling alignment in legacy comparison suite scripts.
- Implement provenance tagging/validation that distinguishes native candidate
  dat emission from conversion-derived dat surrogates.
- Produce explicit SIMIMPL15 `GO`/`HOLD` verdict for SIMIMPL16/17 entry.

### Explicitly Out of Scope
- Runner multi-day execution/publication implementation (SIMIMPL14 scope).
- Full replay closeout rerun and hold-lift disposition (SIMIMPL17 scope).
- Watershed comparator alignment beyond hillslope replay surfaces.

## Deliverables
1. Contract implementation evidence:
   - `artifacts/simimpl15-contract-implementation-evidence.md`
2. Contract-test implementation evidence:
   - `artifacts/simimpl15-contract-test-implementation-evidence.md`
3. Pre-implementation contract gate:
   - `artifacts/simimpl15-preimplementation-contract-gate.md`
4. Implementation/test evidence:
   - `artifacts/simimpl15-implementation-and-test-evidence.md`
5. Kernel-profile compliance checklist:
   - `artifacts/simimpl15-kernel-profile-compliance-checklist.md`
6. Comparator lane policy closure map:
   - `artifacts/simimpl15-comparator-lane-policy-closure-map.md`
7. Parquet alias alignment evidence:
   - `artifacts/simimpl15-parquet-alias-alignment-evidence.md`
8. Dat-vs-parquet strict equivalence policy evidence:
   - `artifacts/simimpl15-dat-vs-parquet-strict-equivalence-policy.md`
9. Candidate-source provenance closure map:
   - `artifacts/simimpl15-candidate-source-provenance-closure-map.md`
10. SIMIMPL15 entry verdict for downstream waves:
    - `artifacts/simimpl15-go-no-go-verdict.md`
11. Governance artifacts:
    - `artifacts/worker-handoff.md`
    - `artifacts/owned-file-manifest.md`
    - `artifacts/gate-results.md`
    - `artifacts/simimpl15_disposition.md`
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
4. Only then implement production tooling code edits.

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
- Canonical authority for replay closure semantics remains in
  `docs/specifications/science-contracts/contracts/SC-*.md` and companion
  subsystem contracts/specifications.
- Legacy baseline provenance defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified.
- Do not invent comparison semantics: lane-policy and alias/provenance
  decisions must trace to canonical contract text plus explicit evidence.

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
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl11-tier-a-semantic-replay-recloseout-and-residual-classification-001/artifacts/simimpl11_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/simimpl13-comparator-tooling-gap-audit.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/simimpl13-candidate-surface-comparability-gap-register.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/simimpl13-contract-test-blind-spot-assessment.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/replay-implementation-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/simimpl13_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl14-runner-wb13-timeseries-span-and-row-key-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl14-runner-wb13-timeseries-span-and-row-key-closure-001/artifacts/simimpl14-go-no-go-verdict.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl14-runner-wb13-timeseries-span-and-row-key-closure-001/artifacts/simimpl14_disposition.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
- `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `tests/integration/pl14_tier_a_candidate_replay_contract.rs`
- `tests/integration/pl14r_tier_a_replay_rerun_contract.rs`
- `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
- `tests/integration/comparator_tier_routing_metadata.rs`
- `docs/work-packages/20260525-simimpl15-replay-comparator-tooling-alignment-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase A - Intake and Entry Confirmation
- Confirm SIMIMPL15 queue authorization and prerequisite closure state
  (`SIMIMPL14` complete/GO).
- Confirm dependency readability and evidence baseline.

### Phase B - Canonical Contract Amendments
- Implement required canonical `SC-*` amendments for comparator lane policy,
  alias continuity, and provenance classification semantics.
- Update contract index cross-links when authority surfaces change.

### Phase C - Contract-Derived Tests and Pre-Implementation Gate
- Implement contract-derived tests that fail when tooling gaps
  `SIMIMPL13-TOOL-001..004` / `SIMIMPL13-COMP-003` / `SIMIMPL13-COMP-005`
  regress.
- Execute and record pre-implementation contract gate evidence.

### Phase D - Tooling Alignment Implementation
- Implement comparison-suite lane policy and parquet alias closure logic.
- Implement provenance tagging/validation for native-vs-conversion candidate
  surface classification.
- Align report diagnostics required for cross-format comparability evidence.

### Phase E - Verification and Disposition
- Run required repository gates.
- Complete dual review + dual verification artifacts.
- Publish SIMIMPL15 `GO`/`HOLD` verdict and final disposition.

## Exit Criteria
- `SIMIMPL13-TOOL-001` closure: strict/parquet lane policy is explicit,
  deterministic, and test-enforced.
- `SIMIMPL13-TOOL-002` and `SIMIMPL13-COMP-005` closure: parquet alias mapping
  correctly resolves required investigation columns (including `Total-Soil`)
  with no false missing-field drift.
- `SIMIMPL13-TOOL-003` closure: semantic report diagnostics for parquet are
  policy-explicit and regression-tested for cross-format comparability claims.
- `SIMIMPL13-TOOL-004` closure: provenance artifacts distinguish conversion-
  derived dat surfaces from native runtime dat emission and gate promotable
  claims accordingly.
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
- security_impact: low
- dedicated_security_review_required: no
- Rationale: comparator/tooling/test surfaces; no direct kernel-physics runtime
  mutation is required for this scope.
