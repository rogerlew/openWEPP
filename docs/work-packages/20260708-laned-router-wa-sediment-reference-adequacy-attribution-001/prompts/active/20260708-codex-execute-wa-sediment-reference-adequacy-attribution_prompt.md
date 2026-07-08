# Codex Kickoff - WA Sediment Reference Adequacy Attribution

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
- `docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001/package.md`
- `docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001/artifacts/required-reading-map.md`
- `docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/artifacts/mesh-policy-ratification.md`

Conditional:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
  rev-43 active mesh-policy surfaces and routed-hydrograph erosion shape.
- `docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/artifacts/mesh-policy-ratification.json`
  for exact comparator values.
- `docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/artifacts/coupled-spacetime-summary.json`
  for run provenance and aggregate surface deltas.

On-demand:

- D13 routed-hydrograph erosion-shape artifacts.
- D15A active-owner consumer proof artifacts.

Required-reading budget: core `48624` bytes, `OK`; map:
`artifacts/required-reading-map.md`.

Files:

- `docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

Task: attribute the `wa_cascades_forest_h1` refined-75 fine-reference annual
pass-sediment adequacy miss (`dx2p5_dt75` versus `dx1p25_dt75`, `tdep:4`) and
classify the mechanism before any renewed `dx5` production promotion.

Constraints: contract-first sequencing; canonical `SC-*` authority; typed
guards; no silent defaults; no production mesh-policy flip; no tolerance
widening to fit the miss; no surrogate physics.

Conservation/output acceptance: annual pass-sediment is a published
conservation-sensitive output. Record operand lineage, daily contributors,
independent routed-water trace comparisons, and exact artifact hashes. Do not
close on self-consistency alone.

Subagent requirement: this prompt explicitly authorizes subagent
spawning/delegation to review and verification roles for read-only package
review and replay checking. Outputs: compact findings written to
`artifacts/review-*.md` and `artifacts/verification-*.md`. Write access:
bounded to this package's artifact directory.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts, disposition, final-disposition, and worker
handoff.
