# Contract Implementation Evidence

Status: executed
Evidence mode: Static

Static:
- Amended `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  with `INV-SNOWFREEZE-028`, guard-map coverage, `OBL-SNOWFREEZE-P-016`, and
  revision-history version `29`.
- Amended `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
  with `INV-RUNOFFPART-025`, guard-map coverage, `OBL-RUNOFFPART-P-011`, and
  revision-history version `37`.
- Amended `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  with `INV-WATBAL-072`, guard-map coverage, `OBL-WATBAL-P-021`, and
  revision-history version `118`.

Interpretation:
- HPHYS0297 requires reconstruction against
  `/workdir/wepp-forest_260430_baseline/src/winter.for:434-448`.
- Corrected openWEPP source lineage remains
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:4231-4276`.
- Residuals cannot leave the failing set without reconstruction closure,
  independent correctness rationale, and explicit verdict.
