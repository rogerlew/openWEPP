# Claude Code Review — WBVAL04 Redo and the WEPPpy Daymet Radiation Producer Fix

Reviewer: Claude Code
Date (UTC): 2026-06-06
Evidence mode: **Static** — read the WBVAL04 package and artifacts
(climate-precondition-audit, wbval01-redo-comparison, disposition, handoff), the
`.cli` radiation column, and the WEPPpy producer fix
(`wepppy/nodb/core/climate_build_helpers.py`, commits `e6b5b9a9b` /
`dcab33fe4`). I independently verified the TOA physics earlier (DOY 49 daily
extraterrestrial ≈ 452 Ly/day ≈ openWEPP `r3` 453). I did **not** re-run the
22-hillslope batch; those results are Codex's `Ran` evidence, attributed.

Verdict: **APPROVE WBVAL04** as a clean, conformant validation redo, and the
**WEPPpy producer fix is correct in approach** — a cap-to-physical-ceiling at the
producer boundary with full provenance, addressing a genuine Daymet product bias,
without loosening openWEPP's guard. Three refinements on the producer fix and one
gap to carry into WBVAL06 follow.

This WBVAL01→02→03→04 arc is the new discipline working as intended: validation
found defects, a DC-ExecPlan (WBVAL02) fixed the real one and localized the
upstream boundary, the producer fixed the data, and a validation redo confirmed
the unblock and routed the genuine remaining defects — in four tight,
evidence-grounded packages with no grind. That is the contrast with HPHYS0298→0320.

---

## The producer fix is sound (and was the right diagnosis)

It confirms my earlier read: the over-TOA radiation is a **genuine Daymet source
bias**, not a wepppy unit/divisor bug. The run used the correct
`srad × dayl / 41840` path; 53 of 2191 days (2.4%), clustered in late-Feb/March
clear days, exceed the astronomical horizontal potential — the documented Daymet
winter/high-terrain srad over-estimation. The fix
(`_normalize_daymet_radiation_to_toa_bound`) caps those rows to the physical
ceiling and records `srad_source`, `srad_toa_bound`, `srad_toa_normalized`,
`srad_toa_normalization_reason`, and a 53-row CSV. This is **cap-with-provenance
at the producer**, exactly the producer↔consumer split that was called for:
WEPPpy owns delivering physical radiation; openWEPP's `CLIM-RUNTIME-E-017` guard
stays and correctly rejects non-physical input. No guard was loosened.

## P1 — Cross-repo formula duplication / drift risk (architectural, main finding)

`_baseline_sunmap_horizontal_daily_potential_ly` in WEPPpy **reimplements
openWEPP's `legacy_sunmap_horizontal_radpot_ly` in Python** — same solar constant
(`1.94 Ly/min`), declination, hour-angle, and `r3` integral. The producer cap and
the consumer guard now agree only because the same legacy formula is *duplicated
in two languages and two repos*. If either side changes the formula, the solar
constant, or the rounding — or if they simply differ in floating-point evaluation
(see P2) — the cap drifts relative to the guard: above it re-triggers
`CLIM-RUNTIME-E-017`, below it over-caps. There is no single shared authority.

Recommendation: make the horizontal-potential bound a **named shared definition**
(a `SC-CLIMATE-001` clause both repos cite) plus a CI cross-check (a golden table
of `r3(julian_day, latitude)` that both the Rust guard and the Python cap are
tested against), so the two implementations cannot silently diverge.

## P2 — The 0.000293 Ly/day margin is too tight to be robust (robustness)

The cap publishes `floor(r3)`, so the margin below the exact bound is `frac(r3)`,
which on `1991-03-30` / `1995-03-30` is **0.000293 Ly/day**. openWEPP's runtime
guard compares the published `rad` against its *own* recomputation of `r3`. Across
Rust and Python `sin`/`cos`/`tan` (different libm), `r3` is not guaranteed
bit-identical; a cross-language difference larger than 0.000293 on that day would
flip `703` from "just under" to "above" and resurrect the guard failure on a
hillslope that passes today. The fix is one-sided against this: it floors to the
bound with no safety margin.

Recommendation: cap with a real margin — e.g. `floor(r3) − 1 Ly`, or
`floor(r3 × (1 − ε))` — so a small cross-language `r3` difference cannot
re-trigger the consumer guard. Combined with P1's shared golden table, this makes
the producer/consumer contract robust rather than coincidentally-aligned.

## P3 — Cap-to-TOA over-credits radiation on snowmelt-onset days (physical fidelity)

The cap target is the TOA/`r3` ceiling — the *brightest physically possible* day.
A real clear winter day is ~0.7–0.75 of TOA, so the 53 capped days are now ~25–30%
brighter than a realistic clear day, and they cluster at **snowmelt onset**
(late-Feb/March), where radiation drives melt and ET. The cap is bounded,
provenance-logged, and a defensible "least-wrong guaranteed-physical" unblock —
but it injects a radiation (hence melt/ET) high-bias precisely on the days that
matter most for the snow/melt surfaces. A clear-sky-fraction cap
(e.g. Bristow–Campbell, or ~0.75·TOA) would be a better central estimate but
needs more modeling.

This is acceptable as an unblock, but **WBVAL06 must treat the 53 capped days as a
known forcing perturbation**, not transparent data. (Note the direction: more
radiation → more ET → a *smaller* positive residual, so the persistence of the
`R>0` leak despite this over-credit actually strengthens the "real leak" finding.)

## WBVAL04 conformance and the follow-ons

WBVAL04 is correctly scoped as **validation/characterization, not a DC-ExecPlan**:
it verified the precondition, ran all 22 hillslopes, characterized the population
(18/22 conservation-break, max 94.433 mm at p4 yr5; 4/22 still J-95
`HKERNEL-WB11-PERC-E-003`), edited no code/contracts, and emitted two
**defect-shaped** follow-ons — `WBVAL05-J95-HKERNEL-WB11-PERC-E-003` and
`WBVAL06-SINGLE-OFE-WAT-CONSERVATION-RESIDUAL` — each with observable failure,
mechanism, write-set, authority, acceptance, and HOLD conditions. The
`executed-hold` is honest (validation done; defects routed). Good conformance.

### Gap to carry into WBVAL06 — the snow protected boundary

WBVAL06's authority list includes `SC-SNOWFREEZE-001`, and the leak's sign
(`R>0`, water vanishing) points at possible snow-pack mass-loss. Per ADR-0018 §8
and my WBVAL01/WBVAL03 reviews, WBVAL06 (when authored as a DC-ExecPlan) **must
declare the snow protected boundary**: if leak attribution lands on snow-pack
mass-loss, it routes to the backlog snow-science review, not an in-package snow
fix. As written, the WBVAL04 handoff lists `SC-SNOWFREEZE-001` as in-scope
authority without that negative boundary — the back-door the protected-boundary
rule exists to close. Add it to WBVAL06's envelope.

---

## Disposition boundary

Findings + evidence per the review model. The producer-fix findings (P1, P2, P3)
are **WEPPpy-side** and belong to the
`20260606_indispensable_presenter_daymet_radiation_bounds` WP — surfaced here
because WBVAL04 depends on that fix; routing them to the WEPPpy WP is a decision
for the maintainer. The WBVAL06 snow-protected-boundary gap is the one item to
fix before WBVAL06 is authored. WBVAL04 itself requires no change.
