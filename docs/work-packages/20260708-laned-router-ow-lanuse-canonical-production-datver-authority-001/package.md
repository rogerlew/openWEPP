# Lane D `ow-lanuse-1` Canonical Production Datver Authority

Status: `EXECUTED-COMPLETE-AUTHORITY`
Package ID:
`20260708-laned-router-ow-lanuse-canonical-production-datver-authority-001`
Owner: Codex
Execution date: `2026-07-08`
Evidence mode: `Static contract/spec authority; no Rust implementation executed`

## Objective

Lock in the post-M-T2P decision that openWEPP will not project Lane D routing
coefficients from legacy cropland fields. New openWEPP production physics uses
native `ow-lanuse-1` or later management datvers with explicit embedded
`routing_coefficients`; earlier datvers remain deprecated compatibility,
validation, rollback, and regression-diagnosis inputs on legacy/off paths.

## Rationale

M-T2P rejected coefficient projection and optional sidecar authority. The next
implementation work needs a contract-backed boundary before code changes:

- `ow-lanuse-1` and later native datvers are canonical for new production
  physics;
- Lane D active/default production requires complete embedded route coefficients
  for every scheduled native lane;
- all-legacy scheduled runs stay on the deprecated compatibility legacy/off
  path;
- native missing coefficients, mixed native/legacy datvers, optional sidecars,
  and mixed coefficient authority fail closed;
- legacy datvers stay available for validation/reference/rollback instead of
  being removed.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/ROADMAP.md` `## Watershed Runtime Performance Queue`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `docs/work-packages/20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001/package.md`
- `docs/work-packages/20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001/artifacts/final-disposition.md`
- `docs/work-packages/20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001/artifacts/worker-handoff.md`
- this package's `artifacts/required-reading-map.md`

Conditional:

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md` if lifecycle metadata changes.

## Scope

### Included

- Scaffold this work package and update package/catalog roadmap pointers.
- Amend `SC-OFEROUTE-001` to rev 49 for canonical native datver production
  authority.
- Amend `plant-file.spec.md` to distinguish parse compatibility from production
  Lane D authority.
- Amend `openwepp-management-lanuse-authority-contract.md` with a native datver
  production rule.
- Record implementation gaps and handoff for runtime/wepppy producer work.
- Complete static review, verification, and gates appropriate to a no-code
  authority package.

### Excluded

- No Rust runtime selector, parser, or test implementation.
- No wepppy producer implementation.
- No coefficient projection or coefficient-table tuning.
- No optional sidecar authority.
- No deletion of legacy datver parser support or legacy/reference runtime paths.
- No groundwater/baseflow implementation.

## Intended Write Set

- `docs/work-packages/20260708-laned-router-ow-lanuse-canonical-production-datver-authority-001/**`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`

## Phase Plan

### Phase A - Decision Audit

1. Verify M-T2P final disposition rejected projection and sidecar authority.
2. Record the locked requirements in `artifacts/decision-lock-audit.md`.

### Phase B - Authority Amendments

1. Amend `SC-OFEROUTE-001` to rev 49.
2. Amend `plant-file.spec.md`.
3. Amend `openwepp-management-lanuse-authority-contract.md`.
4. Record `artifacts/contract-disposition.md`.

### Phase C - Implementation Gap

1. Identify runtime and producer gaps left intentionally open.
2. Record the next package/action in `artifacts/implementation-gap.md` and
   `artifacts/worker-handoff.md`.

### Phase D - Review, Verification, And Closure

1. Complete two static reviews and two static verification artifacts.
2. Run doc/contract gates.
3. Record disposition and final disposition.

## Subagent Authorization

This package explicitly authorizes subagent spawning/delegation for read-only
science-contract review and verification. Expected outputs are package-local
`artifacts/review-*.md` and `artifacts/verification-*.md`. Write access is
read-only unless the operator explicitly expands scope.

Subagent requirement: none for heavy batch/closure runs, because no Rust code,
release binary, comparator suite, or population evidence is in scope.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/decision-lock-audit.md`
- `artifacts/contract-disposition.md`
- `artifacts/implementation-gap.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

Required:

- `git diff --check`
- Markdown/doc lint for touched docs.
- SC unit compliance check.
- BEI check for touched `SC-*` contract.
- Static review and verification of authority consistency.

Not applicable:

- Rust tests and closure gates: no Rust production or test code changed.
- Comparator/timing gates: no runtime behavior changed.
- Anti-evasion guards: no required-case binding, cohort fixture, external
  authority suite posture, or test-suite obligation posture changed.

## Exit Criteria

`EXECUTED-COMPLETE-AUTHORITY`:

- `SC-OFEROUTE-001` rev 49 records the canonical native datver policy.
- Management spec and lanuse interface contract align with rev 49.
- Implementation gaps are explicit and handed off.
- Required static gates pass or are truthfully recorded.

`EXECUTED-HOLD-*`:

- Any touched canonical authority surface cannot be made internally consistent,
  or a required gate fails.

## Final Outcome

Executed complete. Authority was amended; implementation is handed off.
