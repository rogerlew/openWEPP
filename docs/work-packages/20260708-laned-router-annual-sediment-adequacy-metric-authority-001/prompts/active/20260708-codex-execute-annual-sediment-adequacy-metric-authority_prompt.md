# Codex Kickoff - Annual Sediment Adequacy Metric Authority

Scope: local repository science-contract/kernel evidence task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260708-laned-router-annual-sediment-adequacy-metric-authority-001/package.md`
- `docs/work-packages/20260708-laned-router-annual-sediment-adequacy-metric-authority-001/artifacts/required-reading-map.md`
- `docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001/artifacts/wa-sediment-attribution.md`
- `docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001/artifacts/classification.md`

Conditional:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
  rev-43 active mesh-policy judged surfaces and change log.
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
  routed-hydrograph shape consumer surfaces.
- Prior coupled space-time summary JSON and raw pass parquets when replaying
  evidence.

Required-reading budget: core `52622` bytes, `OK`; map:
`artifacts/required-reading-map.md`.

Files:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260708-laned-router-annual-sediment-adequacy-metric-authority-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

Task: decide and implement contract-first annual pass-sediment mesh-policy
adequacy metric authority, then replay selected real-cohort annual sediment
comparisons under the decided rule.

Constraints: contract-first sequencing; canonical `SC-*` authority; no
tolerance widening to fit the WA observed value; no production `dx5` flip; no
sediment process-physics change; no silent defaults.

Conservation/output acceptance: annual pass-sediment is a conservation-sensitive
published output surface. Record operand lineage, rejected formulas, exact
denominators, selected-cohort replay, and review/verification. Do not close on
one-sided bounds or self-consistency alone.

Subagent requirement: this prompt explicitly authorizes subagent
spawning/delegation to review and verification roles for read-only package
review and replay checking. Outputs: compact findings written to
`artifacts/review-*.md` and `artifacts/verification-*.md`. Write access:
bounded to this package's artifact directory.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts, disposition, final-disposition, and worker
handoff.
