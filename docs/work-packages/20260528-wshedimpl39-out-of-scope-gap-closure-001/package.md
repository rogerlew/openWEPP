# 20260528-wshedimpl39-out-of-scope-gap-closure-001

## Status
- state: complete
- date: 2026-05-28
- timezone: UTC
- decision: GO

## Objective
Execute WSHEDIMPL39 as the follow-on closure package for WSHEDIMPL38
out-of-scope blockers by implementing concrete workload-applicability input
validation for watershed routing, reconciling residual system-level
promotability gap language, and publishing disposition-grade evidence for
`GAP-ROUTE-005`, `GAP-SYSTEM-001`, and `GAP-SYSTEM-002`.

## Why This Package Exists
WSHEDIMPL38 completed channel-sediment parity closure but disposition remained
`HOLD` because three out-of-scope governance gaps remained non-promotable.
This package resolves those residual blockers with contract-first updates,
contract-derived tests, and runtime validator wiring.

## Scope
### Included
- Canonical contract/index amendments for:
  - `SC-ROUTE-001` (`GAP-ROUTE-005` applicability validator binding),
  - `SC-SYSTEM-001` (`GAP-SYSTEM-001`, `GAP-SYSTEM-002` system posture),
  - `SC-IMPOUND-001` only if required for cross-contract consistency,
  - `docs/specifications/science-contracts/index.md`.
- Watershed runfile contract surface updates in
  `docs/contracts/openwepp-watershed-runfile-contract.md` for required
  applicability selector semantics.
- Production watershed CLI runfile validator updates in
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` to enforce
  explicit applicability selector rules with typed fail-closed errors.
- Contract-derived watershed CLI tests in
  `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`.
- Required validation gate execution and artifacted disposition evidence.

### Explicitly Out of Scope
- New process-physics migration outside declared gap-closure scope.
- Heuristic/approximate kernel math substitutions.
- Broad refactors unrelated to runfile applicability and contract-gap closure.

## Deliverables
1. `artifacts/wshedimpl39-contract-implementation-evidence.md`
2. `artifacts/wshedimpl39-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl39-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl39-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl39-kernel-profile-compliance-checklist.md`
6. `artifacts/owned-file-manifest.md`
7. `artifacts/gate-results.md`
8. `artifacts/wshedimpl39_disposition.md`
9. `artifacts/worker-handoff.md`
10. `artifacts/review_agent_a.md`
11. `artifacts/review_agent_b.md`
12. `artifacts/verification_agent_a.md`
13. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for WSHEDIMPL39 scope.
2. Implement contract-derived tests.
3. Record pre-implementation contract-gate evidence.
4. Implement production runfile/runtime validator edits.

## Autonomous Execution Intent (Required)
This package executes end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/contracts/openwepp-watershed-runfile-contract.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl38-channel-sediment-symbol-burndown-hold-lift-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl38-channel-sediment-symbol-burndown-hold-lift-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl38-channel-sediment-symbol-burndown-hold-lift-closure-001/artifacts/wshedimpl38_disposition.md`

## Intended Write Set
- `docs/work-packages/20260528-wshedimpl39-out-of-scope-gap-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md` (if needed)
- `docs/specifications/science-contracts/index.md`
- `docs/contracts/openwepp-watershed-runfile-contract.md`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL39 authorization from WSHEDIMPL38 handoff/disposition.
- Freeze target blocker set (`GAP-ROUTE-005`, `GAP-SYSTEM-001`,
  `GAP-SYSTEM-002`).

### Phase B - Contract/test preparation
- Amend canonical route/system (and impound if required) contract language.
- Amend science-contract index summary language for WSHEDIMPL39.
- Add contract-derived watershed CLI validator tests.
- Record pre-implementation contract-gate evidence.

### Phase C - Runtime closure migration
- Implement watershed runfile applicability selector parsing/validation.
- Enforce typed fail-closed behavior for invalid applicability declarations.

### Phase D - Validation evidence run
- Execute required validation gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

### Phase E - Disposition and handoff
- Publish GO/HOLD disposition with explicit closure map for targeted gaps and
  any remaining blockers outside WSHEDIMPL39 scope.

## Exit Criteria
- `GAP-ROUTE-005` is dispositioned with concrete validator-surface authority
  and implementation evidence.
- `GAP-SYSTEM-001` and `GAP-SYSTEM-002` are dispositioned with explicit
  cross-contract/system evidence and promotability posture.
- Required gates pass and are truthfully recorded in artifacts.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local docs/runtime validator/test updates only; no network or
  credential surface changes.
