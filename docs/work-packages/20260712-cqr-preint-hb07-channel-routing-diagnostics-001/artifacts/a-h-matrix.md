# A–H Closure Matrix

| Family | Terminal evidence |
| --- | --- |
| A — nominal | Exact shape geometry; fresh/carried variable MC state and W11C execution. |
| B — boundaries | Geometry, Manning, qref, bracket, celerity, `dencx`, denominator, and `cx = -10` boundary. |
| C — regimes | Shapes 1–3; static/variable; fresh/carried; admissible/rejected grids and timesteps. |
| D — invalid domain | Invalid shape/geometry/Manning/state operands and `cx < -10` reject exactly. |
| E — missing seam | Missing prior state is bit-identical to explicit deterministic initialization. |
| F — non-finite | NaN and positive/negative infinity preserve first-invalid symbol priority. |
| G — conservation | Independent `qref`, `c0..c4`, `q1`, coefficient, volume and storage reconstruction. |
| H — fail closed | Clamp removed; exact `cx` E-003; no clip, damping, repair or static fallback. |

