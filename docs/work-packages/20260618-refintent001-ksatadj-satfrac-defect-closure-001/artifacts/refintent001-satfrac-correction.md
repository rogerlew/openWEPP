# REFINTENT001 Saturation-Fraction Correction

Evidence class: Static + Ran

## Before

WB14 formed the conductivity saturation fraction as:

```text
sat_frac = sum(theta_i) / sum(ul_i)
```

That used upper-limit storage as the denominator and did not include source-intent
porosity, coarse-particle correction, residual water, or the two `avsat` caps.

## After

WB14 now accumulates the top two tillage layers as:

```text
theta_storage = sum(st_i)
tillage_depth = sum(dg_i)
avpor = sum(por_i * dg_i) / tillage_depth
avcpm = sum(cpm_i * dg_i) / tillage_depth
avsm15 = sum(thetdr_i * dg_i) / tillage_depth

avsat = theta_storage / tillage_depth + avsm15
if avsat > avpor:
    avsat = avpor * 0.98
if avsat >= avpor * avcpm:
    avsat = avpor * avcpm * 0.99

sat_frac = avsat / (avpor * avcpm)
```

Implementation references:

- Operand loading and accumulation: `02_ksat_adjustment.rs:38-89`
- Sum validation: `02_ksat_adjustment.rs:258-289`
- Corrected formula and caps: `02_ksat_adjustment.rs:292-325`
- Direct `thetfc` / `thetdr` averages: `02_ksat_adjustment.rs:331-335`
- Integration expected-value oracle: `wb14_infiltration_hyetograph_kernel_contract.rs:231-255`

The effective-conductivity branches remain structurally unchanged after this
`sat_frac` substitution.

## Non-alias proof

The unit fixture intentionally separates the old surrogate from the source-intent
formula:

- source intent: `0.41 / 0.55 = 0.745454545454...`
- old surrogate: `0.06 / 0.40 = 0.15`

The test asserts the source-intent value and rejects accidental aliasing with the
old formula.
