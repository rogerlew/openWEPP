# Execute SNOW-STAGE3-LIQUID-SIGNED-HOUR-TRACE-CLOSURE

Scope: local repository science-contract and diagnostic-publication work;
flat-file reads/edits and local test execution only; no external connectivity.

Execution mode: package-end-to-end.

Phase plan: execute all phases in `package.md` sequentially through
disposition.

## Required Reading

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/package.md`
- `/home/workdir/openWEPP/docs/standards/testing-and-gate-strategy.md`

Conditional because this package amends contract and runtime publication:

- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/crates/AGENTS.md`
- `/home/workdir/openWEPP/tests/AGENTS.md`

On-demand:

- `SC-SNOWFREEZE-001.md`, `SC-SNOWENERGY-001.md`, and
  `SC-RUNOFFPART-001.md` for touched diagnostic/liquid boundaries.
- The predecessor disposition and worker handoff.

Required-reading budget: `509317` local bytes, `WARN`; map:
`artifacts/required-reading-map.md`. The catalog dominates the budget but stays
Core because package-chain and active-package reconciliation are mandatory.

## Task

Execute the package objective end-to-end for the declared write set. Preserve
physics and publish exact existing diagnostic values only.

Constraints: contract-first sequencing; canonical `SC-*` authority; typed
guards; no silent defaults; no canonicalize-and-proceed; no fixture or
observation edits.

No surrogate physics: production code may publish only exact contract-backed
runtime operands; surrogate, provisional, proxy, heuristic, or inferred state
fields are forbidden.

Real consumer proof: the release CLI must write schema-v4 JSONL, and the
independent parser must read that file. Wrappers, adapters, skeletons, shadow
paths, producer-only checks, and v3 compatibility formatters cannot carry the
closure claim.

Conservation/output acceptance: record operand lineage; separate plausible
aliases in fixtures; reject known wrong formulas; run independent
reconstruction plus a real closure/magnitude audit; align schema metadata; do
not close on one-sided bounds or producer self-consistency.

Subagent requirement: REQUIRED. Spawn `comparator_suite_runner` for heavy
quick/frost/full workspace gates; do not run them on the parent model unless it
is unavailable, in which case retain command-level evidence. This prompt
explicitly authorizes subagent spawning/delegation to two read-only independent
reviewers, two read-only terminal verifiers, and the comparator runner for the
scopes and outputs in `package.md`.

Autonomy: execute all phases and update every required artifact without asking
for additional user direction unless hard-blocked.

Outputs: archive this prompt unchanged after use and update package status,
evidence, review disposition, verification, handoff, catalog, roadmap, and
final disposition.
