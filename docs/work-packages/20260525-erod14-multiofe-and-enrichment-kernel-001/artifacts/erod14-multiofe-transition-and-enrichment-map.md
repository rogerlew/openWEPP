# Erod14 multiofe transition and enrichment map

Status: completed
Evidence mode: static

## Static
- Activation gate:
  - `erod14_wave2_enabled = 1` enables Wave-2 runtime lane in `closure_diagnostics`.
- Multi-OFE branch classification inputs:
  - `erod14_case`, `erod14_Qj_minus_1`, `erod14_Vj`, `erod14_Qj`, `erod14_Fh`, `erod14_Fp`.
- Transition geometry/load inputs:
  - `erod14_xtop`, `erod14_xbot`, `erod14_xdetst`, `erod14_ldtop`, `erod14_ldbot`, `erod14_lddend`.
- Hydrology/deposition linkage inputs:
  - `erod14_qout`, `erod14_qin`, `erod14_qostar`, `erod14_slplen`, `erod14_ktrato`, `erod14_ainftc`, `erod14_binftc`, `erod14_cinftc`, `erod14_beta`, `theta`.
- Per-class inputs:
  - `erod14_fall_*`, `erod14_frcflw_*`, `erod14_frac_*`, `erod14_fidel_*`, `erod14_tcf1_*`, `erod14_ssa_class_*`.
- Per-class outputs:
  - `erod14_gend_*`, `erod14_sedmax_*`, `erod14_frcflw_*`, `sed_frac_*`.
- Aggregate outputs:
  - `erod14_sumg`, `ER`.
