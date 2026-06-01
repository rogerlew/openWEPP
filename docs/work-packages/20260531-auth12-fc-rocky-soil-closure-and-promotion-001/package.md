# 20260531-auth12-fc-rocky-soil-closure-and-promotion-001

## Status
- state: complete
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: GO

## Objective
Close the Level-4 direct-theta rocky-soil FC discrepancy by implementing
baseline-authoritative FC physics closure and promoting the cohort suite from
`periodic`/`investigation` to `required`/`hard-fail` only after red/fix/green
evidence is complete.

## Why This Package Exists
AUTH11 established anti-evasion guards and restored missing anchor fixtures, but
the FC process-physics discrepancy remains unresolved in production kernel code.
AUTH12 is the required closure package linked by AUTH11 obligations so
non-blocking posture cannot persist without an explicit queued remediation path.

## Autonomous Execution Intent
This package is execution-ready and self-contained. Execute all phases through
disposition without additional user direction unless hard-blocked.

## Scope
### Included
- Contract-first FC rocky-soil closure across canonical `SC-*` authority.
- Contract-derived tests for anchor-case threshold transition (`exceeds` ->
  `within`) and promotion readiness.
- Production kernel/runtime updates needed to satisfy direct-theta FC authority
  on rocky-soil anchors.
- Post-fix reruns and disposition evidence for posture promotion.

### Explicitly Out of Scope
- Unrelated process-family migrations.
- CI ownership/branch-policy controls.
- Non-FC suite posture changes outside direct-theta cohort scope.

## Deliverables
1. `artifacts/contract-implementation-evidence.md`
2. `artifacts/contract-test-implementation-evidence.md`
3. `artifacts/preimplementation-contract-gate.md`
4. `artifacts/implementation-and-test-evidence.md`
5. `artifacts/kernel-profile-compliance-checklist.md`
6. `artifacts/owned-file-manifest.md`
7. `artifacts/gate-results.md`
8. `artifacts/disposition.md`
9. `artifacts/worker-handoff.md`
10. `artifacts/review_agent_a.md`
11. `artifacts/review_agent_b.md`
12. `artifacts/verification_agent_a.md`
13. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence
1. Amend canonical contracts for FC closure authority.
2. Implement contract-derived tests and anchor-transition assertions.
3. Record pre-implementation contract-gate evidence.
4. Implement production kernel/runtime changes.
5. Run workspace gates and external-authority promotion checks.
6. Publish disposition and posture-promotion decision.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/external-authority/README.md`
- `/workdir/openWEPP/docs/specifications/external-authority/suite-schema.md`
- `/workdir/openWEPP/docs/specifications/external-authority/promotion-protocol.md`
- `/workdir/openWEPP/docs/specifications/external-authority/required-suite-obligations.json`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth11-required-suite-obligation-and-antievasion-guards-001/artifacts/claude-code-auth11-review.md`
- `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/required-suite-obligations.json`
- `docs/specifications/external-authority/suites/cas_l4_soil_fc_direct_theta_minus33_cohort_001.md`
- `tests/integration/auth07_fc_authority_cohort_contract.rs`
- `tests/integration/auth11_required_suite_obligation_guards_contract.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp_hillslope_orchestrator/**`
- `docs/work-packages/20260531-auth12-fc-rocky-soil-closure-and-promotion-001/**`

## Phase Plan
1. **Phase A - Contract authority amendment**
   - Add/ratify FC rocky-soil closure authority and promotion criteria.
2. **Phase B - Contract-derived tests**
   - Add failing tests for anchor transition (`valid_9002`, real rocky H1).
3. **Phase C - Pre-implementation gate**
   - Capture red-state evidence with truthful labels.
4. **Phase D - Implementation**
   - Apply kernel/runtime FC corrections with typed guards.
5. **Phase E - Validation and promotion**
   - Run gates, confirm green-state transition, update suite posture.
6. **Phase F - Disposition**
   - Publish GO/HOLD with explicit residuals and follow-on ownership.

## Exit Criteria
- Direct-theta rocky-soil anchors move from `exceeds` to `within` under the
  declared threshold.
- FC authority tests pass without fixture/cardinality/threshold relaxation.
- Level-4 direct-theta suite posture is promoted to `required`/`hard-fail` only
  after red/fix/green evidence completion.
- Workspace gates pass (`fmt`, `clippy`, `test`, `deny`) with truthful evidence.

## Truthfulness Labeling Requirement
All artifacts must explicitly label evidence as `Static:` and/or `Ran:`.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: kernel/process-authority changes only; no credential/network
  surface changes.
