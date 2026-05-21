# INIMPL02 Wave 1 Integration Sequence

Evidence mode: `Static`

## 1. Scope

Define deterministic intake, cherry-pick order, conflict handling, and gates for integrating `INIMPL03..06` into Wave 1 integration package `INIMPL07`.

## 2. Intake Prerequisites per Worker

Required artifacts from each worker package:
1. `artifacts/worker-handoff.md`
2. `artifacts/owned-file-manifest.md`
3. `artifacts/inimpl0X_disposition.md`
4. `artifacts/verification_agent_a.md`
5. `artifacts/verification_agent_b.md`

Hard intake rule:
- unresolved high-severity findings in worker review/disposition/verification => worker cannot enter integration queue.

## 3. Canonical Integration Order

1. `INIMPL03` (`SC-INFILE-SLOPE-001`)
2. `INIMPL04` (`SC-INFILE-SOIL-001`)
3. `INIMPL05` (`SC-INFILE-CLIMATE-001`)
4. `INIMPL06` (`SC-INFILE-MANAGEMENT-001`)

[DIRECT] This order is aligned with canonical Wave 1 queue and INIMPL07 package integration order.

## 4. Per-Step Integration Procedure

For each worker in order:
1. Confirm branch SHA and owned-file manifest.
2. Cherry-pick worker commits into integration branch.
3. Validate no ownership violations (changed files outside manifest).
4. Resolve conflicts using ownership policy.
5. Run incremental checks appropriate to changed surfaces.

## 5. Conflict Resolution Policy

Priority stack:
1. Science contract correctness (`SC-INFILE-*` guard/invariant semantics)
2. Ownership manifest compliance
3. Minimal diff/change isolation
4. Integration velocity

Conflict classes:
- Shared scaffolding conflict -> resolved only by integration owner.
- Cross-worker fixture collision -> rename/rehome into worker namespace.
- Test harness collision -> integration owner refactors harness with no semantic drift.

All conflicts must be logged in INIMPL07 `merge-conflict-log.md`.

## 6. Wave 1 Promotion Gates (INIMPL07)

Required `Ran` gates:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`
5. Surface acceptance checks for slope/soil/climate/management strict+compat behavior and typed errors.

No `GO` promotion if any gate fails.

## 7. Rollback Policy

If integration of worker `N` fails and cannot be resolved without ownership breach:
1. Revert worker `N` integration commit(s).
2. Keep prior integrated workers in place.
3. Open corrective follow-on package for worker `N`.
4. Continue only if Wave 1 invariants remain intact.

## 8. Blockers

Integration cannot start when:
1. Shared scaffold baseline commit is not established.
2. Any worker lacks verified handoff artifacts.
3. Branch/worktree registry does not match actual branch topology.
4. Unresolved high-severity findings exist in worker package artifacts.
