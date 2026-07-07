# Codex Prompt — Execute LANED-HYB-GAP001

Scope: local repository science-contract/kernel routing task; flat-file
reads/edits only; no external connectivity required.

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
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/package.md`
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/ratification-evidence.md`
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/hold-legitimacy-audit.md`
- `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i0-scheme-design.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/required-reading-map.md`

Conditional:
- `docs/specifications/science-contract-authoring-procedure.md` if contract
  schema/profile repair is needed beyond the local amendment pattern.
- `docs/standards/local-ci-gate-selection.md` when selecting narrowed
  iteration gates before final closure.

On-demand:
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/d10b_reconciliation_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- Runner active-lane files only if H2637 timing/profile output requires
  manifest/profile changes.

Required-reading budget: local_required_bytes_total `215000`,
OK (`<=400000`); map:
`docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/required-reading-map.md`.

Files:
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/d10b_reconciliation_tests.rs`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/`
- `docs/work-packages/README.md`

Task: execute package objective end-to-end for the declared scope. Close or
legitimately hold `GAP-OFEHYB-001`.

Constraints: contract-first sequencing; canonical `SC-*` authority; typed
guards; no silent defaults; no canonicalize-and-proceed for domain violations;
no surrogate/provisional/proxy/heuristic process physics in production code;
do not weaken the parent Case-4 acceptance tolerance.

DC closure: close `GAP-OFEHYB-001` end-to-end if an in-envelope predicate can
pass the retained Case-4 ladder. Do not hold while source reading,
implementation, contract/test work, or validation remains possible inside the
declared envelope. If HOLD is claimed, record a hold legitimacy audit naming
the boundary, evidence, considered in-envelope correction route, and why it
cannot close now.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for heavy
batch/closure/comparator runs (release timing, Case-4 ladder when run as a
heavy retained gate, full workspace `nextest`, `cargo deny check`); do NOT run
them on the parent model unless the subagent is unavailable, in which case
record command-level evidence. Standing user authorization for openWEPP
subagent delegation is expected in the session. This prompt explicitly
authorizes subagent spawning/delegation to comparator, review, verification,
and explorer roles for the package scope; outputs: compact metrics + artifact
paths; write access: read-only except bounded package artifacts for delegated
verification/review notes.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
