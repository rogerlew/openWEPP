# HPHYS0202 Worker Handoff

Status: completed  
Evidence mode: Static + Ran

## Immediate Next Actions
1. Reconcile FC/WP residuals (`39/39` fail hillslopes) against baseline
   authoritative path:
   - trace per-layer `thetfc/thetdr/dg` surfaces from soil parser/runtime
     projection through WB11/WB13 publication,
   - compare against `/workdir/wepp-forest_260430_baseline` layer-state lineage
     for the same cohort inputs.
2. Add per-layer FC/WP diagnostic vector export for one representative failing
   hillslope and one non-regressing control case to localize first divergence.
3. Keep FC/WP scope isolated; do not broaden into RM/ET/snow/runoff closure in
   this follow-on lane.
4. Re-run the same 39-hillslope diagnostic bundle after fixes and verify
   `ProfileFCStore`/`ProfileWPStore` fail counts decrease from `39/39`.

## Follow-on package
- `20260529-hphys0205-layer-authoritative-fcwp-correction-closure-001`
  is the authorized follow-up to resolve this residual while keeping
  `thetfc_####`/`thetdr_####` as publication-authoritative symbols.

## Handoff evidence bundle
- Workspace gates: executed in `/home/workdir/openWEPP` during HPHYS0202
  closeout (`fmt`, `clippy`, `test`, `deny` all pass).
- Diagnostic run root:
  `/tmp/hphys0202_20260530T003833Z/parity/`
- Summary:
  `/tmp/hphys0202_20260530T003833Z/parity/reports/hillslope_semantic_summary.json`
