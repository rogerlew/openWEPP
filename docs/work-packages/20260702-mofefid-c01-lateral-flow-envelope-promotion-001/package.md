# MOFEFID-C01 — Lateral-Flow Observed-Authority Envelope Promotion

Status: **IN EXECUTION** (2026-07-02)
Campaign: [MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane C
(stage C1). Owner: Claude Code. Urgency: adjudicates the live post-DC01
production numbers.

## Objective

Promote the acquired observed datasets
(`tests/fixtures/forest_lateral_flow_authority/`) and literature corpus
(R-62..R-98) into a **ratified acceptance envelope** for forest-hillslope
lateral-flow/water-yield magnitude, satisfying the fixture README's five
use-limit conditions and the FARPOINT01 backlog promotion criteria. C01
delivers the envelope contract; the harness (C02) and the judgment (C03)
are separate packages. No judgment is issued in C01.

## Judged quantities (design decision, adjudicated in this package)

Streams collect both surface and lateral hillslope exports, so single-
channel comparisons (runvol alone vs streamflow) are category errors.
Primary metrics:

- **M-Y (annual water-yield fraction):** `(Σ runvol + Σ latqcc·A_ofe) /
  (Σ P × A_total)` vs observed annual streamflow ratios (WS10, Maimai,
  Coweeta-context).
- **M-E (event quick-flow behavior):** event `(runvol + latqcc)` vs event
  precipitation, judged for **threshold shape and conditioned ratios**
  (Panola 147-storm distribution; WS10 antecedent-conditioned ratio) —
  never as a per-event parity target.
- **M-ET (evapotranspiration plausibility):** annual ET vs regional
  forest-ET ranges — the counterpart check that re-infiltrated water is
  transpiring plausibly, not vanishing.

## H2637 applicability anchor (Ran, from fixture inputs/outputs)

19 OFE, 416.3 m slope length, gradients 0.49–0.91, sandy-loam forest
soils, **2,825 mm/yr** precipitation (34 yr), area 20.65 ha. Live
post-DC01 values to be judged (by C03, not here): runvol 46.98 %,
latqcc ≈ 20.3 %, combined yield ≈ 67.3 %, ET ≈ 863 mm/yr.

## Stages

1. **C01-S1** — quantitative extraction from the corpus (page-cited) +
   observed-data inventory. → `artifacts/extraction.md`
2. **C01-S2** — per-site envelope derivation with uncertainty treatment
   and explicit applicability mapping to H2637 (slope/precip/soil-depth
   class); Coweeta context-only per its README restriction.
   → `artifacts/envelope-derivation.md`
3. **C01-S3** — contract authoring: authority suite + envelope invariants
   (SC-SUBHYD-001 amendment), REF anchors to datasets + papers; the
   envelope is level-4/5 external authority per the correctness
   re-anchoring scheme (test a law, not a number).
4. **C01-S4** — review/disposition (Codex).

## Guardrails

- Fixture README use-limits are binding; Coweeta carries no `latqcc`
  verdict. Legacy remains a flag (ADR-0017) and plays no role here.
- The envelope must be **conditioned** (threshold/antecedent structure),
  not a scalar band, wherever the observations are conditioned.
- Numbers extracted by research agents enter the contract only after
  page-cited verification in the artifact.
