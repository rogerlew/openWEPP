# Kickoff: SNOWDENSITY-10.3.17 Shallow-Pack Compaction Guard

Execution mode: package-end-to-end

Autonomy: Execute all package phases through disposition without additional
user intervention unless a hard blocker is reached. If any current-scope gate
fails or cannot be run, close `HOLD` or non-promotion with the blocker named;
do not activate the candidate.

Required reading:

Core:

- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260627-snowdensity-10-3-17-shallow-pack-compaction-guard-001/package.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/planning/snow-frost-fidelity-strategy.md` section 10.2 item 7 and section 10.3
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`

Conditional:

- `docs/standards/kernel-work-package-preparation.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`

On-demand:

- `references/copyrighted/noaa_6392_DS1.md`
- `references/copyrighted/marks1999.pdf`
- `references/copyrighted/source_pdfs/marks1998.pdf`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `tools/snowfreeze_observed/*`

Required-reading budget: see `artifacts/required-reading-map.md`.

Implement contract-first:

- Add opt-in `physics_bulk_shallow_guard_v1`.
- Keep activated default and explicit rollback intact.
- Reduce only shallow-pack density aggressiveness under authority-derived
  `0.25 m` depth threshold.
- Run real coupled WAT/trace diagnostics.
- Close truthfully as promotion-eligible, HOLD, or non-promotion.

Conservation/output acceptance: record operand lineage, protected output-schema
status, independent trace reconstruction of snow-state SWE/depth-density
closure, and explicit rejection of routed-liquid/mass-removal aliases.

Subagent authorization: this prompt explicitly authorizes subagent
spawning/delegation to read-only review and verification roles for the scope
listed in `package.md`; expected outputs are the package review and
verification artifacts.
