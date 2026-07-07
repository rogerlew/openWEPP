# Codex Kickoff: Execute Tier-2 dx-Target Mesh-Policy Re-scope

Scope: local repository science-contract/kernel adjudication task; flat-file
reads/edits in this worktree plus local build/test/run commands; no external
systems or network actions are required.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases (T2R-A through T2R-H) in
`docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/package.md`
sequentially through disposition.

## Required reading

Read the package-local authority map first:
`docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/required-reading-map.md`.

Core:
- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/package.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/required-reading-map.md`
- `docs/decisions/0037-abandon-hybrid-implicit-stepping.md`
- `docs/work-packages/20260707-laned-router-hybrid-abandonment-removal-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260707-laned-router-tier2-mesh-resolution-adjudication-001/package.md`
- `docs/work-packages/20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/artifacts/selected-cohort-materialization.json`
- `docs/work-packages/20260707-laned-router-hybrid-abandonment-removal-001/artifacts/plain-identity-materialization.json`
- `docs/backlog/20260706-laned-router-numerics-performance-tiers.md`

Conditional:
- `docs/specifications/science-contracts/AGENTS.md` before contract,
  contract-derived test, or kernel semantic edits.
- `docs/specifications/science-contract-authoring-procedure.md` before
  changing canonical `SC-*` text.
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  before changing canonical invariant, BEI, or profile text.
- `docs/specifications/science-contracts/index.md` before registry edits.
- `crates/AGENTS.md` before Rust edits.
- `tests/AGENTS.md` before test edits.

On-demand:
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
  when proposing or implementing mesh-policy authority.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
  when inventorying or editing active runtime mesh construction.
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/` when running or
  editing Case-4/router mesh ladders.
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/`
  when editing runner active-route projection.

Required-reading budget: 350747 bytes, OK; map:
`artifacts/required-reading-map.md`. Recalculate and update the map before
implementation edits if the required-reading list changes.

## Task

Execute the package objective end-to-end: supersede the old hybrid-era Tier-2
package, inventory current fixed-cell mesh behavior, adjudicate target-`dx`
mesh policies on dimensionless Case-4 convergence evidence plus
selected-cohort active-plain self-convergence evidence, and land a production
mesh-policy change only if contract-first authority and all required gates
pass.

## Constraints

- Contract-first sequencing: amend/propose `SC-OFEROUTE-001` mesh-policy
  tolerance and guard authority before contract-derived tests or production
  mesh-policy edits.
- No hybrid revival: do not restore hybrid code, selectors, tests, or
  `SC-OFEROUTE-002`; ADR-0037 controls.
- H2637 is synthetic stress only. Report it separately from real-cohort
  evidence and do not use it as fleet-general proof.
- Error basis: measure candidate and current-baseline mesh errors against an
  adequate fine reference. Never use the current fixed `10 cells/OFE` baseline
  as truth. The package may conclude that the current baseline is inadequate
  and that the ratified policy costs more on some real members.
- Fine-reference adequacy: one further halving of target `dx` must move every
  judged surface by no more than one third of that surface's predeclared
  tolerance. Refine and repeat, or hold, if that rule fails.
- Case-4 role: use Case-4 only as a dimensionless cells-per-reach convergence
  and shock-regime machinery check. Do not interpret Case-4 at absolute
  candidate `dx` as candidate acceptance or rejection evidence.
- T2R-C surface predeclaration: include per-day routed outlet mass,
  hourly-weight D13 erosion-consumer shape, annual pass-sediment sums,
  conservation closure residuals, `routed_end_window_storage_m3`,
  `routed_tail_fold_m3`, `lane_days_erosion_source_shape_degenerate`, and
  `days_uniform_shape`.
- Clamp and stencil regime: justify `min_cells` as a TVD-MacCormack
  scheme-regime constraint, include a short-OFE floor rung, and do not inherit
  the internal one-cell mesh clamp as policy behavior.
- Shadow lane: decide contract-first whether the shadow lane follows the
  active production mesh policy or remains fixed; include
  `crates/openwepp-runner/src/hillslope/laned_shadow.rs` if production mesh
  policy changes and shadow follows.
- Time-step caps: hold `LANED_ACTIVE_SAMPLE_DT_S = 900` and
  `LANED_ACTIVE_MAX_DT_S = 300` fixed across mesh ladders. Do not co-tune
  `dt` caps in this package, and record timing as measured rather than
  assuming quadratic savings.
- No surrogate physics: production code must implement actual
  contract-backed or baseline-authoritative physics; surrogate, provisional,
  proxy, empirical stand-in, or heuristic process-physics substitutions are
  forbidden.
- No silent defaults, unbounded clamping, or canonicalize-and-proceed domain
  behavior. Use typed fail-closed errors/guards unless bounded normalization is
  explicitly contract-authorized.
- Gate evidence non-deferral: a phase may be marked complete only when all of
  its own required gates have direct current evidence. If required evidence
  cannot be produced in-envelope, stop at `EXECUTED-HOLD-*` with a package
  hold legitimacy audit.

Real consumer proof: if production active routing changes, prove the active
routed path's downstream consumers read the new mesh-policy path and prove
wrappers, adapters, skeletons, shadow paths, and old compatibility paths are
not carrying the closure claim.

Conservation/output acceptance: record operand lineage for any changed
conservation-sensitive output; separate plausible aliases in fixtures; reject
known wrong formulas; run independent reconstruction plus real
closure/magnitude audit; align metadata/schema; do not close on one-sided
bounds or self-consistency alone.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for heavy
comparator/timing/full closure runs (release timing, selected-cohort batches,
Case-4 ladders, full workspace gates); do NOT run them on the parent model
unless the subagent is unavailable, in which case record command-level
evidence. Standing user authorization for openWEPP subagent delegation is
expected in the session. This prompt explicitly authorizes subagent
spawning/delegation to `comparator_suite_runner`, `rust_code_reviewer`,
`rust_qa_reviewer`, `explorer`, and bounded `worker` roles for the scopes named
in `package.md`; outputs: compact metrics plus log/artifact paths into package
artifacts; write access: read-only for review/verification/comparator/explorer,
worker bounded to package artifacts unless the parent explicitly assigns a
disjoint implementation write set.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts and disposition for all completed phases.
