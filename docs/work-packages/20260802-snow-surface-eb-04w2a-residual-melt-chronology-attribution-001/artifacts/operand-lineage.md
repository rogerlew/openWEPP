# Operand Lineage And Reconstruction

Evidence mode: **Ran**.

The direct-production quantities come only from the immutable EB-04W1/W2
selected trace and WAT outputs named and hashed in `experiment-freeze.json`.
Snowbench outputs are not substituted for these operands.

| Lane | Selected factor | Peak ratio | Chronology error (d) | Max direct mass closure (m) | Cold-content window hits | `|a+d|` (m) | `|b+c|` (m) | Early-gap late-input hits |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| Mica Creek | 1.4 | 0.968 | 21.0 | 4.441e-16 | 8/23 | 0.1029 | 0.4623 | Not applicable |
| Niwot | 1.7 | 1.029 | 13.5 | 4.441e-16 | 16/40 | 0.0173 | 0.00868 | 0/27 |
| Paradise | 1.8 | 0.989 | 0.0 | 2.221e-15 | 0/19 | 0.2199 | 1.4613 | Not applicable |
| Snowbird | 2.0 | 0.977 | 23.0 | 1.332e-15 | 12/22 | 0.0359 | 0.1364 | 5/16 |

`a`, `b`, `c`, and `d` are the existing CoE empirical melt-depth terms:
radiation, temperature/cloud, wind/dew point, and rain heat. The table reports
median absolute grouped contributions inside each frozen chronology window;
they are not independently measured energy fluxes.

The maximum independently reconstructed Stage-3 energy residual is
`6.094e-08 J m^-2`, below the frozen `1e-6 J m^-2` bound. Every direct mass
closure is below `1e-12 m`.

The raw site-level cold-content flag is false because the runner separately
medians melt fraction and depth before applying the disjunction. That summary
hides windows meeting either arm: Mica `8/23`, Niwot `16/40`, Paradise `0/19`,
and Snowbird `12/22`. The association is interannual and does not establish a
causal cold-content melt gate.

The raw runner counted all peak windows in its late-input population. The
adjudicated reconstruction enforces an early modeled peak: Niwot has `0/27`
hits and Snowbird `5/16`. Observed SWE gain is net of contemporaneous loss, so
neither a positive nor negative screen uniquely identifies precipitation.
