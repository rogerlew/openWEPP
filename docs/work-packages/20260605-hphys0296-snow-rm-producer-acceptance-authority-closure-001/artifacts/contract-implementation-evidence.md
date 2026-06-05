# Contract Implementation Evidence

Status: executed
Evidence mode: Static

Static:
- Amended `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  with `INV-SNOWFREEZE-027`, guard-map coverage, `OBL-SNOWFREEZE-P-015`, and
  revision-history versions `27` and review-disposition `28`.
- Amended `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
  with `INV-RUNOFFPART-024`, guard-map coverage, `OBL-RUNOFFPART-P-010`, and
  revision-history versions `35` and review-disposition `36`.
- Amended `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  with `INV-WATBAL-071`, guard-map coverage, `OBL-WATBAL-P-020`, and
  revision-history versions `116` and review-disposition `117`.

Interpretation:
- The contracts now require explicit snow/`RM` acceptance classification after
  HPHYS0295 cumulative-budget ownership.
- Corrected-negative-melt residuals cannot leave the failing set from
  correlation plus internal closure alone. They require per-window
  defective-model verdicts with mechanistic `file:line` root cause in both
  models, reconstruction to named tolerance, independent correctness
  adjudication, and auditable disposition.
- Downstream compensation in WB17/WB18/WB19/WB13 remains invalid.
