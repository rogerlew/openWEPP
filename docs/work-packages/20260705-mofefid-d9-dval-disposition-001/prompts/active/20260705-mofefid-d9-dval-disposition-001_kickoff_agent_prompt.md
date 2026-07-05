# D9 Kickoff Prompt

Scope: local repository science-contract/kernel validation task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260705-mofefid-d9-dval-disposition-001/package.md`
sequentially through disposition.

Required reading (read before edits):

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-d9-dval-disposition-001/package.md`

Conditional:

- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
  when editing `SC-OFEROUTE-001` or kernel authority.
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
  when editing `SC-OFEROUTE-001`.
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
  when editing `SC-OFEROUTE-001`.
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
  when editing contract registry or profile-bound status.

On-demand:

- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `/home/workdir/openWEPP/docs/planning/mofe-fidelity-campaign-strategy.md`
- `/home/workdir/openWEPP/docs/work-packages/20260702-mofefid-d8-routing-fidelity-defect-closure-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-laned-seam-implementation-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-laned-activation-increment-001/package.md`
- `/home/workdir/openWEPP/docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md`
- `references/copyrighted/Papanicolaou2018.md` and supplemental-derived
  fixture documentation only when D-val source provenance is needed.

Required-reading budget: recorded in
`artifacts/required-reading-map.md`; threshold outcome `OK` for core +
triggered conditional pre-edit reading at scaffold time.

Files:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260705-mofefid-d9-dval-disposition-001/**`
- `docs/work-packages/README.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md` only if status changes.
- Conditional D-val harness/test files listed in `package.md`.

Task: execute the D9 objective end to end. Close the non-numerics
`INV-OFEROUTE-011` D-val acceptance surface: Cases 1-3 after D8, Zone
1/Zone 2 taxonomy, and exact Case-4 handoff to D10 / `GAP-OFEROUTE-005`.

Constraints: contract-first sequencing; canonical `SC-*` authority; typed
guards; no silent defaults; no canonicalize-and-proceed for domain
violations; no surrogate/provisional/proxy/heuristic process physics.

No surrogate physics: production code must not add tuned or placeholder
physics. D9 is validation/adjudication; any production physics correction
outside this package's authority must become a named follow-on or hold.

Real consumer proof: D9 must not claim production activation or runtime
consumer cutover. If any consumer-facing claim is added, prove the real
consumer reads the new path and prove old shadow/diagnostic paths are not
carrying the claim.

Conservation/output acceptance: D9 is not expected to create publication
surfaces. If execution adds or changes any conservation-sensitive diagnostic
or output aggregate, first record operand lineage, reject alias formulas, run
independent reconstruction/closure, and align metadata/schema.

Subagent requirement: REQUIRED for heavy D-val/comparator-style batches and
full closure gates when available. This prompt explicitly authorizes subagent
spawning/delegation to `comparator_suite_runner`, `rust_code_reviewer`,
`rust_qa_reviewer`, and `explorer` for read-only review, verification,
source/harness inspection, and heavy gate execution. Outputs: compact metrics,
findings, log paths, and package-local artifact text. Write access:
read-only unless a later operator explicitly grants a bounded write set.

Autonomy: execute package phases end to end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases and
leave no accepted review finding undispositioned.
