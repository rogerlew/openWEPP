# Adjudicated CRAP Gate Report

Status: `FAIL`

- Acquisition mode: `fresh`.
- Eligible for current-source closure: `True`.
- Debt assessment: `FAIL`.
- Threshold: CRAP strictly greater than `30` is raw debt.
- Production entries assessed: `8686`.
- Raw rows over threshold: `3`.
- Currently adjudicated rows: `2`.
- Actionable rows: `1`.
- Touched production files: `6`.
- Actionable rows in touched files: `1`.
- Actionable rows outside touched files: `0`.
- Source HEAD: `22fb7dfbafdb9e82a42afe0a5356b4c923a45232`.
- Touched-file base: `22fb7dfbafdb9e82a42afe0a5356b4c923a45232`.
- Touched-file head: `WORKTREE`.
- Worktree dirty: `True`.
- CRAP JSON SHA-256: `e55b563823915742b1df42137eba39747568bd67bd7c7ade6e906714cd1c17fe`.
- LCOV SHA-256: `05093316aedf4e63fb979fb63ac44243ba1a5072faf41d70a4af468c6b3bd956`.
- Production source manifest SHA-256: `ff4fb8cfb375dd478aa7158d3408e90d574e0aeb26ec93d2e63071660fe18ecb`.
- Adjudication registry SHA-256: `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.

## Touched Production Files

| Status | Path |
| --- | --- |
| `M` | `crates/openwepp-assurance/src/cli.rs` |
| `M` | `crates/openwepp-assurance/src/engine.rs` |
| `M` | `crates/openwepp-assurance/src/lib.rs` |
| `M` | `crates/openwepp-assurance/src/v2.rs` |
| `U` | `crates/openwepp-assurance/src/v2/confined.rs` |
| `U` | `crates/openwepp-assurance/src/v2/planner.rs` |

## Adjudicated Rows

| File | Function | Line | CC | Coverage | CRAP | Adjudication |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `crates/openwepp-meteorology/src/error.rs` | `MeteorologyError::fmt` | 35 | 7 | 0 | 56 | `CQR-LOW-L08` |
| `crates/openwepp-sim-contract/src/symbols.rs` | `SymbolAliasRegistryError::fmt` | 67 | 9 | 0 | 90 | `CQR-LOW-L11` |

## Actionable Rows

| File | Function | Line | CC | Coverage | CRAP | Adjudication |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `crates/openwepp-assurance/src/cli.rs` | `execute` | 61 | 27 | 75.5102 | 37.7074 | `` |

## Invalid Or Stale Adjudications

None.
