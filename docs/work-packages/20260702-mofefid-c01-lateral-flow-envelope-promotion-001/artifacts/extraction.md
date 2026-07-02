# C01-S1 — Quantitative Extraction (page-cited)

Evidence class: **Static** — page-cited reads of the acquired corpus
(R-64/67/68 and R-86/87/88/89, verified against the PDFs/transcriptions).
Numbers enter the envelope (S2) and contract (S3) only from this record.

## Annual water-yield ratios (catchment streamflow / precipitation)

| Site | Annual P | Runoff ratio | Slope | Soil depth | Source |
|---|---|---|---|---|---|
| HJ Andrews WS10 | 2,220 mm (1990–2002) | **0.56** | 37° avg (27–48°) | ~130 cm | McGuire & McDonnell 2010 p.2 |
| Maimai (Rowe & Pearce) | 2,450 mm (1890–3000) | **≈0.54** (M6 1,320 mm/yr) | mainly >35° | 60 cm avg (25–130) | Woods & Rowe 1996 p.55 |
| Maimai (McGlynn) | ~2,600 mm | **≈0.60** (~1,550 mm) | 34° avg | 0.2–1.8 m | McGlynn 2002 p.3 |
| Maimai (Mosley, 27-mo) | 5,573 mm gross | **0.59 of gross** (74% of net) | 35° | 55 cm avg | Mosley 1979, Pearce & McKerchar |
| Panola (context only) | 1,240 mm | 0.16–0.50 | 13° | 0.63 m avg | Tromp-van Meerveld 2006 p.2 |

Convergence: three independent Maimai papers + WS10 place steep wet
temperate/PNW forest annual yield at **0.54–0.60** on 2,200–2,600 mm.
Panola is drier (1,240 mm) and gentler (13°) — **applicability caveat: use
Panola for event-threshold shape, not annual yield magnitude.**

## Event quick-flow behavior (threshold + conditioned ratio)

- **WS10 (McGuire & McDonnell 2010):** commencement threshold **~30 mm**
  (no quick flow below; p.5). Above threshold, near-linear `Q_F = 0.58·P −
  18.8`, R²=0.91, fit to AP₁₄>20 mm storms (Fig.3, p.7) — a *marginal*
  event fraction with a −18.8 mm intercept, not a mean ratio. Per-storm
  mean `Q_F/P` = **0.22 trench / 0.31 catchment** (Table 1, p.5). WS10
  ratio >0.30 when P>~65 mm.
- **Panola (Tromp-van Meerveld 2006):** threshold **55 mm** (bootstrap
  40–60 mm; p.6). Above it, marginal trench runoff coefficient **30–80% of
  P−55mm** (p.9). Below it, 90% of 147 storms had RC<1%; whole-record
  aggregate only 5% of P. Threshold jointly gated by antecedent moisture.
- **Maimai (Woods & Rowe Fig.6; Mosley Table 1):** event hillslope
  subsurface runoff/rain **0.31–0.35** (37–53 mm storms, wet antecedent);
  per-storm channel `Q_F/net-P` 3%→75% ascending with storm size;
  commencement **~23 mm** (Mosley, via Tromp-van Meerveld p.8).
- **HJ Andrews (Harr 1977):** quick flow **23–51% of gross storm P** (mean
  38%, 7 storms), of which **97% is subsurface** (Table III) → subsurface
  stormflow ≈ 37% of gross storm P.
- **Cross-site (Weiler 2005):** generic commencement threshold band
  **15–35 mm**; above threshold, sites range from ~1:1 (Tani) to Panola's
  30–80% of excess.

## Hillslope-scale lateral fraction (physically measured trench)

- **Woods & Rowe M8 (the in-fixture trench):** 110.5-day summer–autumn
  total **70 mm trench outflow / 550 mm rain = 0.13 measured**;
  water-balance expectation ~0.25 (p.64-67). Dry-season, post-harvest
  regrowth — a *low* anchor.
- **McDonnell et al. 1998 (same trench, 45 wet days):** investigators'
  working hillslope runoff ratio **0.40** on 376 mm (McGlynn p.19-22).
- **Harr flux magnitudes:** top-meter soil 3–4.5 mm/h, subsoil 0.5 mm/h;
  lateral ≈ vertical during storms in the top layer, lateral-dominated at
  70–130 cm — "top 110 cm is the most active zone of downslope movement."

## Mechanistic controls (for the "test a law" framing)

- Steep slope + thin conductive soil + impeding layer → perching → lateral
  flow along the soil–bedrock interface; **no Horton overland flow observed
  at any of WS10/Panola/Maimai/HJA**; saturation-excess source areas only
  4–7% of catchment (Maimai).
- Stormflow is 55–97% **pre-event (old) water** — the hillslope routes
  stored water, it doesn't flush event water. This is what a high modeled
  lateral fraction *should* look like mechanistically.
- Drainable porosity decays exponentially with depth (Weiler Table 1,
  forest n₀ 0.13–0.36); soil-depth variability alone moves subsurface-flow
  volume by orders of magnitude at fixed Ksat.

## Scale/quantity discipline (binding on S2)

Each number is tagged event / seasonal / annual and streamflow / trenchflow
above. The envelope must not compare an annual modeled fraction to an event
observed ratio, or a two-channel modeled yield to a single-channel trench
total. Sub-hillslope per-trough coefficients (up to 2.7 at Woods) are
contributing-area artifacts and are **excluded** as bounds.
