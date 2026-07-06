# Implementation and Test Evidence

Status: executed-hold
Evidence mode: Static + Ran

D10 closes without production Rust code changes.

Implemented:

- `tools/dval/compare_dval.py` now accepts Case-4-only resolution controls:
  `--cells`, `--sample-dt`, and `--max-dt`.

Not implemented:

- No production/default activation.
- No `OPENWEPP_LANED_SHADOW` activation semantics.
- No solver/cascade correction.
- No D11-D13 work.

Focused validation already run before final gates:

- Ran: `.venv/bin/python -m py_compile tools/dval/compare_dval.py` -> PASS.
- Ran: Case-1 resolution-control negative guard -> expected `exit_code=2`.
- Ran: `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing` after
  reverting the rejected limiter trial -> 47 passed.
- Ran: H2637 shadow test through comparator subagent -> PASS.
- Ran: Case-4 D-val baseline/fine/finer harness commands -> PASS as diagnostic
  commands; metrics fail acceptance and support HOLD.
