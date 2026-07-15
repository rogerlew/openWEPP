# Adjudicated CRAP Gate Report

Status: `FAIL`

- Acquisition mode: `fresh`.
- Eligible for current-source closure: `True`.
- Debt assessment: `FAIL`.
- Threshold: CRAP strictly greater than `30` is raw debt.
- Production entries assessed: `8536`.
- Raw rows over threshold: `6`.
- Currently adjudicated rows: `2`.
- Actionable rows: `4`.
- Touched production files: `3`.
- Actionable rows in touched files: `4`.
- Actionable rows outside touched files: `0`.
- Source HEAD: `81770ecb8f9e65702c7401852efa3d7f4682d15a`.
- Touched-file base: `81770ecb8f9e65702c7401852efa3d7f4682d15a`.
- Touched-file head: `WORKTREE`.
- Worktree dirty: `True`.
- CRAP JSON SHA-256: `1b6e23ac57be052b84442c083189d08bdc95dd8f03906c0befddb99b5d29a897`.
- LCOV SHA-256: `ec12316f0484d0ebf26d19758e640ff4e3bd270165d002c31bda95d26d777b5c`.
- Production source manifest SHA-256: `c3b47dcc792c749707ddb8ef5434579e56d556012361d7963247039ee165cc4e`.
- Adjudication registry SHA-256: `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.

## Touched Production Files

| Status | Path |
| --- | --- |
| `M` | `crates/openwepp-assurance/src/cli.rs` |
| `M` | `crates/openwepp-assurance/src/lib.rs` |
| `U` | `crates/openwepp-assurance/src/v2.rs` |

## Adjudicated Rows

| File | Function | Line | CC | Coverage | CRAP | Adjudication |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `crates/openwepp-meteorology/src/error.rs` | `MeteorologyError::fmt` | 35 | 7 | 0 | 56 | `CQR-LOW-L08` |
| `crates/openwepp-sim-contract/src/symbols.rs` | `SymbolAliasRegistryError::fmt` | 67 | 9 | 0 | 90 | `CQR-LOW-L11` |

## Actionable Rows

| File | Function | Line | CC | Coverage | CRAP | Adjudication |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `crates/openwepp-assurance/src/v2.rs` | `validate_schema_document` | 529 | 25 | 76.6667 | 32.9398 | `` |
| `crates/openwepp-assurance/src/v2.rs` | `validate_dependency` | 1262 | 28 | 59.1549 | 81.4238 | `` |
| `crates/openwepp-assurance/src/v2.rs` | `validate_result` | 1370 | 22 | 71.4286 | 33.2886 | `` |
| `crates/openwepp-assurance/src/v2.rs` | `validate_research_object` | 1472 | 20 | 53.3333 | 60.6519 | `` |

## Invalid Or Stale Adjudications

None.
