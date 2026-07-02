# D7 Feasibility Findings

Evidence class: **Ran** — a throwaway spike (2026-07-02) that executed the
real Iwagaki operands through the shadow `ofe_routing` solver and compared to
the digitized enhanced-WEPP series. The spike solver-runner code was **not
committed** (discarded with its worktree); this artifact records the
findings. Copyrighted `Figure_*.xlsx` series are **referenced by sha256, not
duplicated**; only scalar summary values (peaks, times) are cited.

## Digitized data inventory (Figure_*.xlsx)

Source: `references/copyrighted/Papanicolaou2018-supplemental/wrcr23071-sup-0002-2017wr021109-ds01/`
(gitignored). `Figure_4.xlsx` sha256 `2bf68787…d2fe8` (matches D01 manifest);
`Figure_5.xlsx` sha256 `a58c4e29…d1f96`.

| File | Sheets | Series present | Notes |
|---|---|---|---|
| Figure_4 | `Enhanced_WEPP`, `Original_WEPP` | Iwagaki, Neibling, Abban, Jomaa shock cases | Iwagaki `q` in **m³/s/m = m²/s** (= solver unit). Enhanced Iwagaki peak **0.008132 m²/s @ 25.98 s**. |
| Figure_5 | `Results` | **Observed-Helmers** + Enhanced + Original | Enhanced peak **0.0028 m²/s** to t≈29 h — magnitude/timescale ⇒ **Walnut Creek hillslope, not the Case-1 7.5 m plot**. Mixed units per column (`l/s/ha`, `m³/s/m`, `mm/hr`). |
| Figure_6 | `23 mm`,`46 mm`,`92 mm` | grain / isolated / vegetation | rainfall thought-experiments (hillslope basis). |
| Figure_7 | `3.5%`,`7%`,`14%` | grain / isolated / vegetation | slope thought-experiments. |
| Figure_8 | `7%_Gradient` | convex / uniform / concave | curvature thought-experiments. |
| Figure_9 | `Results` | normalizing constants | Zone 1/2: I=150, q=0.0051464, S=40. |

**Cut-point hazard (must resolve in D7-S0):** columns carry different units
and geometries; Fig 5–8 read as the Walnut Creek hillslope thought-experiments
(paper §3.3), not the plot-scale Cases 1–3. Comparing before proving
like-for-like will fabricate agreement or divergence.

## Operands (paper Table 1 + §3.1; D01 `validation-cases.json`)

| Case | Setup | Friction | Gap |
|---|---|---|---|
| 1 bare | 60 mm/h, 9%, 7.5×1.2 m, Tama silt-clay-loam, 5 h | `k_o=500` | needs Green-Ampt `Ks/ψ/Δθ` |
| 2 isolated | 74 mm/h, 2.2%, 6×1 m | `k_o=500,Cd=1,Dr=0.06,λ=0.2` | needs soil params |
| 3 vegetation | 74 mm/h, 7%, 6.1×1.8/3.6 m, strip 2.45 m | `k_o=500,LAI=1,hc=0.1,Cd=1` | needs soil params |
| 4 Iwagaki | 24×0.196 m flume, 3×8 m @ 2/1.5/1%, lateral 0.108/0.0638/0.08 cm/s for 10 s, impermeable | **unspecified** | **flume `k_o` not in paper** |

Case 4 needs no infiltration (impermeable); Cases 1–3 require the D6
Green-Ampt coupling with texture-derived soil params.

## Iwagaki (Case 4) result — reproduction NOT achieved

Ran the exact operands (per-section inflow, 10 s, impermeable) through the
shadow solver; compared outlet unit discharge to digitized enhanced-WEPP
(direct — same m²/s unit).

| Variant | Peak (m²/s) | vs ref 0.008132 | t_peak (s) | vs ref 26 s | Nash-Sutcliffe |
|---|---|---|---|---|---|
| single-mesh, `k_o≈150` | 0.00788 | **−3%** | 31 | **+5 s** | **0.148** |
| single-mesh, `k_o≈200` | 0.00690 | −15% | 31 | +5 s | 0.149 |
| 3-OFE cascade, `k_o≈150` | 0.00445 | −45% | 34 | +8 s | 0.120 |

- Peak **magnitude** is reproducible; peak **timing** lags ~5 s and **no
  `k_o` matches both** — a systematic celerity / shock-timing lag.
- Paper (§3.1.4): *Original* WEPP is "~5 s slower, 25% less" — openWEPP
  behaves like **Original**, not Enhanced, on the shock.
- The 3-OFE cascade is **worse** (handoff interpolation smears the front) —
  so the lag is not a single-mesh-vs-cascade artifact.

**Verdict:** openWEPP does **not** currently reproduce the Iwagaki shock
(NS ≈ 0.15 ≪ Ef 0.88). Candidate causes to attribute in D7-S4: numerical
front-diffusion (TVD limiter), friction-regime dispatch under an unknown
flume `k_o`, or celerity. Not forced into a pass.

## Implication for D7 scope

The D-val is a real staged investigation. Recommended order: pin cut-points
(S0) → complete operands (S1) → skin-unit convention (S2) → the tractable
rise-to-steady cases first for a positive datum (S3) → the shock as a distinct
fidelity attribution that may end in a documented GAP (S4).
