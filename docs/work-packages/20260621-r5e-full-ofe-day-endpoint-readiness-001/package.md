# R5E - Full OFE-Day Endpoint Readiness

Status: complete.

Package type: implementation work package / array-native runtime R5E.

## Objective

Close R5 by proving the direct executor records one canonical 14-phase entry per
OFE-day in `DirectPhaseKind::ORDERED`, folds the R4/R5 direct spans under those
canonical phase entries, preserves no-publication/no-default boundaries, and
provides endpoint evidence for deciding whether R6 direct publication cutover can
resume.

## Scope

In scope:

- add R5E-specific canonical phase-entry reporting where needed;
- prove every canonical phase is `Executed` for every direct day/lane frame;
- prove R4 hydrology subspans remain sub-operations and are not counted as extra
  canonical phase executions;
- prove direct-runtime source remains free of compatibility request/writeback/
  symbol storage in the direct path;
- prove default-disabled compatibility path remains zero direct-runtime
  construction and under the H2637 timing gate;
- record H2637 default-disabled and opt-in direct-skeleton endpoint/RSS
  evidence;
- preserve protected output identity/equivalence;
- update R5 burn-down progress, roadmap, package catalog, reviews,
  verification, and disposition.

Out of scope:

- public WB13/WAT/PASS/loss/manifest cutover;
- direct-publication operand authority;
- default activation;
- output schema changes;
- R6/R7 compatibility-runtime deletion.

## Authority

- `docs/work-packages/r5-burndown-execplan.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- R5A-D package evidence under `docs/work-packages/20260620-r5*/`
- R4P/Q/Z hydrology projection closure evidence.

## Intended Write Set

- `docs/work-packages/20260621-r5e-full-ofe-day-endpoint-readiness-001/**`
- `docs/work-packages/r5-burndown-execplan.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`

Files outside this set require package amendment before edits.

## Phase Plan

1. Confirm R5A-D and R4P/Q/Z are complete and R5E is still unchecked.
2. Add or confirm explicit canonical 14-phase endpoint reporting and focused
   R5E tests.
3. Run focused direct-runtime and runner counter tests.
4. Run no-compatibility source scan and line-count governance.
5. Run default-disabled H2637 benchmark and protected-output comparison.
6. Run opt-in direct-skeleton H2637 endpoint/RSS evidence.
7. Run full Rust closure gates, scoped docs lint, and whitespace checks.
8. Complete review, verification, roadmap/catalog updates, disposition, and
   worker handoff.

## Acceptance Criteria

- R5E focused test proves exactly `14 * lane_count * day_count` canonical phase
  entries in `DirectPhaseKind::ORDERED`.
- The direct execution report distinguishes canonical phase entries from
  sub-operation/direct-span entry counters.
- All 14 canonical phases report `Executed` for every direct day/lane frame.
- R4/R5 direct spans remain folded under canonical phase status counts.
- Direct-runtime source scan includes all direct-runtime modules, including
  growth, and rejects compatibility storage/request/writeback/symbol tokens.
- Default-disabled runner fixture records zero direct-runtime counters.
- Explicit direct-skeleton runner fixture records all direct day/lane frames,
  canonical phase entries, direct sub-operation counters, and exactly one
  declared compatibility-edge handoff for current publication/output validation.
- Default-disabled H2637 median remains `<= 676.67 s`.
- Protected output identity/equivalence passes; no public output cutover occurs.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` pass.
- Scoped markdown lint and `git diff --check` pass.

## Execution Summary

R5E completed the R5 endpoint-readiness scope without public publication
cutover. The direct execution report now records canonical phase entries
separately from direct sub-operation counters, and focused tests prove exactly
`14 * lane_count * day_count` canonical phase entries in
`DirectPhaseKind::ORDERED`.

H2637 default-disabled compatibility remained under the `<= 676.67 s` timing
gate with reps `641.37 s`, `642.02 s`, and `635.47 s` (median `641.37 s`).
The opt-in direct-skeleton endpoint ran at `638.33 s`. Protected output
comparison passed: HBP, WAT, loss, and plot were byte-identical; PASS parquet
bytes differed, but DuckDB row equivalence passed with `12419` rows and zero
bidirectional differences.

R5E does not introduce a direct-only/projection-only public output endpoint.
R6 is unblocked to resume direct publication cutover, subject to first
promoting the PERFDEEP06 publication operand ledger into canonical authority.

## Conservation / Publication Acceptance

R5E does not change public publication authority. Protected output identity is a
regression gate only. R6 owns publication ledger authority and output cutover.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only reviewer, verifier, and benchmark runner
subagents for no-compatibility proof review, H2637 evidence review, output
comparison review, package artifact review, and gate-legitimacy verification.
Expected outputs are compact findings, command logs, and metrics recorded in
package artifacts. Write access is limited to package artifacts unless this
package is explicitly amended.

## Deliverables

- `artifacts/README.md`
- `artifacts/scope-selection.md`
- `artifacts/process-span-contract.md`
- `artifacts/operand-lineage.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/implementation-test-evidence.md`
- `artifacts/no-compatibility-proof-checklist.md`
- `artifacts/default-disabled-regression-gate.md`
- `artifacts/endpoint-rss-evidence.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/disposition.md`
- `artifacts/worker-handoff.md`
- `prompts/active/20260621-r5e-full-ofe-day-endpoint-readiness-001_kickoff_agent_prompt.md`
- `prompts/archived/README.md`
