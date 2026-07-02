# C01-S2 — Envelope Derivation and H2637 Applicability

Evidence class: **Static synthesis** of the S1 extraction (page-cited in
`extraction.md`) + **Ran** H2637 values from the `postmerge`/`dc01-m3`
parquet. The envelope tests a **law with a bounded magnitude** (ADR-0011;
correctness re-anchoring scheme, level-4/5 external authority) — a value
inside a band is *not-contradicted*, not *validated*. Individual anchor
numbers carry agent-extraction provenance; **verification debt is declared**
— C03 re-verifies any number against its cited page before a verdict
consumes it.

## Site class mapping to H2637

H2637: 19 OFE, 416 m slope, gradients 0.49–0.91 (26–42°), sandy-loam
forest, 2,825 mm/yr, no modeled overland-flow pathway.

| Site | Slope | Annual P | Soil depth | Class match |
|---|---|---|---|---|
| HJ Andrews WS10 | 30–48°, avg 37° | 2,220 mm | ~1.3 m + saprolite | **primary** (steep-wet PNW conifer; near-twin) |
| Maimai M8 | 34–40° | 2,450–2,610 mm | 0.55–0.6 m avg | **primary** (steep-wet; shallower soils) |
| Panola | 13° | 1,240 mm | 0.63 m avg | **shape-only** (drier, gentler; threshold structure, not magnitude) |
| Coweeta | — | — | — | context-only (README restriction; no precip in-fixture) |

## ENV-Y — Annual combined water-yield fraction

Comparand: `Y = (Σ runvol + Σ latqcc·A_ofe) / (Σ P·A_total)` (streams
collect surface + lateral export, so both channels must be summed).

Observed streamflow ratios: WS10 **0.56** of 2,220 mm (McGuire & McDonnell
2010 p.2); Maimai **0.54–0.60** — 1,320–1,550 mm of 2,450–2,610 mm,
corroborated independently by Woods & Rowe p.55 (11-yr gauged), McGlynn
2002 p.3, and Mosley 1979 (0.57; 27-month 0.59 of gross).

**Wetness transposition** (H2637 is 200–600 mm/yr wetter): at these
energy-limited wet-forest sites the *non-yield* water (ET + interception +
deep seepage) is approximately conservative across the precipitation range —
WS10 ≈ 977 mm/yr non-yield; Maimai ≈ 950–1,200 mm/yr (interception ~26% of
P + transpiration + ~100 mm deep seepage). Holding non-yield ∈ [950, 1,200]
mm/yr at P = 2,825 mm/yr gives yield **0.575–0.66**.

**ENV-Y envelope: annual water-yield fraction ∈ [0.55, 0.72]** for the
H2637 wet-steep class (band widened above the transposed 0.575–0.66 for
climate-sequence and site-transfer uncertainty; the upper 0.72 is the point
beyond which ENV-ET would be squeezed below plausibility). Conditioned on
the wet-steep class; does **not** transfer to drier/gentler sites.

**H2637 readings (Ran):** pre-DC01 **≈0.77** (runvol 0.72 + latqcc ~0.05) —
**above** the envelope. Post-DC01 **≈0.67** (runvol 0.47 + latqcc ~0.20) —
**inside**, upper-middle. The DC01 correction moved H2637 from
out-of-envelope to in-envelope on the annual tier.

## ENV-T — Event commencement threshold (shape)

Observed: generic **15–35 mm** (Weiler cross-site); WS10 ~30 mm, Maimai ~23
mm, Panola 55 mm (drier/gentler outlier). Law: a storm-size threshold exists
below which combined event export is negligible (RC < ~1%) and above which
it rises steeply, jointly gated by antecedent wetness.

**ENV-T envelope: a detectable commencement threshold in 15–40 mm** for
H2637's wet class (Panola's 55 mm excluded as the dry-site upper bound).
Judged as *shape presence*, not a fitted value.

## ENV-E — Above-threshold conditioned event ratio (shape + band)

Observed: WS10 marginal slope 0.58 for AP₁₄>20 mm (−18.8 mm intercept),
per-storm mean 0.22 trench / 0.31 catchment; Panola 30–80% of P−55mm; Maimai
0.31–0.35; Harr 23–51% (97% subsurface). Law: above threshold the event
combined-export/precip rises with storm size and antecedent wetness toward a
marginal fraction.

**ENV-E envelope: large-storm (>50 mm, wet antecedent) event ratio ∈
[0.25, 0.80]**, with the *ascending distribution shape* (ratio increasing in
P and antecedent) as the primary object and the band as the guard.

## ENV-ET — Evapotranspiration plausibility (counterpart check)

Yield and ET are complementary; an implausibly high yield achieved by
suppressing ET is a false pass. Forest ET context: Maimai interception 26%
+ transpiration ≈ 600–800 mm/yr non-runoff; PNW conifer ET commonly
500–900 mm/yr.

**ENV-ET envelope: 500 ≤ ET_H2637 ≤ 1,000 mm/yr.** H2637 post-DC01
**≈863 mm/yr** — inside; pre-DC01 **≈397 mm/yr** — **below**. ENV-ET is what
makes ENV-Y honest: DC01 raised ET into the plausible band, corroborating
that the re-infiltrated runon becomes real transpiring soil water, not an
accounting sink.

## Hillslope-trench cross-check (secondary, not a gate)

Woods & Rowe M8 measured **0.13** lateral fraction (dry-season, regrowth;
low anchor), water-balance ~0.25; McDonnell et al. 1998 working ratio
**0.40** (wet season). These are single-channel *lateral-only* trench
totals over partial seasons — not comparable to an annual two-channel yield,
so they inform plausibility of the `latqcc` share (H2637 latqcc ≈0.20 of P
sits between the dry-season 0.13 and wet-season 0.40) but do not gate.

## Uncertainty treatment and exclusions

- Annual anchor triangulated ×4; bands wide by construction (law-with-bound,
  not calibration target). Inside-band = not-contradicted.
- **Excluded as bounds:** sub-hillslope per-trough coefficients (up to 2.7,
  area artifacts); Panola/Coweeta annual magnitude (wrong class); any
  single-channel-vs-two-channel comparison.
- Directional biases recorded: Woods 0.13 dry-season regrowth low; WS10
  0.58 marginal-with-intercept; Maimai 2015 series post-weir-rebuild.

## What this buys the campaign

H2637's live post-DC01 numbers (Y≈0.67, ET≈863 mm/yr, latqcc share ≈0.20,
4× lateral growth) sit **inside all four tiers**, and DC01 is what moved
ENV-Y and ENV-ET from out-of-band to in-band. This is the first time the
FARPOINT01 magnitude has been judged against external authority rather than
legacy, and the judgment is *consistent* — pending the formal C03 run that
re-verifies the anchors and issues the verdict.
