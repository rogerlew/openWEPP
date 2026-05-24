# 20260524-ws12-impoundment-physics-equivalence-port-001

## Status
- state: completed-with-hold
- date: 2026-05-24
- timezone: UTC

## Objective
Replace WS10 impoundment algebraic retention surrogate with legacy-equivalent
impoundment hydraulics authority and production runtime execution under typed
guards.

## Why This Package Exists
The PL08 hold-lift queue addendum authorizes
`WS12-impoundment-physics-equivalence-port` after WS11 to close remaining
watershed-physics surrogate posture for impoundment routing. WS10 established a
production path, but the impoundment behavior is still surrogate-based and not
physics-parity authority.

Execution may proceed in parallel with WS11 from a dedicated git worktree, but
merge-back into `main` remains gated on WS11 landing first and WS12 rebasing to
that updated `main`.

This package is kernel-affecting and contract-first:
1. implement canonical `SC-*` authority updates,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

## Scope
### Included
- Implement canonical impoundment physics authority updates in relevant science
  contracts before code edits.
- Implement contract-derived WS12 impoundment tests from canonical authority.
- Record pre-implementation contract-gate evidence before production
  impoundment code changes.
- Replace WS10 impoundment surrogate authority with legacy-equivalent
  impoundment hydraulics under typed guards.
- Produce impoundment vectors and parity traces that prove surrogate
  replacement.
- Preserve typed error/guard posture and prohibit silent defaults/clamping.

### Explicitly Out of Scope
- WS11 channel-routing physics-equivalence implementation.
- Erosion lane coupling execution (`EROD15+`).
- PL14S/PL15S Tier-A closeout decisions.

## Deliverables
1. WS12 contract authority implementation evidence:
   - `artifacts/ws12-contract-implementation-evidence.md`
2. WS12 impoundment authority and guard map:
   - `artifacts/ws12-impoundment-physics-authority-and-guard-map.md`
3. WS12 contract-derived test implementation evidence:
   - `artifacts/ws12-contract-test-implementation-evidence.md`
4. WS12 pre-implementation contract gate evidence:
   - `artifacts/ws12-preimplementation-contract-gate.md`
5. WS12 implementation and test evidence:
   - `artifacts/ws12-implementation-and-test-evidence.md`
6. WS12 impoundment vectors and parity traces:
   - `artifacts/ws12-impoundment-vectors-and-parity-traces.md`
7. WS12 kernel-profile compliance checklist:
   - `artifacts/ws12-kernel-profile-compliance-checklist.md`
8. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/ws12_disposition.md`
9. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
10. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement canonical `SC-*` contract authority amendments.
2. Implement contract-derived tests from canonical authority.
3. Record pre-implementation contract-gate evidence.
4. Only then modify production impoundment code.

Any sequencing violation keeps this package disposition in `HOLD`.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label evidence mode using `Static:` and/
or `Ran:` sections. Claims without explicit evidence labeling are
non-compliant.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15r-tier-a-delta-recloseout-and-hold-lift-rerun-001/artifacts/pl15r-pl08-hold-lift-decision-record.md`
- `/workdir/openWEPP/docs/work-packages/20260524-ws11-channel-routing-physics-equivalence-port-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-ws10-channel-impoundment-production-kernels-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-ws10-channel-impoundment-production-kernels-001/artifacts/ws10_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb16-peak-runoff-kernel-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb16-peak-runoff-kernel-001/artifacts/wb16_disposition.md`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/tests/integration/`
- `/workdir/wepp-forest_260430_baseline` @ `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

## Required Worktree Parallelization and Integration
- Parallel execution model: WS11 and WS12 run concurrently from separate git
  worktrees rooted from the same `main` base commit.
- WS12 worktree branch: `ws12-impoundment-physics-equivalence-port-001`.
- WS11 companion worktree branch:
  `ws11-channel-routing-physics-equivalence-port-001`.
- Record worktree path, branch, and starting `main` commit SHA in
  `artifacts/worker-handoff.md`.
- Integration order back to `main` is mandatory:
  1. merge WS11 to `main`,
  2. rebase WS12 onto updated `main`,
  3. rerun required gates in WS12 worktree after rebase,
  4. merge WS12 to `main`.
- Any merge/rebase conflict touching canonical `SC-*`, tests, or watershed
  runtime surfaces must be resolved with updated evidence artifacts before
  merge approval.

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `tests/integration/**`
- `docs/work-packages/20260524-ws12-impoundment-physics-equivalence-port-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase -1 - Worktree Bootstrap
- Create dedicated WS11/WS12 worktrees and branches from the same `main` base
  commit.
- Record base commit SHA and branch/worktree mapping in both package handoff
  artifacts.

### Phase 0 - Intake
- Confirm queue authority, WS10 surrogate baseline, and WS11 dependency
  posture.

### Phase 1 - Contract Authority
- Implement required canonical impoundment/hydraulics contract amendments.

### Phase 2 - Contract Tests + Pre-Implementation Gate
- Implement contract-derived impoundment tests and record pre-implementation
  gate evidence.

### Phase 3 - Production Impoundment Implementation
- Replace surrogate impoundment authority with legacy-equivalent
  production-path hydraulics under typed guards.

### Phase 4 - Verification
- Run targeted tests and required repository gates.

### Phase 5 - Disposition
- Publish evidence set and final WS12 disposition.

## Exit Criteria
- WS12 queue objective is evidence-backed.
- Impoundment routing production claims are no longer based on simple headroom
  ratio surrogate authority.
- WS12 execution occurred in its dedicated worktree branch and handoff evidence
  records branch/path/base commit.
- Canonical contract authority updates are implemented in `SC-*` files.
- Contract-derived tests are implemented and executed.
- Pre-implementation contract gate demonstrates contract/test completion before
  production code edits.
- Impoundment vectors and parity traces are produced.
- WS12 branch is rebased onto post-WS11 `main`, required gates are rerun on the
  rebased branch, and evidence is recorded before merge.
- Required gates executed if code is changed:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: medium
- dedicated_security_review_required: yes
- Rationale: production watershed impoundment kernel behavior and typed guard
  surface changes.
