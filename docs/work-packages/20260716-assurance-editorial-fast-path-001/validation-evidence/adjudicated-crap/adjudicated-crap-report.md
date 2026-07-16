# Adjudicated CRAP Gate Report

Status: `FAIL`

- Acquisition mode: `fresh`.
- Eligible for current-source closure: `True`.
- Debt assessment: `FAIL`.
- Threshold: CRAP strictly greater than `30` is raw debt.
- Production entries assessed: `9372`.
- Raw rows over threshold: `4`.
- Currently adjudicated rows: `2`.
- Actionable rows: `2`.
- Touched production files: `7`.
- Actionable rows in touched files: `2`.
- Actionable rows outside touched files: `0`.
- Source HEAD: `25bcb17f4a62924976a19381e974a36612ed4845`.
- Touched-file base: `25bcb17f4a62924976a19381e974a36612ed4845`.
- Touched-file head: `WORKTREE`.
- Worktree dirty: `True`.
- CRAP JSON SHA-256: `1a71a820c5456134f807ecf374ff7b5ccde5f8d5a590c62915b4d157dcf44cbc`.
- LCOV SHA-256: `b360f9763b476fba7a29b40b9691884c59ed6615b737c9456af562b630138fc4`.
- Production source manifest SHA-256: `9e19cb291fa78e709777f514d950a843f213d1f8361593f9528d748b672a33e7`.
- Adjudication registry SHA-256: `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.

## Touched Production Files

| Status | Path |
| --- | --- |
| `M` | `crates/openwepp-assurance/src/cli.rs` |
| `M` | `crates/openwepp-assurance/src/error.rs` |
| `M` | `crates/openwepp-assurance/src/lib.rs` |
| `M` | `crates/openwepp-assurance/src/v2.rs` |
| `M` | `crates/openwepp-assurance/src/v2/assembly.rs` |
| `M` | `crates/openwepp-assurance/src/v2/confined.rs` |
| `U` | `crates/openwepp-assurance/src/v2/normalization.rs` |

## Adjudicated Rows

| File | Function | Line | CC | Coverage | CRAP | Adjudication |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `crates/openwepp-meteorology/src/error.rs` | `MeteorologyError::fmt` | 35 | 7 | 0 | 56 | `CQR-LOW-L08` |
| `crates/openwepp-sim-contract/src/symbols.rs` | `SymbolAliasRegistryError::fmt` | 67 | 9 | 0 | 90 | `CQR-LOW-L11` |

## Actionable Rows

| File | Function | Line | CC | Coverage | CRAP | Adjudication |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `crates/openwepp-assurance/src/v2/normalization.rs` | `normalize_report_with_controls` | 155 | 29 | 85.1485 | 31.7549 | `` |
| `crates/openwepp-assurance/src/v2/normalization.rs` | `clone_v2_tree` | 957 | 18 | 66.6667 | 30 | `` |

## Invalid Or Stale Adjudications

None.
