# WAT Residual Attribution Ledger

Status: corrected

Evidence mode: executed

Purpose: attribute `WBVAL06-SINGLE-OFE-WAT-CONSERVATION-RESIDUAL` to a named
mechanism or legitimate branch-out boundary.

Attribution:

- Defect: `WBVAL06-SINGLE-OFE-WAT-CONSERVATION-RESIDUAL`.
- Mechanism: omitted daily canopy/residue interception flux `I` from WAT
  publication and the validation identity.
- Ownership: in-envelope WAT/WB13 publication and water-balance accounting.
- Authority: `SC-WATBAL-001` v146, `SC-EVAP-001`, and unit-boundary registry
  entries for `Interception`.
- Protected boundary check: no snow physics-magnitude, ET, percolation, runoff,
  or climate producer edits were made.

Seven-gate result:

| Gate | Result | Evidence |
|---|---|---|
| Reproduction | pass | Post-SNOWSCI old identity max `26.79080937662684 mm` |
| Mechanism | pass | Annual residual equals omitted annual `I` |
| Ownership | pass | WAT/WB13 publication surface |
| Authority | pass | `SC-WATBAL-001` explicit `I` closure term |
| Safety | pass | Required typed finite/nonnegative `I`; no clamping or tuning |
| Testability | pass | WAT schema/unit and runner publication tests |
| Validation | pass | 22/22 WAT emitters clean with `Interception` |

Static:

- The old WAT parquet surface had no daily `Interception` column; downstream
  identity calculations could not consume the authoritative `I` term.
- `InterceptionStorage` is a nullable storage surface and was not populated in
  the validation outputs; it is not a substitute for daily flux `I`.

Ran:

- Corrected validation rollup:
  `/tmp/wbval06_interception_after_20260607T000000Z/reports/wbval06_interception_rollup.json`.
- Prefix summary:
  `/tmp/wbval06_interception_after_20260607T000000Z/reports/wbval06_interception_prefix_summary.csv`.
