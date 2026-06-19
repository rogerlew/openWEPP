# R2A Kickoff Agent Prompt

Scope: local repository kernel/runtime implementation task; flat-file
reads/edits only; no external connectivity.

In `/home/workdir/openWEPP`, execute
`docs/work-packages/20260619-r2a-direct-runtime-skeleton-001/` end to end.

Execution mode: package-end-to-end.

Objective: implement the R2A direct-runtime skeleton. Introduce a distinct
direct-runtime type namespace and a no-op or shadow-only direct executor
entrypoint selected once at setup. Prove the skeleton does not enter
compatibility execution surfaces and that it is zero-cost on the
default-disabled compatibility path.

Required reading:

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/ROADMAP.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r2a-direct-runtime-skeleton-001/package.md`
- `/home/workdir/openWEPP/docs/architecture/array-native-runtime-specification.md`
- `/home/workdir/openWEPP/docs/decisions/0025-array-native-hillslope-day-frame.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/r0-runtime-schema-planning.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/direct-frame-type-boundary-decision.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/r1-frame-constructor-projection-plan.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/publication-ledger-promotion-plan.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/no-compatibility-proof-plan.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep09-disabled-path-iterative-defect-closure-001/artifacts/disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep09-disabled-path-iterative-defect-closure-001/artifacts/gate-results.md`

Required before Rust edits:

- `/home/workdir/openWEPP/crates/AGENTS.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`

Conditional:

- `/home/workdir/openWEPP/tests/AGENTS.md` before root test edits
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
  if canonical contracts must change
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
  if canonical contracts must change
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
  if canonical contracts must change
- `/home/workdir/openWEPP/tools/owcmp/AGENTS.md` before owcmp edits

Required-reading budget: `212304`, `OK`; map:
`artifacts/required-reading-map.md`.

Execution order:

1. Populate required-reading, owned-file, and pre-implementation artifacts.
2. Inventory current compatibility entrypoints and forbidden APIs.
3. Implement direct-runtime module namespace and direct-frame type shells.
4. Implement no-op or shadow-only direct executor entrypoint selected once at
   setup behind explicit opt-in/test selection.
5. Add static/compile tests for direct-frame type prohibitions and call-graph
   separation.
6. Add runtime counters/audit evidence for direct skeleton execution and
   default-disabled compatibility execution.
7. Run focused tests and prove no default-disabled construction/tax.
8. Run the PERFDEEP09 default-disabled H2637 regression gate. Required final
   median: `<= 676.67 s`.
9. Run full closure gates:
   `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`, `cargo deny check`, scoped docs lint, and
   `git diff --check`.
10. Complete line-count governance, dual review, finding disposition, dual
    verification, roadmap/catalog updates, disposition, and worker handoff.

Constraints: no phase math, no output publication cutover, no output schema,
unit, metadata, or conservation-operand changes, no default activation, no
direct hydrology readiness claim, no hidden compatibility fallback, no silent
dependency masking.

Subagent requirement: this prompt explicitly authorizes spawning/delegation to
read-only static-audit, benchmark runner, reviewer, and verifier subagents for
no-compatibility proof review, default-disabled H2637 regression runs,
closure-gate review, line-count-governance review, package artifact review, and
gate-legitimacy verification. Outputs: compact metrics, log paths, call-graph
findings, and review findings recorded in package artifacts. Write access:
package artifacts only unless the package is explicitly amended. If subagents
are unavailable, record command-level evidence before running locally.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked. Do not
proceed into R3 phase-span implementation.
