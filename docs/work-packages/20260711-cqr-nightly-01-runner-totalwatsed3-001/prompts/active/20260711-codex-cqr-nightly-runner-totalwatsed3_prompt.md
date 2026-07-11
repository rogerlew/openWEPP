# CQR Nightly Batch 01 Target 01 Kickoff

Scope: local behavior-preserving CQR in `/home/workdir/openWEPP`; flat-file
reads/edits only; no external systems or network actions.

Execution mode: package-end-to-end.

Required reading:

- Core: root/crate/test/work-package guidance, this package and reading map,
  CQR ExecPlan, mechanical/CQR guides, ADR-0021, prompt wording guide, target,
  and focused CLI contract test named in `package.md`.
- Conditional: science-contract guidance and relevant `SC-*` only if authority
  surfaces are implicated; local-CI guide if iteration gates are narrowed.
- On-demand: adjacent runner/output modules used by touched mechanisms.

Required-reading budget: `155786` local bytes before package/map additions,
`OK` (`<=400000`); map: `artifacts/required-reading-map.md`.

Files: only the intended write set in `package.md`.

Task: execute all package phases sequentially through disposition. Preserve
aggregation, numeric ordering, row-read ordering, typed errors, schema, units,
and output identity. Do not alter science/publication authority or semantics.

Subagent requirement: REQUIRED. Spawn `comparator_suite_runner` for all heavy
batch/closure/comparator runs; do not run them on the parent model unless the
subagent is unavailable and command-level evidence records that fact. This
prompt explicitly authorizes subagent spawning/delegation to comparator,
closure-runner, review, and verification roles for metric checks, focused/full
gates, output identity, dual review, and dual verification. Outputs: compact
metrics, verdicts, logs, and artifact paths. Write access: read-only unless a
bounded intended-write-set fix is explicitly assigned.

Autonomy: execute end-to-end without requesting direction unless hard-blocked.
Local holds must roll implementation/test edits back to the scaffold, preserve
and commit evidence, then allow the batch to continue. Global holds stop it.

Outputs: update every required artifact, disposition every finding, and leave
the package ready for its completion or hold commit.
