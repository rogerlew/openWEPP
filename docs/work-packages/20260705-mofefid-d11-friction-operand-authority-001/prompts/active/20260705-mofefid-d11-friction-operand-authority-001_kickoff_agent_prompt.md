# D11 Kickoff Prompt

Scope: local repository science-contract/kernel authority task; flat-file
reads/edits only; no external connectivity required.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260705-mofefid-d11-friction-operand-authority-001/package.md`
sequentially through disposition.

Required reading (read before edits):

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-d11-friction-operand-authority-001/package.md`

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
- `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-d10-shock-numerics-gap005-001/artifacts/worker-handoff.md`
- `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-laned-activation-increment-001/package.md`
- `/home/workdir/openWEPP/crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- `/home/workdir/openWEPP/crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/ofe_routing/`
- Management, plant, residue, growth, canopy, rainfall, and friction
  contracts/specs only when a candidate operand source touches them.
- Copyrighted Lane D friction references only when source provenance is
  needed. Copyright governance applies; summarize, do not vendor long source
  excerpts or raw tables.

Required-reading budget: `307265` local bytes for core + triggered
contract/kernel conditional pre-edit reading, `OK`; map:
`artifacts/required-reading-map.md`.

Files:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260705-mofefid-d11-friction-operand-authority-001/**`
- `docs/work-packages/README.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md` only if status changes.
- Conditional Lane D shadow/builder/test files listed in `package.md`.

Task: execute D11 end to end. Close `GAP-OFEROUTE-007` by sourcing,
default-authorizing, or fail-closing every active friction operand
(`I`, `k_o`, `C_d`, `D_r`, `lambda`, `LAI`, `h_c`) and wiring the
active/shadow operand builder when authority permits.

Constraints: contract-first sequencing; canonical `SC-*` authority; typed
guards; no silent defaults; no canonicalize-and-proceed for domain violations;
no production/default activation.

No surrogate physics: production or shadow/active candidate code must not add
tuned, provisional, proxy, or heuristic friction values. Missing authority is
a hold/fail-closed boundary; known in-scope authority is an implementation
obligation.

Real consumer proof: if D11 claims the active friction operand surface is
closed, prove the real Lane D shadow/active candidate consumes the builder and
the old hardcoded all-lane `k_o = 500` / `I = 0` path is not carrying the
claim. Producer-only evidence cannot close the builder claim.

Case-4 boundary: D11 must not close `GAP-OFEROUTE-005`, tune `k_o` to Case 4,
or treat Iwagaki Case 4 as accepted. Any Manning-`n` / `k_o` source finding is
handoff evidence for the D10 follow-on unless the canonical D10 source-authority
surface is explicitly reopened in a separate package.

Conservation/output acceptance: record operand lineage before builder/consumer
edits; separate plausible aliases in fixtures; reject known wrong formulas;
run independent routed-path closure or sanity evidence; align metadata/schema
if any diagnostics change. Do not close on self-consistency alone.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for full
workspace nextest, H2637/fixture batches, and other heavy closure gates when
available; do NOT run them on the parent model unless unavailable, in which
case record command-level evidence. This prompt explicitly authorizes subagent
spawning/delegation to `comparator_suite_runner`, `explorer`,
`rust_code_reviewer`, and `rust_qa_reviewer` for heavy gate execution,
source/authority inspection, read-only review, and verification. Outputs:
compact metrics, findings, log paths, and package-local artifact text. Write
access: read-only unless a later operator grants a bounded write set.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases and
leave no accepted review finding undispositioned.
