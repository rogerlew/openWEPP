# H2637 `latqcc` Operand Plausibility

Package:
`20260618-stage2-latqcc-h2637-magnitude-001`

## Evidence

Static:

- WEPP Chapter 6 and Dun 2009 support a Darcy-style lateral flux controlled by
  equivalent conductivity, saturated/drainable thickness, slope gradient, and
  hillslope length.
- `SC-SUBHYD-001` requires finite, non-negative lateral flux operands and
  layer-bounded withdrawal above
  `drfc_i = fc_i + (1 - coca_i) * dg_i`.
- Existing `cas_l4_subhyd_*` suites test response, layer-pool withdrawal caps,
  and FC/WP consistency. They do not define an absolute H2637 magnitude bound.

Ran:

- H2637 selected-day operand trace from `/tmp/stage2_latqcc/diag3`.
- The top H2637 rows are potential-limited: `q == potential == target`; the
  storage pool and capacity totals are larger than `q`.

## Conductivity

Observed:

- `wb19_lateral_ssh_* == wb18_perc_ssc_*` in every traced row.
- Conductivity range: `9.174305555555549e-06..9.174319444444445e-05 m/s`.
- The 24-substep equivalent `Ke` is derived from the traced
  `conductivity_depth_sum` and `saturated_depth_sum`; recomputing the equation
  from those operands matches the emitted potential within
  `4.163336342344337e-17 m`.

Plausibility verdict:

- No openWEPP conductivity-inflation defect was proven. The lateral conductivity
  is the soil conductivity exposed to WB19, not a larger traced `ssh` override.
- Absolute conductivity realism for H2637 remains a contract/external-authority
  question because the current suite does not supply a site-specific or
  class-specific absolute bound for the H2637 forest soil.

## Drainable Thickness / Active Saturated Depth

Observed:

- Layer thicknesses are `0.2 m`; traced profile depth is `1.8 m`.
- `fcdep_before` is emitted per substep and used in the recomputed potential.
- Capacity-active layer-substep counts range `24..215`; conductivity-active
  counts range `24..212`.
- WB19 capacity totals range `0.25765010110832304..12.912830680839656 m`.
- Minimum available pool across substeps ranges
  `0.1744486720952016..0.5062229379024412 m`.

Plausibility verdict:

- No drainable-thickness invariant violation was found in the traced rows.
  Active depth is layer-bounded and the realized lateral `q` does not exceed
  the pool or capacity limits.
- The high `latqcc` rows are not caused by a storage cap failure; they are
  potential-limited.

## `drfc` Threshold / Drainable Water

Observed static layer ranges:

- `fc`: `0.0135188818707163..0.021510326849809504 m`
- `ul`: `0.07250083428909052..0.12705380676149106 m`
- `drfc`: `0.02005886524571629..0.028172965849809495 m`
- `fzdrfc`: `0.01476865949419295..0.028172965849809495 m`
- traced final `theta`: `0.014638897786115528..0.12331678828424096 m`
- `watyld`: `0.1706334131690198..0.339395182626328`

Layer withdrawals are bounded and sum back to `q` within
`1.3877787807814457e-17 m`. The earlier external suspicion that FC was
"2x too low" was not reproduced as an operand defect in this package.

Plausibility verdict:

- No `drfc`/FC threshold defect was found. The traced threshold is the
  contract formula `fc + (1-coca) * dg`, with frozen adjustment reflected in
  `fzdrfc`.
- As with conductivity, this validates formula lineage and bounds, not the
  absolute physical correctness of H2637's magnitude.

## Overall Operand Plausibility

No kernel-domain, storage-bound, threshold-lineage, or conductivity override
defect was found in the traced H2637 rows. The operands are internally
coherent under `SC-SUBHYD-001`.

However, existing external authority is response-oriented. It proves monotonic
or bounded behavior, not that a 55-72 mm daily lateral event or a 71% annualized
H2637 outlet partition is absolutely correct for this site. That is a
contract gap, not an openWEPP defect proven by this package.
