# D7 Execution Report — Per-Case Verdicts (S3–S5)

Evidence: **Ran** — `ofe_routing::dval` case runs + `tools/dval/compare_dval.py`
(sha256-verified reads of `Figure_4.xlsx`, offline `NS_trace`, derived scalars
only). Committed cited-scalar tests in `ofe_routing::dval`. `NS_trace` =
openWEPP vs enhanced-WEPP **model trace** (method fidelity); the paper's
`Ef_obs` (enhanced-vs-observed) is **not** recomputed and stays a citation.

## Results

| Case | `NS_trace` | peak ratio (ow/enh) | t_peak ow / ref | verdict |
|---|---|---|---|---|
| 1 bare | **0.868** (@ Ks 6.8) | 1.07 | ~steady plateau | **REPRODUCES (operand-sensitive)** |
| 2 isolated | 0.454 | 0.75 | 10800 / 10620 s | operand-limited (under-predicts −25%) |
| 3 vegetation | 0.538 | 0.55 | 3600 / 3603 s | does-not-reproduce (−45%; S0 magnitude caveat) |
| 4 shock | 0.13–0.18 | ~0.6–1.2 (noisy) | 31 / 26 s | **GAP — shock timing (`GAP-OFEROUTE-D7-SHOCK-LAG`)** |

Shape co-conditions (package acceptance §5) applied per case; details below.

## Case 1 (bare) — REPRODUCES, operand-sensitive

At the literature Ks = 6.8 mm/h (Rawls silt-loam, chosen before the comparison,
**not tuned**), `NS_trace = 0.868 ≥ 0.85` with peak +7% and matching
rise-to-steady shape. **But the fit is knife-edge in Ks:**

| Ks (mm/h) | 2 | 5 | 6.8 | 10 | 14 | 20 |
|---|---|---|---|---|---|---|
| `NS_trace` | −0.51 | 0.76 | **0.868** | 0.37 | −1.04 | −4.28 |

That the independent literature value coincides with the NS-optimum is a real
method-fidelity signal — openWEPP + textbook soil params reproduces the
enhanced-WEPP bare-surface trace. The verdict is **qualified** (operand-
sensitive), not robust, and rests on the S0 col-11 physical-column choice.

## Case 2 (isolated roughness) — operand-limited

`NS_trace 0.454`, peak −25%. openWEPP under-predicts; the sandy/gravel soil
Ks and the isolated-roughness form/wave friction operands are loosely
constrained. Not certifiable as a solver defect without tighter operands.

## Case 3 (vegetation) — does-not-reproduce (caveated)

`NS_trace 0.538`, peak −45% (openWEPP 9.2e-5 vs enhanced 1.685e-4). Carries the
S0 magnitude anomaly (enhanced peak exceeds the plot's I·L). Under-prediction
is consistent with the vegetation resistance over-attenuating, but the S0
caveat blocks a clean magnitude verdict — shape-only, does-not-reproduce.

## Case 4 (Iwagaki shock) — GAP-OFEROUTE-D7-SHOCK-LAG

Attribution (S4):
- **Magnitude**: right order of magnitude but **numerically noisy** —
  peak non-monotonic in k_o (9.5→12.0→8.9→8.8e-3 for k_o 150→175→200→250) —
  a shock-capture sensitivity, not an operand effect.
- **Timing**: outlet t_peak lags the cited ~26 s (observed shock at 23 s) by
  ~5-6 s; `NS_trace 0.13-0.18`. A **lag-corrected** NS peaks at ~0.52 at a
  −5.5 s shift → the gap is **dominantly a phase lag** with residual shape.
- **Operand-independent**: the lag survives the full k_o scan and is **worse**
  as a 3-OFE cascade (handoff smears the front). So it is **solver-side**, not
  operand-limited or method-decomposition.
- **Interpretation**: openWEPP behaves like the paper's *Original* WEPP
  ("~5 s slower") — it does not sharpen the concave-curvature shock catch-up
  that enhanced-WEPP captures. Candidate root cause: TVD-MacCormack front
  diffusion / celerity at the section interfaces.

**Disposition:** promote the package-local provisional gap to a contract
`GAP-OFEROUTE` (shock-capture fidelity). Per package split-rule, a solver
correction is a **separate future package** (write-set exceeds D-val analysis)
— open only after this attribution, which now exists.

## S2 (skin I-unit convention, `INV-OFEROUTE-002`)

Not independently audited. Case 1 reproduces the enhanced trace with the
current `f_s = (3393 I^0.407 + k_o)/Re` implementation, so the convention is
**not grossly wrong**, but an explicit unit audit is **deferred** (recorded as
still-open, not closed by D7).

## S5 contract disposition

- `INV-OFEROUTE-011`: D7 supplies **partial** method-fidelity evidence — Case 1
  reproduces (qualified); Cases 2-3 operand-limited/caveated; Case 4 gap. The
  invariant is **not** closed; evidence row updated to cite this report.
- New `GAP-OFEROUTE-004` (shock-capture / phase-lag) opened.
- Zone 1/Zone 2 taxonomy (named by `INV-OFEROUTE-011`): **not run** — explicitly
  deferred in the contract so D7 does not appear to close an unrun obligation.

## Honesty summary

1 of 4 cases reproduces (qualified); 3 carry distinct, attributed shortfalls.
Per the package success criterion — *truthful per-case verdicts, not all four
pass* — D7 is executed. No result is reported as validation against nature.
