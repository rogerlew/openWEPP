# MOFE01 Kickoff — increment M-D (per-OFE state architecture design)

Execution mode: staged increments per `artifacts/mofe-staged-increment-plan.md`.
This kickoff governs increment **M-D** (per-OFE state architecture design;
**NO production code, NO contract edits** — a design artifact only). Later
increments (M-E implementation, M-F publication, M-G erosion, M-H acceptance)
are dispatched by the plan's dispatch line.

Subagent authorization (REQUIRED, not optional): this prompt explicitly
authorizes subagent spawning/delegation to `comparator_suite_runner`
(gpt-5.3-codex-spark) for any heavy batch/comparator runs and to
review/verification subagents. M-D is design-only, so heavy runs are unlikely;
record command-level evidence if the subagent is unavailable
(`docs/standards/prompt-wording-guidance.md` §4a).

Autonomy: execute M-D end-to-end — produce the architecture design artifact —
without asking for direction on intermediate steps. Hard stop: NO production
code or contract edits in M-D (design increment).

## The assignment (M-D)

Per `artifacts/mofe-staged-increment-plan.md` increment M-D, produce
`artifacts/mofe-per-ofe-state-architecture.md` with all five sections filled,
`Static:` and file:line-cited against the **current tree**:

1. Target per-OFE daily WB state/flux shape (replacing the aggregate maps at
   `HillslopeWritebackSurface`, `scheduler.rs:240`).
2. Sequential OFE execution model (i's daily state → i+1 run-on; topology vs
   lane iteration over `execute_with_kernel`, `scheduler.rs:501`; legacy
   `irs`/`rochek` mapping from `mofe-routing-port-scope.md`).
3. Contract surface for the per-element + transfer identities.
4. Change map across kernel-contract / scheduler / writeback / publication,
   with the single-OFE-anchor preservation argument.
5. Red-test definitions + the M-E sub-increment breakdown and sizing (each
   sub-increment behind a conservation hard stop; per-element + transfer
   identities first proven in M-E).

Read the lines, do not infer from symbol tables (Dh lesson). The M-C2 evidence
(`m-c2-per-ofe-daily-state-scope-evidence.md`) already line-cites the as-is
seams — start there and the legacy routing map.

## Gate (Gate Evidence Non-Deferral Rule)

M-D's only completion criterion is the design artifact itself — every section
filled with current-tree citations and an M-E sub-increment plan whose every
gate is measurable in its own scope. With no production scope, M-D legitimately
closes `complete` on that evidence. Do NOT begin M-E implementation under the
M-D dispatch.

## Required reading

- `artifacts/mofe-staged-increment-plan.md` (this plan; the M-D section + universal rules)
- `package.md` (envelope, comparator posture, conservation identities)
- `artifacts/mofe-routing-port-scope.md` (legacy routing authority)
- `artifacts/m-c2-per-ofe-daily-state-scope-evidence.md` (as-is seam citations)
- `artifacts/m-c-wat-publication-closure-evidence.md` (the architectural finding)
- `/workdir/openWEPP/AGENTS.md`, `docs/codex_exec_plans.md`,
  `docs/work-packages/AGENTS.md` (Gate Evidence Non-Deferral Rule),
  `docs/work-packages/README.md`
- On-demand: `SC-RUNOFFPART-001`/`SC-WATBAL-001`/`SC-SYSTEM-001` (section-targeted);
  current-tree `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`,
  `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs`,
  `crates/openwepp-runner/src/hillslope/` publication seams; FDHP01
  `d3-fine-sublayer-port-scope.md` (the design-artifact template)
