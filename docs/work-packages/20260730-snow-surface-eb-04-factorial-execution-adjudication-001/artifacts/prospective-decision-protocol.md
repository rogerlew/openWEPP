# Prospective Decision Protocol

Status: `frozen before result execution`

Evidence mode: `Static`

Frozen at: `2026-07-30`

## Matrix

The fixed validation lanes are five SNOTEL open controls; Harvard open and
hardwood; Marcell open, deciduous, and conifer; and the Sleepers South open and
Sleepers W9 hardwood diagnostic frost lanes. Every lane uses `B`, `L`, `S`,
and `LS` with the EB-03A carrier and unchanged forcing/fixture inputs.

`B` disables both experimental mechanisms; `L` enables only sub-canopy
longwave; `S` enables only energy-consistent vapor/latent exchange; `LS`
enables both. Density, phase, albedo, CoE melt/rain, liquid routing, canopy,
frost, and numeric policy remain identical within a lane.

## Physical Gates

- mass residual: maximum absolute daily snow-state residual
  `<= 1e-9 m` water equivalent;
- energy residual: maximum absolute Stage 3 residual
  `<= 1e-6 J m^-2`;
- latent/mass identity: maximum absolute residual `<= 1e-6 J m^-2`;
- thermal bounds: every populated snow temperature is `(-273.15, 0] deg C`;
- trace identity: fixture bytes and every non-target selector are identical
  within a lane;
- real-consumer proof: both WAT parquet and snow trace exist and are consumed.

Any failure rejects that candidate cell before scientific scoring. A failed
physical gate cannot be offset by observation agreement.

## State And Timing Operators

- SWE and depth use exact-date observation/model pairs from the installed
  normalized datasets.
- The canonical `INV-SNOWFREEZE-050` rubric supplies the primary ordinal
  forcing-robust scores: `fail=0`, `marginal=1`, `pass=2`, `strong=3`.
- Snow-present is `SWE > 1 mm` water equivalent. Snow disappearance is the
  first day at or below `1 mm` followed by seven consecutive days at or below
  `1 mm`. This threshold is above floating noise and the seven-day persistence
  window avoids classifying transient melt/refreeze gaps as seasonal loss.
- Each runoff timing window is water-year October 1 through the last persistent
  snow-disappearance day; if persistence is unresolved, it ends September 30.
  The operator reports the liquid-outflow centroid and peak day from the same
  WAT field and identical window for all cells.
- Ties in dates use the earliest day. Missing observations remain unavailable;
  they are not imputed.

## Primary Promotion Rule

Only `LS` is eligible to advance as the combined candidate. `L` and `S`
marginals diagnose attribution.

LS earns `GO_TO_EB05` only if all are true:

1. every LS physical gate passes;
2. across independent-validation lanes, LS increases the sum of
   forcing-robust ordinal scores relative to B by at least one;
3. LS reduces the total forcing-robust `fail` count relative to B;
4. no independent-validation lane gains a new forcing-robust `fail`;
5. neither open-control nor canopy-stratum protected groups has a lower
   forcing-robust ordinal sum than B;
6. the direct longwave and sublimation operands are nonzero where physically
   applicable and have the contracted signs;
7. the interaction does not reveal a compensating-error pattern, defined as
   opposite marginal degradations in protected state/timing signatures hidden
   by an improved aggregate LS score; and
8. no forcing rescaling, coefficient change, fixture change, observation
   leakage, or post-result operator change occurred.

Otherwise the result is `CLOSE_NONPROMOTION`. A tie is not improvement.

## Claim Limits

- Harvard/Marcell paired canopy/open contrasts may support cold/humid
  continental inference.
- SNOTEL lanes constrain open-control sublimation behavior but cannot identify
  canopy longwave.
- Sleepers lanes are diagnostic-only for frost/downstream effects.
- No decisive warm-maritime conifer transfer claim is permitted without a
  bound paired observation lane.
- Comparator similarity is not correctness authority.

## Stop-Loss

This is the single preregistered round. If the promotion rule fails, do not
retune or open another round from the same evidence. Close nonpromotion and
advance only with new authoritative process science, new discriminating data,
or an independently testable formulation.
