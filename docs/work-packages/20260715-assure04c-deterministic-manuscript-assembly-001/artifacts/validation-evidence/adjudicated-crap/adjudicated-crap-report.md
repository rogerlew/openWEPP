# Adjudicated CRAP Gate Report

Status: `PASS`

- Acquisition mode: `fresh`.
- Eligible for current-source closure: `True`.
- Debt assessment: `PASS`.
- Threshold: CRAP strictly greater than `30` is raw debt.
- Production entries assessed: `8948`.
- Raw rows over threshold: `2`.
- Currently adjudicated rows: `2`.
- Actionable rows: `0`.
- Touched production files: `7`.
- Actionable rows in touched files: `0`.
- Actionable rows outside touched files: `0`.
- Source HEAD: `e704f0202278ebb86c6a8c667caf73d599be04ab`.
- Touched-file base: `e704f0202278ebb86c6a8c667caf73d599be04ab`.
- Touched-file head: `WORKTREE`.
- Worktree dirty: `True`.
- CRAP JSON SHA-256: `f24ed50bf5754912e5f1a16ce635d9cccf67f161cf05c1bae65cbb1e828e35d3`.
- LCOV SHA-256: `0f39a4cc95a527c67fd3ad4d7a3d6721925a1d282e8fb661361a3313c177585f`.
- Production source manifest SHA-256: `ed4213f8be4d1921740658865f4f3ec12cc1804b4c8d7e64ff16d9d7ae9c5d5e`.
- Adjudication registry SHA-256: `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.

## Touched Production Files

| Status | Path |
| --- | --- |
| `M` | `crates/openwepp-assurance/src/cli.rs` |
| `M` | `crates/openwepp-assurance/src/error.rs` |
| `M` | `crates/openwepp-assurance/src/lib.rs` |
| `M` | `crates/openwepp-assurance/src/v2.rs` |
| `U` | `crates/openwepp-assurance/src/v2/assembly.rs` |
| `M` | `crates/openwepp-assurance/src/v2/confined.rs` |
| `M` | `crates/openwepp-assurance/src/v2/planner.rs` |

## Adjudicated Rows

| File | Function | Line | CC | Coverage | CRAP | Adjudication |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `crates/openwepp-meteorology/src/error.rs` | `MeteorologyError::fmt` | 35 | 7 | 0 | 56 | `CQR-LOW-L08` |
| `crates/openwepp-sim-contract/src/symbols.rs` | `SymbolAliasRegistryError::fmt` | 67 | 9 | 0 | 90 | `CQR-LOW-L11` |

## Actionable Rows

None.

## Invalid Or Stale Adjudications

None.
