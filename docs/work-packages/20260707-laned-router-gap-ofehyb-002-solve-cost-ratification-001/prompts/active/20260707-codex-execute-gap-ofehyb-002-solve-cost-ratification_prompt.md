# Codex Prompt - Execute GAP-OFEHYB-002 Solve-Cost Ratification

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/package.md`
sequentially through disposition.

Required reading (read before edits):

Core:
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- `docs/backlog/20260706-laned-router-numerics-performance-tiers.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/final-disposition.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/verification-h2637-timing.md`
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/final-disposition.md`
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/ratification-evidence.md`
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/verification-h2637-timing.md`
- `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i0-scheme-design.md`
- `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i1-implicit-stepper-evidence.md`
- `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i2-hybrid-evidence.md`
- `docs/work-packages/20260706-laned-router-t3-aggressive-deficit-carry-001/artifacts/fix-evidence.md`
- Package-local `artifacts/required-reading-map.md`

Conditional:
- `docs/specifications/science-contract-authoring-procedure.md` if contract
  schema/profile repair is needed beyond a local amendment pattern.
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  if adding or reorganizing contract invariant/guard/profile sections.
- `docs/specifications/unit-governance.md` if any runtime symbol, output,
  counter, unit, or metadata surface changes.
- `docs/standards/local-ci-gate-selection.md` when choosing narrowed iteration
  gates before final closure.

On-demand:
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/d10b_reconciliation_tests.rs`
- Runner active-lane/profile files only if timing/provenance surfaces need
  changes.

Required-reading budget: `260236`, OK; map:
`artifacts/required-reading-map.md`.

Files:
- `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` only for
  parent-pointer synchronization if needed.
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
- Focused tests under the same crate.

Task: execute the package objective end-to-end for declared scope. Close, or
legitimately hold, `SC-OFEROUTE-002#GAP-OFEHYB-002`: reduce or adjudicate the
implicit solve-cost bottleneck on the source-memory hybrid path, then audit
`INV-OFEHYB-008` fidelity/timing promotion readiness.

Constraints: contract-first sequencing; canonical SC authority; typed guards;
no silent defaults; no branch-history-dependent seeding; no surrogate physics;
no tolerance weakening; no compatibility wrappers; no selector/default
promotion unless every promotion gate is directly proven.

No surrogate physics: production code must implement actual contract-backed or
baseline-authoritative numerics; surrogate/provisional/proxy/heuristic
stand-ins are forbidden. Missing authority is a hold-for-authority boundary;
known in-scope contract-neutral optimization is an implementation obligation.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for all heavy
batch/closure/comparator runs (release H2637 timing, full workspace nextest,
deny, comparator ladders); do NOT run them on the parent model unless the
subagent is unavailable, in which case record command-level evidence. Standing
user authorization for openWEPP subagent delegation is expected in the session.
This prompt explicitly authorizes subagent spawning/delegation to
`comparator_suite_runner`, `rust_code_reviewer`, `rust_qa_reviewer`,
`explorer`, and bounded `worker` roles for timing/comparator execution, dual
review, QA verification, bounded codebase questions, and assigned disjoint
implementation subtasks; outputs: compact metrics, review/verification
artifacts, command lines, and log paths; write access: read-only except bounded
worker assignments.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
