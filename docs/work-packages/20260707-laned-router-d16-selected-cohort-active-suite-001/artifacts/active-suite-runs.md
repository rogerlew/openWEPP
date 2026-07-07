# Active Suite Runs

Status: EXECUTED-HOLD-ACTIVE-RUN. Evidence mode: Ran.

## Corrected Runner

Ran:

```text
/home/workdir/wepppy/.venv/bin/python docs/work-packages/20260707-laned-router-d16-selected-cohort-active-suite-001/artifacts/run_active_suite.py
```

Result: exit code `1`, stopped on first active-run failure after two passing
H2637 runs.

Machine-readable command log:

- `artifacts/active-suite-command-log.json`

Raw logs:

- `artifacts/active-suite-run-logs/build.log`
- `artifacts/active-suite-run-logs/h2637-plain.time.log`
- `artifacts/active-suite-run-logs/h2637-hybrid.time.log`
- `artifacts/active-suite-run-logs/mn_corn_h4-plain.time.log`

## Command Results

| Scope | Member | Mode | Status | Exit | Wall | User | Sys | Hybrid flag check |
|---|---|---|---:|---:|---:|---:|---:|---:|
| build | | | PASS | `0` | | | | |
| run | `h2637` | plain | PASS | `0` | `0:39.71` | `39.64` | `0.05` | `true` |
| run | `h2637` | hybrid | PASS | `0` | `0:33.37` | `33.33` | `0.02` | `true` |
| run | `mn_corn_h4` | plain | FAIL | `1` | `0:00.06` | `0.05` | `0.00` | |

The H2637 hybrid manifest confirms `hybrid_implicit_stepping = true`.

## Stop Condition

`mn_corn_h4` active plain failed before publication outputs were completed:

```text
CLIHILL-E-011 runtime surface failure for r7c_direct_production_executor: HS-SIMPIPE-E-001 direct runtime day execution failed at lane 1 day 136: direct runtime kernel guard failed in laned_active_rev21_operands: lane 1 day 136 has LAI 0.01182723510043506 > 0 with missing/non-positive typed-management canhgt (rev-21 fail-closed)
```

The remaining selected members were not counted as completed suite evidence
because the package runner stops on first hard active-run failure.
