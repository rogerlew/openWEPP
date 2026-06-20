# R4I-L - Direct Runoff-Path Input Completion

Status: complete.

Package type: implementation work package / array-native runtime R4I-L.

## Objective

Replace the remaining manually seeded inputs consumed by R4A direct runoff
partition with direct upstream producers for liquid input, runon/carry,
cumulative infiltration, depression-storage delta, and surface-saturation
addback.

R4I-L must prove that R4A no longer accepts those operands solely because tests
or executor setup initialized `runoff_partition_inputs`. Each operand must have
typed direct inputs, direct handoff compute, state mutation, downstream
operands, and shadow projection before R4A may compute runoff.

## Rationale

R4A proved the runoff-partition consumer, but its input surface remained
seeded. R4I-L completes the direct runoff-path input producer layer while
keeping full WB14 infiltration/depression/saturation equation migration out of
scope unless the package contract gate explicitly authorizes it.

## Scope

In scope:

- direct liquid-input handoff producer after interception/snow/rain/irrigation
  coupling;
- direct surface runon/carry handoff producer plus typed subsurface-carry
  diagnostic separation;
- direct cumulative-infiltration and depression-storage-delta handoff producer;
- direct surface-saturation-addback handoff producer;
- R4A missing-upstream fail-closed requirements for all R4I-L producers;
- focused tests for producer identity, R4A consumption, invalid guards,
  anti-alias vectors, aggregate executor counters, and runner counters;
- source split for runoff-specific direct-runtime code if needed to stay under
  line-count governance thresholds;
- package artifacts, reviews, verification, line-count governance, final gates,
  commit, push, and ExecPlan progress update.

Out of scope:

- full WB14 infiltration/depression/saturation equation migration;
- public runoff/WAT/PASS/loss/schema publication cutover;
- scheduler edits, compatibility runtime edits, default activation, or output
  schema/manifest changes;
- treating raw precipitation, routed melt, irrigation publication, public
  `UpStrmQ`, storage residual, or publication runoff as authoritative R4I-L
  operands.

## Authority

Canonical authority:

- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
  - direct runoff-partition equation surface and operand separation;
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - daily water-balance ordering and closure constraints;
- R3C/R4A package evidence for topology/carry and runoff-partition consumer
  boundaries.

R4I-L is a handoff migration package. It does not amend canonical `SC-*` text
unless the pre-implementation contract gate finds insufficient authority.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/r4-burndown-execplan.md`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/work-packages/20260620-r4a-direct-runoff-partition-span-001/package.md`
- `docs/work-packages/20260620-r3c-direct-multilane-transfer-span-001/package.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`

On-demand source inventory:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`

## Intended Write Set

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `docs/work-packages/20260620-r4il-direct-runoff-path-input-completion-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/r4-burndown-execplan.md`
- `docs/ROADMAP.md`

No scheduler, output writer, output schema, dependency, or compatibility
runtime edit is authorized.

## Subagent Authorization

No delegated subagent work is required by this package. Review and verification
artifacts may be completed locally unless the user separately requests
delegated agents.

## Phase Plan

1. Record producer selection, process spans, operand lineage, contract gate,
   no-compatibility plan, default-disabled gate plan, and line-count baseline.
2. Split runoff-specific direct-runtime code out of `direct_runtime.rs` if
   needed for line-count governance.
3. Add R4I-L direct producer types, validation, compute, downstream operands,
   and shadow projections.
4. Extend R4A completeness checks so runoff partition requires R4I-L upstream
   shadows.
5. Extend the direct executor order so R4I-L runs after storage-budget handoff
   producers and before R4A.
6. Add focused tests for identity, invalid input guards, anti-alias vectors,
   missing-upstream fail-closed behavior, aggregate counters, and runner
   counters.
7. Run focused tests, closure gates, no-compatibility scan, scheduler no-diff,
   markdown lint, `git diff --check`, and H2637 default-disabled reps.
8. Complete review, disposition, verification, line-count governance, roadmap
   and package catalog updates, worker handoff, final disposition, commit, and
   push.

## Exit Criteria

- R4I-L process spans are recorded before production Rust edits.
- Operand lineage records units, source authority, and diagnostic vs
  authoritative status for liquid input, runon/carry, infiltration, depression
  storage, and saturation addback.
- Direct producers include typed inputs, direct handoff compute, direct state
  mutation, downstream operands, and shadow projection.
- Direct producers validate finite nonnegative runoff-path operands.
- R4A consumes R4I-L-produced values and fails closed if any required R4I-L
  producer did not run.
- Anti-alias tests distinguish R4I-L operands from adjacent
  process/publication/diagnostic/residual substitutes.
- Direct-runtime source remains free of compatibility
  storage/request/writeback/symbol calls and owned legacy-symbol construction.
- Default-disabled runner fixture records zero direct-runtime counters.
- Explicit opt-in runner fixture records positive counters for all direct spans
  through R4I-L and one production compatibility handoff.
- Default-disabled H2637 median is `<= 676.67 s` and protected identity passes.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` pass.
- Scoped markdown lint and `git diff --check` pass.
- Line-count governance is current; no touched non-exempt `.rs` file at or
  above 3000 lines remains unresolved.
- Dual review, finding disposition, dual verification, worker handoff, and
  final disposition are complete.

## Closure Verdict

PASS. R4I-L added direct handoff producers for liquid input, runon/carry,
cumulative infiltration plus depression-storage delta, and surface-saturation
addback. R4A now fails closed unless those producers ran, focused and workspace
gates passed, and the H2637 default-disabled median remained below the
`676.67 s` regression threshold.

## Security / Safety

R4I-L preserves fail-closed typed errors, direct-runtime default-disabled
behavior, no scheduler/publication cutover, and no compatibility storage access
inside direct runtime. Handoff values are shadow-only until later R4/R6 packages
promote full compute and publication surfaces.
