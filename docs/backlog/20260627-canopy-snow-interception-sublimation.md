# Canopy Snow Interception and Sublimation (Hedstrom-Pomeroy)

## Status

- `state`: **backlog (concept)** — a real, confirmed physics gap, but **not the
  cause of the current maritime over-accumulation blocker** (those sites are
  open/leaf-off). Latent until the dense-conifer fixtures carry paired snow
  observations; not promotable before then.
- `date`: 2026-06-27 (created, Claude Code)
- `relates`:
  [ADR-0011](../decisions/0011-architecture-first-top-down-science-contracts.md)
  (contract-first new physics),
  [ADR-0017](../decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md),
  `SC-SNOWFREEZE-001` (canopy-cover authority; rain-only interception),
  the `cancov` canopy machinery (currently melt-attenuation only),
  `tests/fixtures/cancov_forest/` (the conifer→deciduous→open gradient this would
  differentiate),
  [winter-thaw / over-accumulation arc](20260626-snow-code-deferred-science-review.md)
  (the *distinct* current blocker).
- `provenance`: design discussion 2026-06-27 — confirmed in code
  (`direct_runtime/runoff.rs` interception operates only on
  `interception_rainfall_input_m`) and contract (`cancov` drives melt attenuation
  + rain interception, not snow). No code/contract authored.

## Summary

WEPP/openWEPP **does not model canopy snow interception storage.** Interception is
**rain-only** — the canopy intercepts rainfall up to an LAI/biomass capacity and
evaporates it; **snowfall bypasses the canopy entirely** and goes straight to the
ground snowpack. There is no canopy snow-load state, no sublimation from
intercepted snow, and no unloading. The only "sublimation" in WEPP is in the
**inactive** blowing-snow drift routine (`INV-SNOWFREEZE-008`), not canopy.

Because canopy snow interception + sublimation is a **major loss term under dense
evergreen forest** (10–40% of snowfall; Hedstrom & Pomeroy 1998), delivering *all*
snowfall to the ground makes openWEPP **over-deliver snow under conifer canopy**,
over-predicting the under-canopy snowpack.

## Scoping (read before prioritizing)

- **This is NOT the current maritime over-accumulation cause.** That blocker
  (10.3.4–10.3.7) is at **Sleepers (open/ag)** and **Harvard (hardwood, leaf-off
  in winter)** — little-to-no winter canopy to intercept. The lever there is
  winter-thaw drainage, not interception.
- **It WOULD bite the dense-conifer fixtures** — HJ Andrews (Douglas-fir /
  western hemlock), the conifer SNOTEL sites — where interception/sublimation is
  large. Those are currently **observation-blocked / untested**, so the
  over-prediction it would cause has not been observed.
- It is the canopy process that would actually **differentiate the
  `cancov_forest` gradient**: a conifer canopy intercepts heavily, deciduous/open
  barely — so it is the natural pairing for the conifer-vs-mixed-vs-open fixtures.

## Mechanism (minimal abstraction)

Hedstrom & Pomeroy (1998) canopy snow interception, with the
Pomeroy/Storck sublimation + unloading:

- **Interception** to a canopy snow load `L`, bounded by a species/LAI-dependent
  maximum `L_max` (function of canopy closure / leaf area and fresh-snow density):
  `ΔL = c · (L_max − L) · (1 − e^{−Cp·Sf/L_max})` for snowfall `Sf`.
- **Sublimation** of the intercepted load (the dominant loss; an aerodynamic /
  energy term — couples to the `openwepp-meteorology` latent-heat / vapour-pressure
  primitives).
- **Unloading** of the remainder to the ground (temperature/melt/wind-triggered),
  delayed relative to the storm.
- Ground snowfall = `Sf − ΔL`; the intercepted `ΔL` is later partitioned to
  sublimation (loss) vs unloading (delayed ground delivery).

Driven by `cancov` / LAI — the same canopy state already used for melt
attenuation, so it is additive, not a parallel canopy model.

## The openWEPP opportunity

The current `cancov` machinery does **only** the `(1−cancov)` melt attenuation.
This adds a **canopy snow process** (load state + sublimation + unloading) on the
same canopy state — another physical mechanism the SNOBAL/CRHM lineage carries and
WEPP lacks, adopted selectively and validated (the same pattern as Anderson
compaction and the Harder-Pomeroy partition), rather than a wholesale model swap.

## Governing constraints

- **Contract-first (ADR-0011):** author an `SC-SNOWFREEZE` amendment (or new `SC-*`)
  for the canopy snow-load state, the interception/sublimation/unloading laws, and
  the `cancov`/LAI driver, before production code.
- **Conservation:** mass balance must close — intercepted snow = sublimated +
  unloaded + remaining canopy load; no snow created or destroyed. Sublimation must
  draw a matching latent-heat/vapour term (reuse `openwepp-meteorology`).
- **Opt-in, default-preserving, no site calibration** (the arc's standing
  discipline): `L_max` and sublimation/unloading constants from cited authority,
  never fitted to the validation fixtures.
- **Reuse the canopy state:** drive from the existing `cancov`/LAI surface, not a
  new canopy model (the contract already forbids a "separate diagnostic canopy
  model").

## Falsifiable validation gates

1. **Under-canopy reduction:** with interception on, the modeled conifer
   under-canopy snowpack is **lower** than the no-interception baseline, in the
   direction of observed under-canopy SWE.
2. **Gradient differentiation:** across `cancov_forest`, conifer accumulates less
   ground snow than open at the same forcing (interception scales with canopy);
   deciduous/open barely change.
3. **Sublimation magnitude** falls in the literature range (~10–40% of forest
   snowfall) — not tuned to hit a fixture.
4. **Conservation** of the canopy snow mass/energy balance closes.

## Promotion criteria

- Dense-conifer fixtures carry paired snow observations (e.g. HJ Andrews EDI
  `MS007` under-canopy, the on-forest SNOTEL pairs), **and**
- under-canopy snow fidelity is on the critical path (e.g. conifer
  over-prediction is observed once those obs are installed), **and**
- a canopy snow-load contract surface is authored.

When these hold, spin up a dated work-package under `docs/work-packages/`.

## Open questions

- `L_max` parameterization: species/LAI-based vs a `cancov`-derived proxy.
- Sublimation formulation: simple energy/aerodynamic term vs the
  Pomeroy/Liston ventilated-ice-sphere model; which is defensible on openWEPP's
  hourly forcing.
- Unloading trigger: temperature/melt vs wind vs a simple decay.
- Interaction with the melt-attenuation `(1−cancov)` term (double-counting risk:
  interception reduces ground snow; attenuation reduces ground melt — keep
  distinct).
- Validation forcing confound: undercatch / lapse already limit absolute SWE at
  the high-relief conifer sites (carry the forcing-robust tiering).

## References

- **Hedstrom, N. R., Pomeroy, J. W. (1998).** *Measurements and modelling of snow
  interception in the boreal forest.* Hydrol. Process. 12, 1611–1625. The
  canonical canopy snow interception model.
- **Pomeroy, J. W., Parviainen, J., Hedstrom, N., Gray, D. M. (1998).** *Coupled
  modelling of forest snow interception and sublimation.* Hydrol. Process. 12,
  2317–2337.
- **Storck, P., Lettenmaier, D. P., Bolton, S. M. (2002).** *Measurement of snow
  interception and canopy effects on snow accumulation and melt in a mountainous
  maritime climate, **Oregon**, United States.* Water Resour. Res. 38(11), 1223.
  DOI `10.1029/2002WR001281` — directly applicable to `hjandrews_conifer_or`.
- **Lundquist et al. (2013)** (R-50; `references/copyrighted/lundquist2013.pdf`) —
  canopy effects on snow.
- CRHM (Pomeroy et al. 2007) — a reference implementation of the interception +
  sublimation modules.
- Internal: the `cancov` canopy machinery, `tests/fixtures/cancov_forest/`,
  `crates/openwepp-meteorology` (latent-heat / vapour primitives for sublimation).
