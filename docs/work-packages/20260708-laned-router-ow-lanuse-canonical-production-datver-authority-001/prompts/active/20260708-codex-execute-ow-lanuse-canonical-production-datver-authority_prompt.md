Scope: local repository science-contract/work-package task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading:

Core:
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/ROADMAP.md` `## Watershed Runtime Performance Queue`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `docs/work-packages/20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001/package.md`
- `docs/work-packages/20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001/artifacts/final-disposition.md`
- `docs/work-packages/20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001/artifacts/worker-handoff.md`
- `artifacts/required-reading-map.md`

Conditional:
- `docs/specifications/science-contract-authoring-procedure.md` if contract
  profile/schema questions arise.
- `docs/specifications/science-contracts/kernel-process-contract-profile.md` if
  profile checks fail.

Required-reading budget: `390810`, `WARN`; map:
`artifacts/required-reading-map.md`.

Files:
- `docs/work-packages/20260708-laned-router-ow-lanuse-canonical-production-datver-authority-001/**`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`

Task: lock in the operator decision that openWEPP will not project Lane D route
coefficients. Amend authority so `ow-lanuse-1` or later native management files
with complete embedded `routing_coefficients` are required for new-physics Lane
D active/default production, while earlier datvers remain deprecated
compatibility/validation/rollback inputs on legacy/off paths.

Constraints: contract-first sequencing; canonical `SC-*` authority; typed
guards; no silent defaults; no optional sidecar authority; no legacy-field
projection; no runtime implementation in this package.

Subagent requirement: none for heavy batch/closure/comparator runs. This prompt
explicitly authorizes subagent spawning/delegation to read-only review and
verification roles for authority consistency checks; outputs:
`artifacts/review-*.md` and `artifacts/verification-*.md`; write access:
read-only unless operator expands scope.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
