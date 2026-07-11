# Kickoff — FDIR finite-value guard closure

Execution mode: defect-closure ExecPlan, end-to-end
Autonomy: close defect `FDIR-FINITE-VALUE-GUARD-001` through terminal
disposition without asking for next steps unless a declared hard boundary is
proven.

Read the following before edits.

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260711-dc-fdir-finite-value-guard-001/package.md`

Conditional (applicable):

- `docs/defect_closure_execplans.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

On-demand for the touched mechanism:

- `docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md`
- `docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md`
- pinned legacy files listed in `package.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- the FQ-01 queue plan and prior nightly hold package

Required-reading budget: `554345` local bytes, `WARN`; justified and itemized in
`artifacts/required-reading-map.md`.

Close defect `FDIR-FINITE-VALUE-GUARD-001` end-to-end within the Correction
Authority Envelope. Amend contracts, add failing contract tests, and pass the
pre-implementation gate before production edits. Reject invalid state with the
typed contract error; no silent defaults, clamping, or canonicalize-and-proceed.
Complete science-tier coverage, A-H obligation binding, eligible CRAP closure,
full gates, dual review/disposition, and dual verification in this package.

HOLD legitimacy audit: a hold must name a declared boundary, cite proof, list
the in-envelope correction route considered, and explain why it cannot close
the defect. Do not hold while source reading, implementation, or validation is
possible in-envelope.

Subagent authorization: this prompt explicitly authorizes subagent
spawning/delegation to independent contract/technical reviewers, verification
agents, and heavy gate/coverage runners. Review/verification is read-only;
heavy runners may write only their named package artifact and build outputs.
