# Rung-3 Frost Defect Handoff

Status: proposed-follow-on
Evidence mode: Static + Ran

Context:
- FROSTVAL01 executed all reachable rung-2 work for `/wc1/runs/al/algebraic-radium` single-OFE hillslopes.
- 37/43 targets are blocked by `HS-RUNTIME-E-062` before hydrology outputs.
- 6/43 targets run successfully but show no frost activation signal under ksflag on/off pairing.
- All 6 runnable targets classify as `frost-break` for annual closure residuals (years 2-7).

Priority follow-on queue:

1. HS-RUNTIME-E-062 lineage-coverage unblock (highest priority)
- Problem: 37 single-OFE hillslopes terminate before frost and closure evaluation.
- Evidence: `run_status.tsv` blocked taxonomy (layer 4 and layer 6 coverage gaps).
- Required outcome: all 43 single-OFE hillslopes produce `.wat.parquet` and `.hbp` outputs.

2. ksflag/frost activation authority check on runnable cohort
- Problem: for `p8,p13,p22,p23,p26,p28`, ksflag on/off runs are identical for tracked terms and `frozwt` remains zero.
- Evidence: `activation_summary.csv`, `off_ksflag_checks.txt`, `off_status.tsv`.
- Required outcome: demonstrate whether frost gating should activate for this run family and, if yes, identify where activation is lost.

3. Closure residual defect analysis under current runnable cohort
- Problem: each runnable prefix has large year-level residual magnitude (`|residual|` up to 133.983394 mm).
- Evidence: `closure_yearly.csv`, `closure_prefix_summary.csv`.
- Required outcome: explicit term-by-term root-cause map for residual drivers, with no imputed zero terms.

Rung continuation recommendation:
- Keep FROSTVAL01 at `executed-hold`.
- Open a dedicated defect-closure package for HS-RUNTIME-E-062 first; without this unblock, Milestone 1 cannot be completed for full scope.
- After unblock, rerun full 43 on/off activation audit, then rerun closure under frost.
