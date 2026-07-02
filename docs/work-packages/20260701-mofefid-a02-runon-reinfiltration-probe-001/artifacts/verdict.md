# A02 Verdict: CONFIRMED (material)

Evidence class: **Ran** (all numbers from executed H2637 runs this package,
2026-07-01; commands and gate outputs in the run log below).

## Result against the pre-declared rubric

| Metric | Baseline (env unset) | Probe (env=1) | Δ |
|---|---:|---:|---:|
| `runvol_pct_precip` | 72.3318324819 % | **62.2312715310 %** | **−10.10 pp** |
| runvol (canonical, m³) | 14,349,153 | 12,345,409 | −14.0 % |
| outlet `latqcc` volume (m³) | 975,467 | 2,273,482 | +133 % |
| ET total (outlet rows, mm) | 13,517 | 18,108 | +34 % |
| `Dp` total (outlet rows, mm) | 304 | 304 | 0 |
| wall | 33.26 s | 33.06 s | — |

**CONFIRMED**: movement toward legacy (−10.1 pp) is double the ≥5 pp
rubric threshold, from a probe that measures a **lower bound** (dry-runon
days skipped; daily-lump rather than hourly distribution). Against this
recipe's baseline-vs-legacy span (72.33 % → ~55.5 %), the lower-bound
probe closes roughly **60 %** of the FARPOINT01 magnitude gap.

The redistribution is mechanistically coherent, not an artifact:
re-infiltrated runon becomes soil moisture (ET +34 %), partly re-emerges
as lateral flow (outlet `latqcc` +133 %), and leaves deep seepage
untouched — exactly the filter-strip water pathway the baseline
`fin`/`xfin` semantics encode.

## Metric-basis note

The recipe here (canonical volume `QOFE(outlet) × Area(outlet)` ≡
`Σ pass.runvol` — the two agreed to machine precision in both runs, an
internal consistency check — against `Σ P × A_total` on outlet rows) gives
baseline 72.33 %, whereas the FARPOINT01 record cites 71.0036550031206 %
from its own tooling. The bases differ slightly; the **delta** (−10.10 pp,
same recipe both runs) is the verdict-bearing quantity.

## Structural witness (bonus finding)

The probe's first run failed at the erosion seam: `erod14` enforces
`qin ≤ qout + ε` (`erosion.rs:426`) — **flow may never shrink downslope**
— an assumption that holds in production only because runon never
re-infiltrates. A downslope OFE absorbing upstream inflow (the
filter-strip case, `qout < qin`) is a domain violation to the current
MOFE erosion surface. Under the probe flag qin is clamped to qout
(diagnostic-only; no water feedback). This hard-codes F-A2's semantics
into a second surface and must be part of the D01 contract decision:
adopting re-infiltration requires reworking the erod14 monotonicity
assumption and the (already-held, F-A1) case machinery together.

## Gates

1. Unit tests 2/2; orchestrator suite 146/146; runner suite green (see
   package log); fmt/clippy clean (0 warnings).
2. Default path **byte-identical** on all five H2637 protected outputs
   with the env unset (verified twice: before and after the erosion-seam
   clamp landed).
3. Probe run exit 0, all closure guards green across 235,961 OFE-days —
   the re-infiltration accounting conserves by construction.

## Consequence (per rubric)

F-A2's contract decision **escalates ahead of Lane D**: the runon
re-infiltration seam is a first-order driver of the FARPOINT01 magnitude
flag, not a deferred fidelity nicety. Recommended next: a Defect-Closure
ExecPlan anchoring the pinned-baseline `fin`/`xfin` semantics under
ADR-0024 into `SC-RUNOFFPART-001`/`SC-WATBAL-001` (hourly-faithful
implementation, not this probe's daily-lump approximation), sequenced
with the erod14 monotonicity/case rework, with Lane C's observed
envelope as the post-change magnitude bar. The probe itself remains
opt-in diagnostic code; no production semantics changed.
