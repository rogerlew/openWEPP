# R3A Kickoff Agent Prompt

Scope: local repository kernel/runtime implementation task; flat-file
reads/edits only; no external connectivity.

In `/home/workdir/openWEPP`, execute
`docs/work-packages/20260620-r3a-first-direct-phase-span-001/` end to end.

Execution mode: package-end-to-end.

Objective: implement the first complete direct-runtime phase span. Extend the
R2A direct skeleton into a real direct phase execution path for one selected
bounded span. The selected span must include typed inputs, direct compute,
state mutation, downstream operands, and shadow projection, without
publication cutover, endpoint-improvement claim, or default activation.

Required reading:

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/ROADMAP.md`
- `/home/workdir/openWEPP/docs/work-packages/20260620-r3a-first-direct-phase-span-001/package.md`
- `/home/workdir/openWEPP/docs/architecture/array-native-runtime-specification.md`
- `/home/workdir/openWEPP/docs/decisions/0025-array-native-hillslope-day-frame.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/direct-frame-type-boundary-decision.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/no-compatibility-proof-plan.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/r1-frame-constructor-projection-plan.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r2a-direct-runtime-skeleton-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r2a-direct-runtime-skeleton-001/artifacts/disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r2a-direct-runtime-skeleton-001/artifacts/gate-results.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r2a-direct-runtime-skeleton-001/artifacts/no-compatibility-proof-checklist.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r2a-direct-runtime-skeleton-001/artifacts/worker-handoff.md`

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

Execution order:

1. Populate required-reading, owned-file, and pre-implementation artifacts.
2. Inventory R2A direct runtime execution and forbidden compatibility surfaces.
3. Define the R3A span contract, including selected phase name(s), typed
   inputs, direct compute, state mutation, downstream operands, and shadow
   projection.
4. Implement direct phase-span status/error types and dispatch for the selected
   span.
5. Implement fail-closed direct frame numeric-domain validation for the span.
6. Wire explicit opt-in/test direct skeleton selection through the complete
   span, preserving default compatibility early return.
7. Add focused tests for valid execution, invalid value rejection, state
   mutation, downstream operands, shadow projection identity, default-disabled
   inactivity, opt-in execution, source-token prohibitions, runtime counters,
   and no scheduler diff.
8. Run focused Rust tests and static no-compatibility scans.
9. Run the default-disabled H2637 regression gate. Required final median:
   `<= 676.67 s`.
10. Run full closure gates:
   `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`, `cargo deny check`, scoped docs lint, and
   `git diff --check`.
11. Complete line-count governance, dual review, finding disposition, dual
    verification, roadmap/catalog updates, disposition, and worker handoff.

Constraints: no broad hydrology path migration, no output publication cutover,
no output schema/unit/metadata/conservation-operand changes, no default
activation, no runtime-readiness claim, no hidden compatibility fallback, no
silent dependency masking. The selected span must still include typed inputs,
direct compute, state mutation, downstream operands, and shadow projection.
Gate: phase-span identity plus no-compatibility call-graph proof and
non-tautological runtime counters. If the selected span requires unsourced
physical capacity or output authority, hold instead of inventing surrogate
math.

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
proceed into R4 hydrology or R6 publication implementation.
