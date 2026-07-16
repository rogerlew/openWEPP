# Adjudicated CRAP Gate Report

Status: `FAIL`

- Acquisition mode: `fresh`.
- Eligible for current-source closure: `True`.
- Debt assessment: `FAIL`.
- Threshold: CRAP strictly greater than `30` is raw debt.
- Production entries assessed: `8828`.
- Raw rows over threshold: `5`.
- Currently adjudicated rows: `2`.
- Actionable rows: `3`.
- Touched production files: `5`.
- Actionable rows in touched files: `3`.
- Actionable rows outside touched files: `0`.
- Source HEAD: `e704f0202278ebb86c6a8c667caf73d599be04ab`.
- Touched-file base: `e704f0202278ebb86c6a8c667caf73d599be04ab`.
- Touched-file head: `WORKTREE`.
- Worktree dirty: `True`.
- CRAP JSON SHA-256: `08e338cf3b5765684d5878515b34f8272690f1aeece1bea6d1b40958dab59652`.
- LCOV SHA-256: `f88967017d97e9a319bfec625d201a965cc7507beea42b895ad3a0d7bf4bb5f4`.
- Production source manifest SHA-256: `a8568a55624aeffc65feaa449d9d8eee1dd027ce6de882ecb6e4ec55bdf67280`.
- Adjudication registry SHA-256: `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.

## Touched Production Files

| Status | Path |
| --- | --- |
| `M` | `crates/openwepp-assurance/src/cli.rs` |
| `M` | `crates/openwepp-assurance/src/lib.rs` |
| `M` | `crates/openwepp-assurance/src/v2.rs` |
| `U` | `crates/openwepp-assurance/src/v2/assembly.rs` |
| `M` | `crates/openwepp-assurance/src/v2/planner.rs` |

## Adjudicated Rows

| File | Function | Line | CC | Coverage | CRAP | Adjudication |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `crates/openwepp-meteorology/src/error.rs` | `MeteorologyError::fmt` | 35 | 7 | 0 | 56 | `CQR-LOW-L08` |
| `crates/openwepp-sim-contract/src/symbols.rs` | `SymbolAliasRegistryError::fmt` | 67 | 9 | 0 | 90 | `CQR-LOW-L11` |

## Actionable Rows

| File | Function | Line | CC | Coverage | CRAP | Adjudication |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `crates/openwepp-assurance/src/v2.rs` | `validate_table` | 1842 | 17 | 62.8571 | 31.8089 | `` |
| `crates/openwepp-assurance/src/v2/assembly.rs` | `resolve_bindings` | 226 | 23 | 66.6667 | 42.5926 | `` |
| `crates/openwepp-assurance/src/v2/assembly.rs` | `remove_existing_directory` | 1294 | 8 | 21.0526 | 39.4915 | `` |

## Invalid Or Stale Adjudications

None.
