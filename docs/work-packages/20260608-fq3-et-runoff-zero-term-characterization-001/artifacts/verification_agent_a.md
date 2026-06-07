# Verification Agent A

Evidence mode: Ran

## Executed Checks

1. Legacy comparator rerun over 42 prefixes from `/wc1/runs/al/algebraic-radium/wepp/runs`:
   - Result: 42/42 `rc=0`.
2. Legacy WAT interchange parse:
   - Result: 107,394 rows, non-empty.
3. Classification generation:
   - Result files created (`management_group_map.csv`, `per_prefix_term_classification.csv`, `group_term_aggregate.csv`, `fq3_summary.json`).

## Key Assertions Verified

- Corn `Ep` defect count = 36.
- Corn interception legacy-unavailable count = 36.
- `Q` defect count = 35.
- `QOFE` defect count = 35.
