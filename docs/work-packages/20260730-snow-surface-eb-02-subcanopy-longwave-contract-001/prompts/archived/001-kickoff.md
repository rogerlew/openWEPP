# SNOW-SURFACE-EB-02 Kickoff

Status: `completed / archived`.

Scope: local repository science-contract work; flat-file reads/edits only; no
external-system mutation is required.

Execution mode: `package-end-to-end`.

Phase plan: execute all phases in `package.md` sequentially through
disposition.

## Required Reading

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- package-local `package.md`

Conditional because canonical contract authority is edited:

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contract-spec.md`
- `docs/specifications/unit-governance.md`

On demand:

- `SC-SNOWFREEZE-001`
- `SC-PLANT-001`
- EB-01 and EB-01A package decisions
- cited primary-source bytes

Required-reading budget: `WARN`; the mandatory local set exceeds `400000`
bytes primarily because the work-package catalog is large, but remains below
the `800000` justification threshold. Map:
`artifacts/required-reading-map.md`.

## Task

Execute the package objective end-to-end for the declared write set.

Constraints: contract-first sequencing; canonical `SC-*` authority; typed
guards; no silent defaults; no canonicalize-and-proceed for material domain
violations.

No surrogate physics: canonical authority may contain only source-backed or
physically derived equations. Production runtime edits are prohibited in this
increment.

Conservation/output acceptance: no runtime conservation output is changed.
Analytical vectors must nevertheless reconstruct every radiative equation
independently and distinguish rejected direct-alias/double-count candidates.

Subagent authorization: this prompt explicitly authorizes subagent
spawning/delegation to two independent contract reviewers and two independent
terminal verifiers. Outputs are the four assigned package-local artifacts;
write access is bounded to each assigned artifact.

Subagent requirement: none for heavy batch execution.

Autonomy: execute through disposition without requesting user intervention
unless the declared stop-loss is reached.
