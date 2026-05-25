# MOFE06 Semantic Parity Execution Report

Static:
- Baseline run root: `/wc1/runs/ca/carved-letter`.
- Candidate intent: run openWEPP single-hillslope lane for `p324` and compare
  candidate `H324` WAT surface against legacy baseline `H324.wat.dat`.

Ran:
1. MOFE closure audit execution:
- Command:
  - `python -m tools.hillslope_mofe_daily_closure_audit /wc1/runs/ca/carved-letter/wepp/output/interchange --wepp-id 324 --output-dir /tmp/openwepp_mofe324_semantic_parity/audit_h324`
- Result highlights:
  - `n_ofe_min=2`, `n_ofe_max=2`
  - `rows=2192`
  - `full_physical_requires_scientific_review_days=0`
  - `full_physical_closure_residual_total_mm=-1485.928632`
  - `max_abs_day.audit_full_physical_closure_residual_mm=25.915000000000003`

2. openWEPP candidate generation attempts:
- Attempt A (direct carved-letter `p324.run`):
  - `CLIHILL-E-010 parse failure for run_file: invalid TOML in .../p324.run`
  - Cause: carved-letter `p324.run` is legacy text format, not
    `openwepp-hillslope-runfile-v1` TOML.
- Attempt B (generated TOML runfile; original carved-letter slope/soil):
  - `CLIHILL-E-010 parse failure for slope: token parse error at line 7, column 3: expected integer, got '0.0000'`
- Attempt C (exploratory slope normalization in temp run dir):
  - slope parse unblocked, then
  - `CLIHILL-E-010 parse failure for soil: SOL-E-006 ... expected 9 token(s), found 15`
- Attempt D (exploratory soil-header normalization in temp run dir):
  - parser still blocked on carved-letter header arity mismatch under current
    parser expectations.

3. Semantic comparator execution:
- Not executed.
- Blocking reason: no valid openWEPP candidate WAT surface emitted for `H324`
  under current carved-letter MOFE slope/soil parsing compatibility.

Outcome:
- Lane executed through selection + audit + candidate attempts.
- Final posture: blocked at candidate generation; semantic parity comparison
  cannot be completed for carved-letter MOFE `H324` without parser compatibility
  closure.
