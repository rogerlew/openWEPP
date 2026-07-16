# Adjudicated CRAP Gate Report

Status: `PASS`

- Acquisition mode: `fresh`.
- Eligible for current-source closure: `True`.
- Debt assessment: `PASS`.
- Threshold: CRAP strictly greater than `30` is raw debt.
- Production entries assessed: `9392`.
- Raw rows over threshold: `2`.
- Currently adjudicated rows: `2`.
- Actionable rows: `0`.
- Touched production files: `7`.
- Actionable rows in touched files: `0`.
- Actionable rows outside touched files: `0`.
- Source HEAD: `25bcb17f4a62924976a19381e974a36612ed4845`.
- Touched-file base: `25bcb17f4a62924976a19381e974a36612ed4845`.
- Touched-file head: `WORKTREE`.
- Worktree dirty: `True`.
- CRAP JSON SHA-256: `5780a6f5c6b788f8342a6918648e3ed32b055679a53df85b85bc86d43bc8327a`.
- LCOV SHA-256: `99da70468234dcb4638d0582687fa068f68088500ad100b50839a035f2427f53`.
- Production source manifest SHA-256: `72d2fa3d449fc492a05818daa680b548d2aa6bb14b6c0428ab8af6b0e16873ae`.
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

None.

## Invalid Or Stale Adjudications

None.
