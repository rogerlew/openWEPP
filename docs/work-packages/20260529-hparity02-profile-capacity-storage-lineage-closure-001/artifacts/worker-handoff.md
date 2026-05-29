# HPARITY02 Worker Handoff

Status: completed  
Evidence mode: Static + Ran

## Immediate Next Actions
1. Close residual `ProfileFCStore` lineage mismatch (`27/39` fail hillslopes):
   - trace normalized-layer `thetfc` derivation against baseline
     `input.for` + `scon.for` with per-layer diagnostic vectors,
   - verify `SoilLayer.fc_measured/wp_measured` source selection for 9002
     surfaces and any missing legacy compatibility transformations.
2. Close residual `ProfileWPStore` mismatch (`1/39` fail hillslope):
   - isolate hillslope 7 diagnostics and reconcile thetdr path.
3. Resolve control-column regression signal in rerun evidence (`Q`, `QOFE`):
   - confirm candidate generation lane parity assumptions and input identity
     against baseline partitions before attributing kernel residual cause.
4. Re-run the same 39-hillslope semantic closure bundle after fixes:
   - keep year offset `2012`,
   - regenerate summary and verify `MEASURE-HP02-001..004` pass.

## Handoff evidence bundle
- Gate logs: `/tmp/hparity02_20260529T204555Z/gates`
- Candidate outputs and semantic reports:
  `/tmp/hparity02_20260529T204555Z/parity`
- Package-level summary:
  `/tmp/hparity02_20260529T204555Z/parity/reports/hillslope_semantic_summary.json`
