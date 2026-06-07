# FDMC01 Characterization Verdict

Status: complete

Evidence mode: Ran.

## Verdict

`Materially off`.

The openWEPP freeze-index frost-depth proxy is not crude-but-close against legacy heat-flow depth behavior on the frost-active `algebraic-radium` single-OFE cohort.

## Why

- Depth ceiling mismatch is systematic: openWEPP is hard-capped at `200 mm`; legacy peaks span `240..503.2 mm` (mean `414.22 mm`).
- All 43 prefixes exceed the open cap under legacy (`43/43`).
- Depth-series mismatch remains large on winter-observed days:
  - mean MAE `123.81 mm`
  - mean RMSE `146.44 mm`
  - median correlation `0.133`
- Frozen-duration mismatch is systematic:
  - openWEPP `frozwt>0` mean `1017` days
  - legacy `frozwt>0` mean `759.37` days
  - open - legacy mean `+257.63` days

Together, these show materially different frost magnitude and persistence behavior, not a small approximation error.

## Promotion Recommendation

Recommend target `(1)` from the backlog item:

Promote to a heat-flow parity Defect-Closure ExecPlan for frost-depth model migration (legacy `frostn` lineage) under `SC-SNOWFREEZE-001` authority.

Do not sanction the current proxy by contract as-is.

## Comparator Distinction (ADR-0017 posture)

Legacy was used here only as a sizing yardstick for an acknowledged Stage-2 model-fidelity gap. This characterization does not make legacy a normative acceptance target. Any corrective implementation remains contract-first and must be validated against canonical `SC-*` authority.

## Evidence

- `artifacts/frost-depth-characterization-ledger.md`
- `artifacts/frost_depth_characterization_metrics.csv`
- `artifacts/frost_depth_timeseries_pairs.csv`
- `artifacts/frost_depth_characterization_summary.json`
