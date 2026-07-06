# Contract-Test Implementation Evidence

Status: executed
Evidence mode: Static + Ran

D10 added Case-4-only resolution controls to `tools/dval/compare_dval.py`:
`--cells`, `--sample-dt`, and `--max-dt`. The harness rejects those controls
for Cases 1-3 because D10 does not own Cases 1-3.

Harness validation:

- Ran: `.venv/bin/python -m py_compile tools/dval/compare_dval.py` -> PASS.
- Ran: `compare_dval.py --case 4 --ko 200 --cells 120 --sample-dt 1.0
  --max-dt 0.5 ...` -> PASS; metrics in
  `artifacts/case4-ko200-c120-s1-md05.log`; the JSON records
  `resolution_controls` and `dval_command`.
- Ran: `compare_dval.py --case 4 --ko 200 --cells 240 --sample-dt 0.25
  --max-dt 0.25 ...` -> PASS; metrics in
  `artifacts/case4-ko200-c240-s025-md025.log`; the JSON records
  `resolution_controls` and `dval_command`.
- Ran: `compare_dval.py --case 4 --ko 200 --cells 480 --sample-dt 0.125
  --max-dt 0.125 ...` -> PASS; metrics in
  `artifacts/case4-ko200-c480-s0125-md0125.log`; the JSON records
  `resolution_controls` and `dval_command`.
- Ran: `compare_dval.py --case 1 --cells 120 ...` -> expected rejection
  `exit_code=2`, `{"error": "resolution controls are Case-4-only"}` in
  `artifacts/case1-resolution-control-rejection.log`.

No production Rust contract-derived test was added because the contract gate
closed as source-authority HOLD before a production correction rule existed.
