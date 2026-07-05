# D10 Kickoff Prompt

Scope: local repository science-contract/kernel defect-closure task;
flat-file reads/edits only; no external connectivity required.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260705-mofefid-d10-shock-numerics-gap005-001/package.md`
sequentially through disposition.

Required reading (read before edits):

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/defect_closure_execplans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-d10-shock-numerics-gap005-001/package.md`

Conditional:

- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
  before editing `SC-OFEROUTE-001` or kernel authority.
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
  before editing `SC-OFEROUTE-001`.
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
  before editing `SC-OFEROUTE-001` or kernel authority.
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
  before editing contract registry or profile-bound status.
- `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
  only if execution uses pinned-baseline WEPP provenance.

On-demand:

- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `/home/workdir/openWEPP/docs/planning/mofe-fidelity-campaign-strategy.md`
- `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-d9-dval-disposition-001/artifacts/case4-d10-handoff.md`
- `/home/workdir/openWEPP/docs/work-packages/20260702-mofefid-d8-routing-fidelity-defect-closure-001/artifacts/execution-report.md`
- `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-laned-activation-increment-001/package.md`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/ofe_routing/infiltration.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
- `/home/workdir/openWEPP/tools/dval/compare_dval.py`
- `references/copyrighted/Iwagaki1955_runoff_characteristics_DPRI10.pdf`,
  `references/copyrighted/Papanicolaou2018.md`,
  `references/copyrighted/Papanicolaou2018-supplemental/`, and TVD
  references only when source provenance is needed. Copyright governance
  applies; summarize, do not vendor raw rows or long source excerpts.

Required-reading budget: `334130` local bytes for core + triggered
contract/kernel conditional pre-edit reading, `OK`; map:
`artifacts/required-reading-map.md`.

Files:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260705-mofefid-d10-shock-numerics-gap005-001/**`
- `docs/work-packages/README.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md` only if status changes.
- Conditional D-val / Lane D solver files listed in `package.md`.

Task: close defect `GAP-OFEROUTE-005` end to end. Resolve the Case-4 shock /
resolution numerical-method blocker through contract-first correction, or
record a legitimate primary/source-authority hold. D10 owns Iwagaki Case 4
evidence, convergence criteria, and the real-H2637 resolution-sensitivity
reproduction. D10 does not own Cases 1-3, Zone taxonomy, D11 friction
operands, D12 melt-limb coverage, D13 erosion shape, or D14 activation.

Constraints: contract-first sequencing; canonical `SC-*` authority; typed
guards; no silent defaults; no canonicalize-and-proceed for domain
violations; no production/default activation.

DC closure: close defect `GAP-OFEROUTE-005` end-to-end; do not hold while
source reading, implementation, contract/test work, or validation remains
possible inside the declared envelope. If `HOLD` is claimed, record a HOLD
legitimacy audit naming the boundary, evidence, considered in-envelope
correction route, and why it cannot close now.

No surrogate physics: production code must implement actual contract-backed
or source-authorized numerics; surrogate/provisional/proxy/heuristic stand-ins
are forbidden. Missing or contradictory authority is a hold-for-authority
boundary; known in-scope authority is an implementation obligation.

Real consumer proof: D10 must not claim production activation or routed-water
consumer cutover. If a numerical correction is landed, prove the real
solver/cascade path and D-val/H2637 harness consume it; wrappers, adapters,
shadow-only metrics, or stale compatibility paths cannot carry the closure
claim.

Conservation/output acceptance: record operand lineage before solver/cascade
edits; separate plausible aliases in fixtures; reject known wrong formulas;
run independent reconstruction plus real closure/magnitude audit; align
metadata/schema if any diagnostics are changed. Do not close on one-sided
bounds or self-consistency alone.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for heavy
Case-4/H2637 sweeps, full workspace nextest, and other heavy closure gates
when available; do NOT run them on the parent model unless unavailable, in
which case record command-level evidence. This prompt explicitly authorizes
subagent spawning/delegation to `comparator_suite_runner`, `explorer`,
`rust_code_reviewer`, and `rust_qa_reviewer` for heavy gate execution,
source/harness inspection, read-only review, and verification. Outputs:
compact metrics, findings, log paths, and package-local artifact text. Write
access: read-only unless a later operator grants a bounded write set.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases and
leave no accepted review finding undispositioned.
