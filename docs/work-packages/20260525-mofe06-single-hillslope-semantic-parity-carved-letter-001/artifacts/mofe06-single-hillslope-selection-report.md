# MOFE06 Single-Hillslope Selection Report

Static:
- Selection pool: `/wc1/runs/ca/carved-letter/wepp/output/interchange/audit_hillslope_mofe_daily_closure_all_20260525T175046Z/hillslope_audit_rollup.csv`.
- MOFE eligibility rule: hillslope must have `max(OFE) > 1` in
  `/wc1/runs/ca/carved-letter/wepp/output/H<id>.wat.dat`.

Ran:
- Command used to rank MOFE candidates by closure and verify OFE cardinality:
  - Python script over rollup + `H*.wat.dat` key rows (`OFE,J,Y`).
- Top MOFE candidates by `max_abs_closure_mm`:
  1. `H324` (`max_abs_closure_mm=25.915000000000003`,
     `max_abs_ofe_closure_mm=42.46533199999999`, OFE count `2`)
  2. `H335` (`27.201985`, OFE count `2`)
  3. `H15` (`27.25758999999998`, OFE count `2`)

Decision:
- Selected hillslope: `H324` / runfile family `p324.*`.
- Rationale: lowest observed closure residual among true MOFE candidates in the
  carved-letter cohort.
