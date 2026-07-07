# Codex Execution Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):

Core:
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/package.md`
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/required-reading-map.md`
- Parent T3 and T3-AGG package artifacts named in `package.md`.

Conditional:
- `docs/specifications/science-contract-authoring-procedure.md` if contract
  schema/profile repair is needed.
- `docs/standards/local-ci-gate-selection.md` if narrowed iteration gates are
  selected.

On-demand:
- D10B/D15A artifacts and `ofe_routing` source files for touched mechanisms.

Required-reading budget: 208462 bytes, OK; map:
`artifacts/required-reading-map.md`.

Files:
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/laned_shadow.rs` if profile output
  schema changes.
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/`
- `docs/work-packages/README.md`
- Scaffold-only Tier-1/Tier-2 package dirs named in `package.md`.

Task: execute package objective end-to-end for declared scope.

Constraints: contract-first sequencing; canonical `SC-OFEROUTE-001` authority;
typed guards; no silent defaults; no canonicalize-and-proceed for domain
violations; no surrogate/provisional/proxy/heuristic process physics.

Real consumer proof: do not claim hybrid selector promotion unless the real
active Lane-D hybrid path, not a shadow-only or skeleton path, carries the
evidence.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for heavy
batch/closure/comparator runs and release timing runs; do NOT run them on the
parent model unless the subagent is unavailable, in which case record
command-level evidence. This prompt explicitly authorizes subagent
spawning/delegation to `comparator_suite_runner`, `rust_code_reviewer`,
`rust_qa_reviewer`, and `explorer` for comparator/timing execution, dual review,
verification, and bounded codebase questions; outputs: compact metrics plus
package-local review/verification artifacts; write access: read-only unless a
worker is separately assigned a disjoint bounded implementation write set.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.

