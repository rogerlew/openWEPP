# D7 Execution Report — Per-Case Verdicts (S3–S5)

Evidence: **Ran** — `ofe_routing::dval` case runs + `tools/dval/compare_dval.py`
(sha256-verified reads of `Figure_4.xlsx`, offline `NS_trace`, derived scalars
only). Committed cited-scalar tests in `ofe_routing::dval`. `NS_trace` =
openWEPP vs enhanced-WEPP **model trace** (method fidelity); the paper's
`Ef_obs` (enhanced-vs-observed) is **not** recomputed and stays a citation.

> **Correction (Codex execution review).** The first execution had a Case-4
> forcing bug (`run_iwagaki` fed the lateral-supply rate into the skin-term
> rainfall intensity `I`, but Iwagaki has no rain → `I` must be 0). That
> inflated the skin friction and produced a spurious ~5-6 s "shock lag" that
> was wrongly attributed to the solver (`GAP-OFEROUTE-004`, now **WITHDRAWN**).
> All Case-4 numbers below are the corrected `I = 0` results. The Case-1
> verdict is also corrected: the required rise-limb shape gate is now applied
> and it **fails**, so Case 1 is downgraded to PARTIAL.

## Results (corrected)

| Case | `NS_trace` | peak ratio | rise 10-90% ow/ref | verdict |
|---|---|---|---|---|
| 1 bare | 0.868 | 1.07 | **5000 / 3580 s (~40% slow)** | **PARTIAL** — steady magnitude reproduces, rise-limb shape fails |
| 2 isolated | 0.454 | 0.75 | — | operand-limited (−25%) |
| 3 vegetation | 0.538 | 0.55 | — | does-not-reproduce (−45%) + S0 magnitude caveat |
| 4 shock (`I=0`) | ~0.30 (@k_o 200) | 0.79 | 20.6 / 20.9 s (**matches**) | operand-limited (unspecified k_o) |

**Zero cases cleanly reproduce** (NS ≥ 0.85 **and** shape gate). Per the
package success criterion — *truthful per-case verdicts, not all four pass* —
D7 is executed.

## Case 1 (bare) — PARTIAL

At the literature Ks = 6.8 mm/h (Rawls silt-loam, untuned), `NS_trace = 0.868`
with peak +7%. But `NS` is **plateau-dominated** (5 h steady rain fills the
comparison window), and the **rising-limb shape gate fails**: openWEPP's 10-90%
rise time is ~5000 s vs the enhanced ~3580 s (~40% slow). So the steady
magnitude reproduces but the transient does not, and the fit is knife-edge in
Ks (`NS` −0.51 @ Ks 2, 0.868 @ 6.8, 0.37 @ 10). Verdict: **PARTIAL** — not a
clean reproduction under the NS-**and**-shape acceptance model. (This is the
NS-necessary-not-sufficient failure mode the scaffold review CX-D7-002
anticipated.)

## Case 2 (isolated) — operand-limited

`NS_trace 0.454`, peak −25%. Sandy/gravel Ks and the isolated-roughness
form/wave operands are loosely constrained; not certifiable without tighter
operands.

## Case 3 (vegetation) — does-not-reproduce (caveated)

`NS_trace 0.538`, peak −45%, plus the S0 magnitude anomaly (enhanced col-8
peak exceeds the plot's I·L). Shape-only; does-not-reproduce.

## Case 4 (Iwagaki shock) — operand-limited (corrected)

With the corrected `I = 0` forcing (lateral supply is excess, not rain):

| k_o | 50 | 100 | 150 | 200 | 300 |
|---|---|---|---|---|---|
| `NS_trace` | 0.115 | 0.157 | 0.298 | **0.301** | 0.063 |
| peak ratio | 0.54 | 0.62 | 0.85 | 0.79 | 0.61 |
| t_peak (s) | 33 | 36 | 38 | **28** | 28 |
| rise 10-90% (s) | 28.3 | 30.4 | 31.1 | **20.6** | 20.3 |

At k_o ~ 200 the **timing (28 s vs 26 s) and rise shape (20.6 s vs 20.9 s)
reproduce**; the residual is peak magnitude (~20% low) and moderate `NS_trace`
(~0.30). The shortfall tracks the **unspecified flume k_o** (operand-limited),
not a solver defect. The earlier solver-lag attribution is **withdrawn**.

Two residual solver observations (noted, not GAP-promoted):
- The peak is **k_o-noisy** (non-monotonic peak ratio), and the solver's
  internal `time_to_peak_s` disagrees with the sampled-hydrograph peak by ~9 s
  — a shock-capture multi-modality worth a look if Case 4 is revisited with a
  known k_o.

## S2 — skin `I`/`ν` unit convention (`INV-OFEROUTE-002`) — OPEN, not confirmed

D7 did **not** audit the skin-term convention, and the intensity-usage bug
above shows the `I` path was not exercised correctly. The skin term
`f_s = (3393 I^0.407 + k_o)/Re` is **k_o-dominated** for Cases 1-3 (k_o = 500;
the `I` term ≈ 60 for Case 1), so Case-1 reproduction does **not** validate the
convention. `INV-OFEROUTE-002` / `GAP-OFEROUTE-002` are corrected to mark the
convention **unconfirmed / audit open** — D7 does not close it.

## S5 contract disposition (corrected)

- `INV-OFEROUTE-011`: **PARTIAL, not closed** — Case 1 partial, Cases 2-3
  operand-limited/caveated, Case 4 operand-limited. Zero clean reproductions.
- `GAP-OFEROUTE-004`: **WITHDRAWN** (forcing-bug artifact).
- `INV-OFEROUTE-002` / `GAP-OFEROUTE-002`: skin convention unconfirmed, open.
- Zone 1/2 taxonomy: **not run**, explicitly deferred.

## Honesty summary

The first pass over-claimed (a solver GAP from a forcing bug; a "REPRODUCES"
that skipped its shape gate). Corrected: **zero cases cleanly reproduce**;
each shortfall is attributed (operand-limited, shape-gap, or open convention)
without a manufactured solver defect. No result is reported as validation
against nature.
