# Active Suite Run Summary

Status: EXECUTED-HOLD-ACTIVE-RUN.
Evidence mode: Ran.

| Scope | Member | Mode | Status | Exit | Wall | User | Sys | Manifest | Output dir check | Hybrid flag check |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| build |  |  | PASS | 0 |  |  |  | False | None | None |
| run | h2637 | plain | PASS | 0 | 0:39.71 | 39.64 | 0.05 | True | True | True |
| run | h2637 | hybrid | PASS | 0 | 0:33.37 | 33.33 | 0.02 | True | True | True |
| run | mn_corn_h4 | plain | FAIL | 1 | 0:00.06 | 0.05 | 0.00 | False | None | None |

Hold condition:

- Member: `mn_corn_h4`
- Mode: `plain`
- Log: `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-d16-selected-cohort-active-suite-001/artifacts/active-suite-run-logs/mn_corn_h4-plain.time.log`
- First failure line: `CLIHILL-E-011 runtime surface failure for r7c_direct_production_executor: HS-SIMPIPE-E-001 direct runtime day execution failed at lane 1 day 136: direct runtime kernel guard failed in laned_active_rev21_operands: lane 1 day 136 has LAI 0.01182723510043506 > 0 with missing/non-positive typed-management canhgt (rev-21 fail-closed)`
