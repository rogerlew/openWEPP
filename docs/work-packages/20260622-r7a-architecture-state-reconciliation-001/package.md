# R7A Architecture State Reconciliation

Status: complete.

Package type: Documentation authority reconciliation work package.

Objective: reconcile the array-native runtime architecture authority with the
actual post-R6J implementation state, without changing Rust behavior,
activation defaults, output schemas, science contracts, or runtime selection.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only reviewer and verifier subagents for
architecture-state review, catalog consistency review, and gate-evidence audit.
Expected outputs are compact Markdown findings summarized into
`artifacts/review-disposition.md`; subagents may not edit files.

## Rationale

R6J completed opt-in direct publication cutover, but the default runner API and
CLI still select compatibility mode. The architecture specification was updated
with a post-R6 burndown sequence, but R7A must make the current authority state
explicit so later packages do not confuse opt-in direct publication cutover with
default direct runtime completion.

## Scope

In scope:

- Record PERFDEEP09 as the hold-lift authority for PERFDEEP07.
- Record R2 through R5 as direct-runtime scaffold and phase-coverage evidence.
- Record R6J as opt-in direct publication cutover, not default activation or
  full runtime completion.
- Add a current-state matrix separating compatibility mode, shadow mode, direct
  publication cutover, and future production direct mode.
- Reconcile ADR-0025 references with Revision 3 / R7 burndown authority.
- Update the work-package catalog with R7A closure.
- Complete package-local evidence, review, verification, and handoff artifacts.

Out of scope:

- Rust implementation edits.
- Default activation.
- Direct executor routing changes.
- Publication producer authority changes.
- Output schema changes.
- Science-contract changes.
- Benchmark execution.

## Write Set

- `docs/architecture/array-native-runtime-specification.md`
- `docs/decisions/0025-array-native-hillslope-day-frame.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260622-r7a-architecture-state-reconciliation-001/**`

## Phase Plan

1. Scaffold this package and active prompt.
2. Update architecture state tables and current-state matrix.
3. Add ADR-0025 amendment noting Revision 3 and R7 burndown authority.
4. Update work-package catalog with final R7A disposition.
5. Complete evidence artifacts.
6. Run scoped Markdown lint and `git diff --check`.

## Acceptance Criteria

- Architecture spec and work-package catalog agree on R0-R6J status.
- The spec explicitly states that R6J is opt-in direct publication cutover, not
  default activation or full runtime completion.
- The current-state matrix names compatibility mode, shadow mode, direct
  publication cutover, and future production direct mode, including normal
  authority and remaining blockers.
- ADR-0025 points to Revision 3/R7 burndown without changing science or output
  schema authority.
- Package artifacts record static evidence, review, verification, line-count
  governance, and handoff.
- Scoped Markdown lint and `git diff --check` pass.

## Security / Safety Impact

Documentation-only. No secrets, runtime code, generated artifacts, schema files,
or validation fixtures are changed. The package must not weaken fail-closed,
contract-first, direct-runtime, or no-compatibility requirements.

## Progress

- [x] Scaffold package.
- [x] Reconcile architecture state.
- [x] Reconcile ADR and work-package catalog.
- [x] Complete evidence artifacts.
- [x] Run scoped validation.

## Outcomes

Final disposition: `COMPLETE-R7A-ARCHITECTURE-STATE-RECONCILIATION`.

The package reconciled documentation authority only:

- `docs/architecture/array-native-runtime-specification.md` now records
  PERFDEEP09 as the hold-lift authority for PERFDEEP07, includes a
  current-state table through R7, and includes a runtime-mode matrix separating
  compatibility, shadow, direct publication cutover, and future production
  direct modes.
- `docs/decisions/0025-array-native-hillslope-day-frame.md` now includes
  Amendment 2 binding Revision 3 and the R7A-R7H burndown sequence while
  preserving science/output/schema non-decisions.
- `docs/work-packages/README.md` records this package as complete.

No Rust code, runtime behavior, default activation, output schema, fixture, or
science-contract authority changed.
