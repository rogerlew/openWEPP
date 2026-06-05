# Contract Implementation Evidence

Status: executed
Evidence mode: Static

Static:
- Amended `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  with `INV-SNOWFREEZE-027`, guard-map coverage, `OBL-SNOWFREEZE-P-015`, and
  revision-history version `27`.
- Amended `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
  with `INV-RUNOFFPART-024`, guard-map coverage, `OBL-RUNOFFPART-P-010`, and
  revision-history version `35`.
- Amended `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  with `INV-WATBAL-071`, guard-map coverage, `OBL-WATBAL-P-020`, and
  revision-history version `116`.

Interpretation:
- The contracts now require explicit snow/`RM` acceptance classification after
  HPHYS0295 cumulative-budget ownership.
- Corrected-negative-melt residuals can be accepted only when trace evidence
  explains the residual.
- Downstream compensation in WB17/WB18/WB19/WB13 remains invalid.
