# SNOWDENSITY-10.3.17 Shallow-Pack Compaction Guard

Status: complete - non-promotion
Package id: `20260627-snowdensity-10-3-17-shallow-pack-compaction-guard-001`
Owner: Codex
Execution mode: package-end-to-end

## Objective

Add and adjudicate an opt-in snow-density candidate whose only algorithmic delta
from `physics_bulk_density_compaction_v1` is reduced densification
aggressiveness for shallow packs. The candidate targets the density-arm-induced
under-persistence tail identified by SNOWDENSITY-10.3.13, especially
`harvard_hardwood`, while preserving the activated default bundle and rollback.

## Rationale

SNOWDENSITY-10.3.13 showed that `177/234` bundle under-persistence rows were
induced by the density arm, not inherited from holding-capacity-only behavior.
SNOWDENSITY-10.3.11 and 10.3.16 showed that additional mass-removal or
densification levers fail the bidirectional guardrail. The binding residual is
now the density-arm-induced under-persistence tail from shallow-pack
over-densification: the current density arm can reduce depth
too aggressively after SWE is conserved. A shallow-pack guard is the narrowest
one-lever candidate that can recover under-persistence without adding mass loss,
changing the cap, or re-opening melt physics.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260627-snowdensity-10-3-17-shallow-pack-compaction-guard-001/package.md`
- `docs/planning/snow-frost-fidelity-strategy.md` section 10.2 item 7 and section 10.3
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`

Conditional:

- `docs/codex_exec_plans.md`
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
- prior packages `10.3.11`, `10.3.13`, `10.3.16`

## Authority Envelope

- Canonical contract authority: `SC-SNOWFREEZE-001`, amended before production
  edits.
- Physical authority: Anderson 1976 compaction/metamorphism lineage plus
  Marks/SNOBAL shallow-pack structure precedent. The authority threshold is the
  SNOBAL active surface layer maximum depth, approximately `0.25 m`, already
  recorded in the planning strategy from Marks/SNOBAL paper evidence.
- Licensing guard: `deny.toml` allow-lists permissive licenses and excludes GPL,
  AGPL, and LGPL by absence. PySnobal/libsnobal C source is not implementation
  authority for this package unless a non-GPL-family license is confirmed before
  reading it. This package uses clean-room contract/paper-derived authority.

## Included Scope

- Amend `SC-SNOWFREEZE-001` for opt-in
  `physics_bulk_shallow_guard_v1`.
- Add contract-derived tests for selector isolation, shallow-depth reduced
  densification, SWE conservation, density cap preservation, and boundary scans.
- Implement the opt-in density candidate in the direct runtime only.
- Add a coupled direct-production WAT/trace diagnostic over the observed
  snow-depth fixture ladder.
- Record under-persistence recovery, over-persistence non-worsening,
  authority-derived threshold, conservation closure, reviews, verification, and
  final disposition.

## Excluded Scope

- No default activation.
- No density-cap change.
- No output-schema, fixture, parser, runfile, user CLI, compatibility-runtime,
  Qwet/frzftp, frost, phase, canopy, radiation, longwave, rain-heat, melt, or
  ablation change.
- No observed snow-depth/density, site identity, residual class, or tolerance
  input to runtime calculation.
- No spring wet-compaction acceleration, two-layer snow-surface thermal model,
  sublimation, or other mass-removal lever.
- No frost attribution clearance.

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/planning/snow-frost-fidelity-strategy.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260627-snowdensity-10-3-17-shallow-pack-compaction-guard-001/**`
- `Cargo.toml`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/hillslope/snowbench_coe_density.rs`
- `crates/openwepp-runner/src/hillslope/snowbench_physics_bulk.rs`
- `tests/integration/snowdensity10_3_17_shallow_pack_compaction_guard.rs`
- `tools/snowfreeze_observed/shallow_pack_compaction_guard.py`

## Phase Plan

1. Scaffold package and required evidence placeholders.
2. Amend `SC-SNOWFREEZE-001` contract-first with selector, invariant,
   obligation, guard threshold, addendum, protected boundaries, and changelog.
3. Add contract-derived tests and pre-implementation contract evidence.
4. Implement the opt-in density model and direct-production selector.
5. Add and run coupled WAT/trace diagnostic evidence.
6. Run focused tests and available closure gates, then complete dual reviews,
   finding disposition, verification, line-count governance, and package
   disposition.

## Exit Criteria

Current-scope gates:

- Contract version is bumped and binds `physics_bulk_shallow_guard_v1`.
- Absent/empty direct-production selectors still choose
  `coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1`.
- Explicit rollback still accepts `legacy_coe` and `legacy_wepp`.
- Unknown selectors fail closed.
- Candidate only reduces shallow-pack densification aggressiveness relative to
  `physics_bulk_density_compaction_v1`; it does not change SWE, melt/liquid
  terms, cap, constants outside the guard, or public output schema.
- Shallow threshold is authority-derived from the `0.25 m` active surface-layer
  depth and is not fitted to fixtures.
- Real coupled direct-production WAT/trace evidence proves the candidate reached
  the snow partition.
- Gate 1: candidate cuts the induced under-persistence tail, with
  `harvard_hardwood` reported explicitly.
- Gate 2: candidate does not worsen the over-persistence tail.
- Gate 3: whole-model snow-state conservation closes.
- Protected boundaries remain unchanged.
- Focused and workspace gates are run or the package closes `HOLD` with the
  missing gate named:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

If any current-scope gate is missing, worse, or fails, close `HOLD` or
non-promotion. Do not activate the candidate.

## Execution Result

Final disposition: `NON-PROMOTION-SHALLOW-GUARD-GATE-NOT-MET`.

The opt-in selector and shallow-pack density guard were implemented and reached
the real direct-production WAT path, but the coupled evidence did not meet the
package gates. The candidate reduced induced under-persistence only `177 -> 176`,
recovered `0` induced-under rows at `harvard_hardwood`, worsened
over-persistence `264 -> 267` with `3` new over rows from non-over rows, and
changed downstream snow mass terms (`max_abs_mass_term_delta_m =
3.3417423040965196e-3`). Local SWE-depth-density identity closed, but the
whole-model guardrail failed. The candidate remains opt-in diagnostic only; no
default activation is authorized.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to two read-only review subagents and two read-only verification subagents for
implementation, evidence, gate-legitimacy, and protected-boundary review.
Expected outputs are `artifacts/review_agent_a.md`,
`artifacts/review_agent_b.md`, `artifacts/verification_agent_a.md`, and
`artifacts/verification_agent_b.md`; write access is read-only. Local
Codex-authored independent review artifacts may substitute when subagent
dispatch is unavailable.

## Security / Safety Gate

Unknown selector values must fail closed. The runtime calculation must not read
observations, fixture identity, site metadata, residual class, or tolerance.
No GPL-family source may be read or ported without permissive-license
confirmation. The candidate is opt-in diagnostic evidence only unless all gates
pass in this package and a later package explicitly authorizes activation.
