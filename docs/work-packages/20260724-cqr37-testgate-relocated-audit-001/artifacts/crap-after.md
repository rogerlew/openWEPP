# CRAP After

Status: `PASS`

Ran:

```text
bash tools/release/run_adjudicated_crap_gate.sh \
  --scope affected \
  --package openwepp-gate-planner \
  --nextest-profile affected \
  --base-ref 79be17ec \
  --output-dir target/cqr37-testgate-relocated-audit-r2
```

Result:

| Function | Coverage | CC | CRAP |
| --- | ---: | ---: | ---: |
| `validate_relocated_audit` | 100% | 8 | 8 |

- report status: `PASS`
- raw rows above `30`: `0`
- actionable rows: `0`
- source-manifest before/final SHA-256:
  `c95e9549c74c9b3d18adf12810a84aec33041568d647caf00dcb9e95ed51d6d8`
- run interval: `2026-07-24T01:39:00Z` to `2026-07-24T02:26:55Z`

The first attempt exited `100` before metric publication because the
uncommitted test change correctly triggered
`GATE-COMMITTED-CHECKOUT-NOT-EXACT`. Commit `dd649b78` established the clean
exact head; the unchanged command then passed.
