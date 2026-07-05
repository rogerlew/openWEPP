# Contract-Test Implementation Evidence

Status: executed
Evidence mode: Static + Ran

## Harness Checks

Static:

- Existing Figure 4 D-val harness: `tools/dval/compare_dval.py`.
- New Figure 9 taxonomy harness: `tools/dval/zone_taxonomy.py`.
  It asserts both published `I*` support and published `Psi*` support
  (`<=10%` relative grid error), then checks Zone 2 near-linearity and
  Zone 1 nonlinearity where the workbook grid permits a fit.
- Existing Rust cited-scalar tests remain in
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`.

Ran:

- Cases 1-3:
  `.venv/bin/python tools/dval/compare_dval.py --case <1..3> --fig4 .../Figure_4.xlsx --crate-dir .`
  passed via subagent logs in this artifact directory.
- Case 2 sensitivity:
  `.venv/bin/python tools/dval/compare_dval.py --case 2 --fig4 .../Figure_4.xlsx --ks 10 --crate-dir .`
  passed with `NS_trace=0.9612086844726946`.
- Zone taxonomy:
  `.venv/bin/python tools/dval/zone_taxonomy.py --fig9 .../Figure_9.xlsx`
  passed and wrote `artifacts/zone-taxonomy-20260705-1545.json`.
- Focused Rust regression:
  `cargo nextest run -p openwepp-hillslope-orchestrator case2_underprediction_is_ks_operand_limited`
  passed: `1 passed, 278 skipped`.

## Contract Surface

These checks cover `SC-OFEROUTE-001#INV-OFEROUTE-011` only. They do not claim
production activation, default activation, or `GAP-OFEROUTE-005` closure.
