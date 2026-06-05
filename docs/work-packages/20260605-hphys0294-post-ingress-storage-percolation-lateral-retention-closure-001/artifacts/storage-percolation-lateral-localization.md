# Storage/Percolation/Lateral Localization

Status: executed-hold
Evidence mode: Static + Ran

Static:

- HPHYS0294 requires `SC-PERC-001#INV-PERC-019` and
  `SC-WATBAL-001#INV-WATBAL-069` before assigning residual ownership.

Ran:

- Full suite and targeted traces under `/tmp/hphys0294_full_20260605T050323Z`.

Findings:

- WB18 aggregate identity is internally closed on H1/H7/H39 target rows:
  `wb18_recomputed_minus_wb11_m = 0`.
- WB18 `D=Pe` is internally closed on target rows; `ΔDp` is approximately
  `+0.0048 mm`, too small and too uniform to explain storage residuals.
- WB19 target/unrealized lineage is internally closed on target rows:
  `wb19_q_lateral_unrealized_m = 0`.
- `latqcc` residual is material but not directionally sufficient as the storage
  owner: H1 has positive storage residual with positive `latqcc` residual,
  while H7/H39 have negative storage residuals with mixed/negative `latqcc`
  residuals.
- HPHYS0293 snow/`RM` residual masks remain material on spring 2014 target
  rows and must remain excluded from WB18/WB19 compensation.

Disposition:

- No production WB18/WB19 patch is justified in this package.
- Continue with a cumulative storage-budget ownership package that attributes
  row-to-row storage deltas across WB17 `Ep/Es`, WB18 `D`, WB19 `latqcc`, and
  excluded snow/`RM` masks before changing process code.
