# Hold Legitimacy Audit

Status: `EXECUTED-HOLD-DX5-UNRATIFIED`
Evidence mode: Ran.

## Hold Condition

`dx5` is not production-ratified because the selected real-cohort coupled
fine-reference adequacy basis is not fully adequate under the predeclared
one-third thresholds.

## Evidence

The selected real-cohort ladder completed 21/21 rungs with no active closure,
rev-40 clamp-source, or rev-41 positivity failure.

Blocking comparisons:

| Member | Comparison | Surface | Observed | Threshold |
|---|---|---|---:|---:|
| `mn_corn_h4` | `dx2p5_dt300` vs `dx1p25_dt300` | routed hourly shape max L1 | `0.020180511` | `0.016666667` |
| `wa_cascades_forest_h1` | `dx2p5_dt75` vs `dx1p25_dt75` | annual pass-sediment max relative (`tdep:4`) | `0.022131684` | `0.0066666667` |

Supporting non-blocking evidence:

- `dx5_dt300` vs `dx2p5_dt300` passes production candidate tolerances for all
  selected real-cohort members.
- `dx5_dt75` vs `dx2p5_dt75` also passes production candidate tolerances for
  all selected real-cohort members.
- `dx5_dt300` vs `dx5_dt75` passes production tolerance-class timestep-control
  checks for all selected real-cohort members.
- Runtime cost is priced: `dx5_dt300` costs `88.68 s` aggregate real-cohort
  user time versus fixed10 `18.18 s`, about `4.88x`; cost is not the hold
  blocker under the operator's fidelity-first posture.

## Why This Is Outside Safe Closure Here

The package objective allowed a production default flip only after
candidate-vs-adequate-fine-reference evidence closed. The evidence instead
shows that:

- The production-cap spatial reference still carries the known
  `mn_corn_h4` shape adequacy miss.
- The refined-75 reference closes `mn_corn_h4` shape but exposes a WA annual
  pass-sediment adequacy miss.

Changing the one-third adequacy rule, widening routed-shape or sediment
tolerances, changing production `max_dt`, or treating `dx5` candidate passes as
promotion authority without an adequate reference would be tolerance fitting
and would violate `SC-OFEROUTE-001` rev 43. No contract-backed production
correction is in scope because the new WA sediment adequacy miss has not been
mechanism-attributed.

## First Actionable Follow-On

Scaffold a narrow WA annual pass-sediment fine-reference adequacy attribution
package:

- Target member: `wa_cascades_forest_h1`.
- Target comparison: `dx2p5_dt75` vs `dx1p25_dt75`.
- Target surface: annual pass-sediment max relative `0.022131684` on `tdep:4`.
- First actions: identify the member-year/sediment column from
  `mesh-policy-ratification.json`, compare annual and daily pass-sediment
  deltas against routed outlet/shape/closure traces, then classify the miss as
  erosion-consumer sensitivity, routed-water timing/magnitude sensitivity, or
  a deeper active-router numerics issue.
- Binding rule: do not amend tolerances or promote `dx5` until the WA sediment
  adequacy miss is attributed and the fine-reference basis closes.
