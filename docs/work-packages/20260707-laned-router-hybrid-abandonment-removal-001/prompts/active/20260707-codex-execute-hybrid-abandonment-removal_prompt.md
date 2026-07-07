# Codex Kickoff: Execute Hybrid Abandonment Removal (ADR-0037)

Scope: local repository engineering work; flat-file reads/edits in this
worktree plus local git branch creation and local build/test/run commands;
no external systems or network actions are required.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases (A-F) in
`docs/work-packages/20260707-laned-router-hybrid-abandonment-removal-001/package.md`
sequentially through disposition.

## Direction context (new — read first)

The operator has ABANDONED the hybrid implicit-explicit stepper subsystem.
`docs/decisions/0037-abandon-hybrid-implicit-stepping.md` is the decision
record: grounds (evidence-base discount — H2637 is synthetic and was the
only demonstrated win; all real cohort members are non-bare and gain
nothing), the keep-list, and the archive-then-strip mechanics. Standing
direction changes that outlive this package:

- H2637 is demoted to a synthetic stress case. Do not rest future
  performance or promotion claims on H2637-only evidence.
- The Tier-2 mesh package
  (`20260707-laned-router-tier2-mesh-resolution-adjudication-001`) must NOT
  be executed as scaffolded; it will be re-scoped separately to a
  Δx-target adaptive per-OFE cell policy (cells/OFE is not a resolution —
  the cohort spans `Δx 2.6 m` to `30 m` at the same 10-cell setting). Do
  not touch it in this package.
- The historical record stays on main: work-package directories, revision
  histories, and the execution log are protected. Only code, tests, and
  the `SC-OFEROUTE-002` contract are removed.

## Required reading (read before edits)

Core, Conditional, and On-demand tiers per
`docs/work-packages/20260707-laned-router-hybrid-abandonment-removal-001/artifacts/required-reading-map.md`.
Read `SC-OFEROUTE-002.md` BEFORE deleting it — its guard map, test-vector
obligations, and BEI rows are the authoritative strip inventory
(`artifacts/strip-inventory.md`).

Required-reading budget: ~240000 bytes, OK; map:
`artifacts/required-reading-map.md`.

## Files

Write set and protected set per `package.md`. Key operations:

- Branch: create `abandoned/hybrid-implicit-stepping` at the commit
  containing the executed no-harm selector package (commit it first if
  still pending), BEFORE any removal edit; record tip hash.
- Delete: `SC-OFEROUTE-002.md`;
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs`;
  all hybrid tests (Case-4 HYBRID ladder included; the plain Case-4 oracle
  ladder is protected).
- Strip: hybrid composition in `cascade.rs` (cooldown predicate,
  deficit-carry, hour-partition guard); restore single fail-closed
  `run_with_options` in `kinematic_wave.rs`; implicit profile counters;
  selector env/intake plumbing and manifest counters in the runner; hybrid
  fields in both `laned_active.rs` files.
- Amend: `SC-OFEROUTE-001` removal revision (hybrid Branch/Guard,
  test-vector, BEI pointer rows out; changelog cites ADR-0037 + branch);
  registry row -> `withdrawn`; `docs/numerics/` knowledge extraction
  (Phase B, before deletion).

## Task

Execute the package objective end-to-end: archive the final hybrid state,
extract the two numerics knowledge items, remove the hybrid code and
contract from main, prove four-member plain-path byte identity, and close
with dual review/verification and disposition.

## Constraints

- Contract-first sequencing: the `SC-OFEROUTE-001` removal revision and
  registry withdrawal land before the code strip.
- No behavioral edit to any surviving line: this package is mechanical
  removal; plain-path byte identity is the acceptance gate and the reason
  the package exists. If identity fails, HOLD and root-cause; do not
  reconcile by editing plain-path code.
- Decide and record the `OPENWEPP_LANED_ACTIVE_IMPLICIT` posture
  (recommended: typed startup rejection naming ADR-0037; silent-ignore
  must be argued if chosen). Test the chosen posture.
- Protected: all other work-package directories, prior revision-history
  entries, plain Case-4 oracle ladder, `ow-lanuse-1` consumer path,
  `canhgt` publication, explicit-path profile counters, Tier-2 package.
- Reconcile the workspace test-count drop from `1442` against the strip
  inventory: every removed test is named in `artifacts/strip-inventory.md`.
- Exact release-binary provenance (QA-M3 recipe) for both identity runs.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for all
heavy batch/closure runs (full workspace `nextest`, release identity runs,
`cargo deny check`); do NOT run them on the parent model unless the
subagent is unavailable, in which case record command-level evidence.
Standing user authorization for openWEPP subagent delegation is expected in
the session. This prompt explicitly authorizes subagent
spawning/delegation to `comparator_suite_runner`, `rust_code_reviewer`,
`rust_qa_reviewer`, `explorer`, and bounded `worker` roles for release
builds, identity runs, workspace gates, strip-diff review, and bounded
codebase questions; outputs: compact metrics + log paths into package
artifacts; write access: read-only for review/verification/comparator/
explorer, worker bounded to strip-inventory files.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked (identity
gate failure is a hard block by design).

Outputs: update package artifacts and disposition for all completed
phases; add the execution-log closure entry to
`docs/work-packages/README.md`; worker handoff names the Tier-2 re-scope
as the expected first follow-on.
