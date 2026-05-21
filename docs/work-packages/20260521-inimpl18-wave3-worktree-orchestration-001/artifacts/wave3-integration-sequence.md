# INIMPL18 Wave 3 Integration Sequence

Evidence mode: `Static`

## 1. Scope

Define deterministic intake, cherry-pick order, conflict handling, and
promotion gates for integrating `INIMPL19..21` into Wave 3 integration package
`INIMPL22`.

## 2. Intake Prerequisites Per Worker

Required artifacts from each worker package:
1. `artifacts/worker-handoff.md`
2. `artifacts/owned-file-manifest.md`
3. `artifacts/inimpl2X_disposition.md`
4. `artifacts/verification_agent_a.md`
5. `artifacts/verification_agent_b.md`

Hard intake rules:
- unresolved high-severity findings => worker cannot enter integration queue,
- owned-file manifest must remain within worker ownership boundaries,
- shared-file mutation requests must be explicitly listed in handoff.

## 3. Canonical Integration Order

1. `INIMPL19` (`SC-INFILE-WATERSHED-STRUCTURE-001`)
2. `INIMPL20` (`SC-INFILE-WATERSHED-CHANNEL-001`)
3. `INIMPL21` (`SC-INFILE-WATERSHED-IMPOUNDMENT-001`)

[DIRECT] Order aligns with `INIMPL22` package integration order and Wave 3
queue intent.

## 4. Per-Step Integration Procedure

For each worker in order:
1. Confirm branch SHA and owned-file manifest.
2. Confirm no unresolved high-severity findings.
3. Cherry-pick worker commit set into integration branch.
4. Validate no ownership violations (changed files outside manifest).
5. Resolve conflicts under ownership policy and log all decisions.
6. Run incremental checks for touched surfaces.

## 5. Conflict Resolution Policy

Priority stack:
1. science contract correctness (`SC-INFILE-*` guard/invariant semantics),
2. ownership-manifest compliance,
3. minimal-diff isolation,
4. integration velocity.

Conflict classes:
- shared scaffolding conflict -> resolved only by integration owner,
- cross-worker fixture collision -> rename/rehome into canonical fixture
  namespace,
- parser registry/harness collision -> integration owner applies centralized
  wiring with no semantic drift.

All conflicts must be logged in `INIMPL22` `merge-conflict-log.md`.

## 6. Wave 3 Promotion Gates (INIMPL22)

Required `Ran` gates:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`
5. watershed-core parser acceptance checks for strict/compat behavior and typed
   errors across all three Wave 3 surfaces.

No `GO` promotion if any gate fails.

## 7. Rollback Policy

If integration of worker `N` fails and cannot be resolved without ownership
breach:
1. revert worker `N` integration commit(s),
2. keep prior integrated workers in place,
3. open corrective follow-on package for worker `N`,
4. continue only if Wave 3 invariants remain intact.

## 8. Blockers

Wave 3 integration beyond intake cannot start when:
1. any worker handoff artifact bundle is incomplete,
2. any worker worktree stream (`INIMPL19..21`) is not provisioned,
3. branch/worktree registry diverges from actual topology,
4. shared scaffold baseline commit is not recorded,
5. unresolved high-severity findings exist in any worker stream,
6. prerequisite dependency stream `INIMPL17` (Wave 2 integration) is not
   verified closed.
