# Follow-On Parser Implementation Work-Package Queue

Date: 2026-05-21
Evidence mode: `Static`

## Proposed Queue

1. `20260521-inimpl02-wave1-worktree-orchestration-001`
- Scope: shared Wave 1 scaffold governance, branch/worktree registry, disjoint
  ownership manifest, and integration sequencing rules.
- Exit signal: canonical worktree execution plan is `GO`.

2. `20260521-inimpl03-implement-sc-infile-slope-parser-001`
- Scope: implement `SC-INFILE-SLOPE-001` in dedicated worker worktree.
- Exit signal: slope parser + strict/compat tests + worker handoff complete.

3. `20260521-inimpl04-implement-sc-infile-soil-parser-001`
- Scope: implement `SC-INFILE-SOIL-001` in dedicated worker worktree.
- Exit signal: soil parser + strict/compat tests + worker handoff complete.

4. `20260521-inimpl05-implement-sc-infile-climate-parser-001`
- Scope: implement `SC-INFILE-CLIMATE-001` in dedicated worker worktree.
- Exit signal: climate parser + strict/compat tests + worker handoff complete.

5. `20260521-inimpl06-implement-sc-infile-management-parser-001`
- Scope: implement `SC-INFILE-MANAGEMENT-001` in dedicated worker worktree.
- Exit signal: management parser + strict/compat tests + worker handoff complete.

6. `20260521-inimpl07-wave1-core-parser-integration-001`
- Scope: integrate `INIMPL03..06` outputs onto mainline and run Wave 1 global
  gates.
- Exit signal: Wave 1 integrated with recorded `Ran` gate evidence.

7. `20260521-inimpl08-implement-wave2-hillslope-sidecars`
- Scope: Wave 2 sidecar parser implementation after Wave 1 integration.

8. `20260521-inimpl09-implement-wave3-watershed-core-parsers`
- Scope: Wave 3 watershed core parser implementation.

9. `20260521-inimpl10-implement-wave4-watershed-sidecar-parsers`
- Scope: Wave 4 watershed sidecar implementation.

10. `20260521-inimpl11-close-parser-governance-gap-fixtures`
- Scope: fixture-backed closure for highest-risk `*-GAP-*` governance items.

## Sequencing Constraints

1. `inimpl03..inimpl06` cannot start until `inimpl02` is `GO`.
2. `inimpl07` cannot start until `inimpl03..inimpl06` are each at least
   `GO-WITH-AMENDMENTS`.
3. `inimpl08` cannot start until `inimpl07` is `GO`.
4. `inimpl09` cannot start until `inimpl08` is `GO`.
5. `inimpl10` cannot start until `inimpl09` is `GO`.
6. `inimpl11` can start after `inimpl10` reaches at least `GO-WITH-AMENDMENTS`.

## Ownership Notes

- Parser implementations remain openWEPP-owned and contract-authority-driven.
- Any divergence from `SC-INFILE-*` strict/compat policy requires explicit
  work-package disposition updates, not silent implementation drift.
