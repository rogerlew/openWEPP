# C03 Verdict — full evidence

Evidence class: **Ran** (per-storm decomposition of the post-DC01 H2637 WAT
parquet, 34 yr / 1,297 rain storms; script `c03_quickflow.py`) + **Ran**
first-hand PDF re-verification of the event-tier threshold anchors.

## Method

Per-storm decomposition at the outlet OFE: storms = consecutive wet days
(P>0.2 mm) separated by ≥1 dry day; event export accumulated over storm days
+ recession (to next storm, capped 3/5/7 d — verdict stable across all).
Combined daily export = surface (`QOFE·A_out/A_total`) + lateral
(`latqcc·A_out/A_total`), both normalized to hillslope depth. Event tiers
judged on **quickflow** (Hewlett–Hibbert-style constant-slope baseflow
separation on the total-export series) — see the methodological finding.

## ENV-T size-bin shape (post-DC01, quickflow)

| P bin (mm) | n | mean ratio | median | frac >1 mm QF |
|---|---:|---:|---:|---:|
| 0–10 | 463 | 1.957* | 0.000 | 0.248 |
| 10–20 | 142 | 0.473 | 0.095 | 0.528 |
| 20–30 | 115 | 0.343 | 0.092 | 0.522 |
| 30–50 | 133 | 0.318 | 0.228 | 0.677 |
| 50–80 | 115 | 0.384 | 0.264 | 0.774 |
| 80+ | 329 | 0.419 | 0.392 | 0.894 |

*0–10 mm mean inflated by snowmelt (median 0.000; only 25% produce >1 mm
quickflow). Median ratio is **near-monotonic** — ascending overall with one
minor reversal (0.095 → 0.092 across the 10–20/20–30 bins, inside noise);
response frequency (frac >1 mm) ascends cleanly 0.25 → 0.89. The
commencement signal is the median transition from 0.000 (0–10) to positive
(10–20), placing commencement **~10–20 mm**. The script's mean-based
step-fit returns 5 mm, but that estimator is **rejected as a snowmelt
artifact** — snowmelt injects export on small-rain days, inflating the
small-bin mean; the median- and frequency-based transition at 10–20 mm is
the reliable commencement, and it is what the verdict uses.

## ENV-E band + shape (post-DC01) — robust to the separation parameter

Codex-flagged: the quickflow baseflow-separation slope must not decide the
verdict. Sensitivity sweep (`c03_sensitivity.py`), large + wet-antecedent
(AP14>20 mm, n=371) event ratio vs daily baseflow-rise slope:

| slope (mm/day) | ENV-E ratio | Spearman vs P | vs AP14 |
|---|---:|---:|---:|
| 0.005 | 0.583 | −0.01 | +0.58 |
| **0.0475 (Hewlett–Hibbert canonical)** | **~0.40** | +0.38 | +0.63 |
| 0.10 | 0.365 | +0.47 | +0.59 |
| 0.50 (10× H–H, excessive) | 0.297 | +0.55 | +0.51 |
| **surface-only (no separation param)** | **0.482** | +0.10 | +0.52 |

Hewlett–Hibbert canonical slope: 0.55 L s⁻¹ km⁻² h⁻¹ = 0.0475 mm/day rise.
**ENV-E stays inside [0.25, 0.80] across the entire range** (0.30–0.58), and
the parameter-free surface-runoff anchor (0.48) confirms it independent of
any separation choice. The ascending shape (positive Spearman vs antecedent,
+0.51 to +0.65) holds at every slope. The verdict does not hinge on the
separation parameter. *Headline figure: H–H-canonical **0.40**, not the
earlier arbitrary-slope 0.46.*

## Annual tiers (re-confirmed, Ran)

- ENV-Y combined yield **0.673 ∈ [0.55, 0.72]**.
- ENV-ET **863 mm/yr ∈ [500, 1000]**.
- (Surface-only yield 0.470; quickflow annual fraction 0.455.)

## The methodological trap (total-export, pre-separation)

| metric | total-export | quickflow-separated |
|---|---:|---:|
| small (<15 mm) mean ratio | 5.4–6.5 (impossible) | 1.77 (median 0) |
| Spearman ratio vs P | −0.51 | +0.29 |
| large wet-antecedent ratio | 0.65 | 0.46 |

Total-export conflates event response with continuous baseflow drainage; a
naive judgment would have **falsely failed** the ascending-shape law. The
observed anchors are quickflow/trench-stormflow measurements, so quickflow
separation is the correct comparand — now written into `INV-SUBHYD-033`.

## Anchor re-verification (first-hand, discharges C01 verification debt)

- **Panola 55 mm** — Tromp-van Meerveld & McDonnell 2006, §4.3 ¶20 p.6:
  "significant subsurface stormflow occurred only during rainstorms larger
  than 55 mm... almost 2 orders of magnitude increase"; bootstrap 40–60 mm
  (¶21). ✔ matches C01 extraction.
- **WS10 30 mm + 0.22/0.31 mean Q_F/P** — McGuire & McDonnell 2010, p.5 +
  Table 1: "Quick flow was not produced... for rainfall amounts less than 30
  mm"; Table 1 mean row 0.22 (hillslope) / 0.31 (WS10); WS10 ratio >0.30
  when P>65 mm. ✔ matches C01 extraction.
- Annual anchors (WS10 0.56; Maimai 0.54–0.60) triangulated ×4 in C01.

## Bottom line

H2637's DC01-corrected forest lateral-flow magnitude is **not-contradicted**
by the observed field authority on all four tiers. The FARPOINT01 magnitude
flag — open as an unvalidatable CONTRACT-GAP since 2026-06-18 — is resolved:
the corrected magnitude is consistent with observed steep-wet-forest
hydrology. Legacy's 55.5% is no longer the reference; the field band is, and
H2637 sits inside it.
