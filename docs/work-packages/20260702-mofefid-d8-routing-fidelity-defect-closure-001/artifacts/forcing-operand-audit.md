# D8 Forcing and Operand Audit

Every item starts with forcing, operand, and unit confirmation before solver or
kernel attribution.

## D8-1 Skin `I`/`nu` Convention

Static:

- R-63 states `Re = q / nu`, where `q` is unit discharge and `nu` is kinematic
  viscosity, and states rainfall intensity `I` in `m/s` for equation (2)
  (`references/copyrighted/Papanicolaou2018.md:69-71`).
- D7 Cases 1-3 have `k_o = 500`; the `I` term is about 38.5 for Case 1
  (`60 mm/h`) and about 42 for `74 mm/h`, so those cases are `k_o` dominated.
  They cannot validate the `I` unit convention.

Ran:

- Added `shen_li_low_ko_vector_pins_si_rainfall_intensity`.
- Added `shen_li_negative_intensity_is_not_silently_zeroed`.
- Focused `ofe_routing` nextest passed.

Verdict: corrected. The local SI `I` convention is pinned against R-63 by a
low-`k_o` vector where `I` dominates; negative intensity is no longer silently
normalized in the pure helper. Primary Shen & Li / Hirsch / Woolhiser provenance
remains `GAP-OFEROUTE-002`.

## D8-2 Iwagaki Metrics and Peak Noise

Forcing / operands:

- Static: R-63 §3.1.4 states no rain; water is supplied laterally for 10 s at
  0.108, 0.0638, and 0.08 cm/s over three 8 m sections. D7 already corrected
  `run_iwagaki` to `rainfall_intensity_m_s = 0`.
- Static: `k_o` remains unspecified for the Iwagaki flume; scans are diagnostic,
  not tuning authority.
- Ran: after the sampler correction, `compare_dval.py --case 4 --ko 200`
  reports `NS_trace=0.262677`, peak ratio `0.837`, sampled `t_peak=37 s`, and
  rise `29.4 s` vs reference `20.9 s`.

Mechanism:

- The previous sampled-hydrograph metric stamped step-end values onto sample
  times crossed by the step. D8 corrected this to interpolate between solver
  step endpoints.
- After correction, sub-step and sampled `t_peak` reconcile within the sample
  interval for the default D-val run, so the metric disagreement itself is
  corrected.
- Peak/timing remain materially resolution-sensitive. Example: default
  `120 cells, sample 1.0 s, max_dt 0.5 s` gives sampled peak `6.803e-3` at
  `37 s`; `240 cells, sample 0.25 s, max_dt 0.25 s` gives sampled peak about
  `8.700e-3` at `39 s`.

Verdict: corrected metric + declared boundary for shock-capture numerics.
`GAP-OFEROUTE-005` records the unresolved Case 4 numerical convergence /
limiter boundary. A simple in-envelope increase of the implicit `alpha`
iteration count was tried and rejected because it broke existing
steady/cascade/conservation tests, so D8 does not land a surrogate numerical
change.

## D8-3 Cases 2-3 Under-Prediction

Forcing / operands:

- Case 2: rainfall `74 mm/h`, slope `2.2%`, length `6 m`, `k_o=500`,
  `C_d=1`, `D_r=0.06 m`, `lambda=0.2`; forcing channels are rainfall to
  Green-Ampt and rainfall intensity to the skin term.
- Case 3: rainfall `74 mm/h`, slope `7%`, length `6.1 m`, width `1.8 m`
  from the D01 docx fixture, `k_o=500`, `LAI=1`, `h_c=0.1 m`, `C_d=1`.
- R-63 equations (4)-(6) match the implemented form, wave, and vegetation
  formulas. Existing and D8 tests cover formula evaluation and D-val sensitivity.

Ran:

- Case 2 default `Ks=20 mm/h`: `NS_trace=0.453954`, peak ratio `0.747`.
- Case 2 `Ks=10 mm/h`: `NS_trace=0.961209`, peak ratio `0.922`.
- Case 3 `Ks=0`: peak ratio `0.740`, still below the enhanced trace because
  the enhanced peak exceeds the recorded rainfall-length ceiling.

Verdicts:

- Case 2: operand-limited (no friction defect). The shortfall is materially
  controlled by uncertain sandy/gravel `Ks`; plausible lower `Ks` reproduces
  the trace without changing form/wave kernels.
- Case 3: declared boundary. The enhanced-WEPP trace/cut-point remains
  inconsistent with the recorded rainfall-length ceiling, so D8 cannot use it
  as a kernel defect verdict.

## D8-4 Case 1 Rising-Limb Lag

Forcing / operands:

- Case 1 rainfall `60 mm/h`, slope `9%`, length `7.5 m`, `k_o=500`.
- Green-Ampt operands are texture-derived: `Ks=6.8 mm/h`, `psi=0.167 m`,
  `delta_theta=0.35`. The paper does not provide the initial moisture/suction
  state needed to make this a unique transient operand.

Ran:

- Default: `NS_trace=0.868483`, peak ratio `1.066`, rise `4999.7 s` vs
  enhanced `3579.9 s`.
- Impermeable/routing-only (`Ks=0`) diagnostic: rise `77.4 s`, proving routing
  celerity is not the source of the slow limb.
- `Ks` sensitivity remains high: D7/D8 scans show lower `Ks` speeds the limb but
  over-raises peak; higher `Ks` preserves/lower peak but slows the limb further.

Verdict: operand-limited (no routing defect). The slow limb is attributable to
Green-Ampt infiltration operand uncertainty (suction storage / initial moisture
and `Ks`) rather than kinematic-wave routing celerity or the cascade coupling.
