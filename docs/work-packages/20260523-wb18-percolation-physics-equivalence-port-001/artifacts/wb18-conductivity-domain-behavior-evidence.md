# WB18 Conductivity Domain Behavior Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Conductivity-Domain Behaviors Implemented
- Saturation-factor branch:
  - for `stz < 0.95`: `fx = max(stz^Bi, 0.002)`
  - for `stz >= 0.95`: `fx = 1.0`
- Conductivity scaling:
  - `ks_adjusted = ssc * fx`
- Daily timestep scaling:
  - `pei_pre = min(86400 * ks_adjusted, theta - fc)`
- Lower-layer saturation restriction:
  - upper-layer transfer scaled by `sqrt(max(1 - stu_lower, 0))`

## Guarded Domain Conditions
- `nsl` integral and `>= 1`
- `theta >= 0`
- `ul > 0`
- `0 <= fc <= ul`
- `ssc > 0`
- finite ratios and finite computed fluxes

## Ran Evidence
- `cargo test --test wb18_percolation_physics_kernel_contract`:
  - nominal conductivity-domain vector passed
  - non-finite conductivity (`wb18_perc_ssc_0002 = NaN`) failed with
    `HKERNEL-WB11-PERC-E-002`
  - domain-invalid upper limit (`wb18_perc_ul_0002 = 0.0`) failed with
    `HKERNEL-WB11-PERC-E-003`
