# SNOWDENSITY-10.3.10 Spring Pack-Depletion and Compaction Adjudication

Status: complete
Owner: Codex
Date: 2026-06-27

## Objective

Adjudicate whether the remaining March/April over-persistent snow-depth failures
after SNOWDENSITY-10.3.8 are physically closable by compaction under the
contract snow-density cap, or whether they require a spring pack-depletion /
patchy-meltout process before additional compaction work is justified.

## Context

SNOWDENSITY-10.3.9 found that March/April accounts for `282/761` remaining paired
failures and that depth-only over-persistence, density/compaction, and patchy
meltout/depletion dominate. That package intentionally did not resolve whether
the modeled SWE can fit the observed depth under a physically valid density. This
package performs that feasibility check using the existing `SC-SNOWFREEZE-001`
`522 kg m^-3` upper density cap.

## Required Reading

- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/planning/snow-frost-fidelity-strategy.md` §10.3
- `docs/work-packages/20260627-snowdensity-10-3-9-march-april-residual-attribution-001/package.md`
- `docs/work-packages/20260627-snowdensity-10-3-9-march-april-residual-attribution-001/artifacts/march-april-residual-attribution.json`
- `docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001/artifacts/liquid-holding-capacity-coupled-wat.json`
- `tools/snowfreeze_observed/march_april_residual_attribution.py`
- `tools/snowfreeze_observed/observed_harness.py`

## Scope

In scope:

- Add a diagnostic-only spring pack-depletion and compaction adjudication tool.
- Reuse the SNOWDENSITY-10.3.8 coupled WAT candidate
  `coe_liquid_holding_capacity_v1` and the installed paired snow-depth
  observations.
- For each March/April residual failure, compute whether the modeled SWE can be
  reconciled with observed snow depth by increasing density only, bounded by the
  `522 kg m^-3` `SC-SNOWFREEZE-001` density cap.
- Estimate the row-level SWE mass that would have to be depleted when even the
  density cap cannot fit the observed depth.
- Separate compaction-only-feasible rows from cap-limited depletion rows,
  patchy-meltout rows, and under-persistence rows.
- Produce JSON and Markdown artifacts with the next one-lever disposition.
- Add a focused guard test for diagnostic confinement and artifact schema.

Out of scope:

- Production snow/frost physics changes.
- New selectors, defaults, parser/runfile/user surfaces, or public WAT schemas.
- Fixture input changes or site-specific tuning.
- Activation/retirement decisions for existing opt-in candidates.
- A new density cap or production compaction formula.
- Treating observation-blocked surfaces as verdict-bearing.

## Closure Gates

Closure may be `complete` only if:

- The tool consumes the 10.3.8 coupled WAT candidate baseline and March/April
  paired observation surfaces.
- The report explicitly uses `522 kg m^-3` as the existing contract cap and does
  not introduce a new cap.
- The report quantifies compaction-only-feasible, cap-limited-depletion,
  patchy-meltout/depletion, and under-persistence rows.
- The report recommends either compaction-first or depletion-first based on the
  feasibility counts.
- Observation-blocked surfaces remain non-verdict.
- Diagnostic boundaries are preserved: no runtime production physics, defaults,
  selectors, fixtures, schema, coefficients, radiation, canopy, phase partition,
  density, melt, rain heat, longwave, or frost code changes.
- Focused gates pass:
  - `.venv/bin/python tools/snowfreeze_observed/spring_pack_depletion_compaction_adjudication.py`
  - `cargo fmt --check`
  - `cargo test --test snowdensity10_3_10_spring_pack_depletion_compaction`
  - `cargo clippy --test snowdensity10_3_10_spring_pack_depletion_compaction -- -D warnings`

## Status Log

- 2026-06-27: Scaffolded diagnostic adjudication package.
- 2026-06-27: Added diagnostic feasibility tool and focused guard test.
- 2026-06-27: Generated cap-feasibility report from the SNOWDENSITY-10.3.8
  coupled WAT candidate and March/April paired observations.
- 2026-06-27: Closed complete with `SPRING-COMPACTION-FIRST`: `190/282`
  March/April failures are compaction-only feasible within the existing
  `522 kg m^-3` cap; `49/282` require depletion or patchy meltout even at the
  cap; `43/282` are under-persistence rows.
