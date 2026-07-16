# Adjudicated CRAP Gate Report

Status: `FAIL`

- Acquisition mode: `fresh`.
- Eligible for current-source closure: `True`.
- Debt assessment: `FAIL`.
- Threshold: CRAP strictly greater than `30` is raw debt.
- Production entries assessed: `9216`.
- Raw rows over threshold: `9`.
- Currently adjudicated rows: `2`.
- Actionable rows: `7`.
- Touched production files: `7`.
- Actionable rows in touched files: `7`.
- Actionable rows outside touched files: `0`.
- Source HEAD: `ec396c458a5015c504011a75814ff13e274544a1`.
- Touched-file base: `ec396c458a5015c504011a75814ff13e274544a1`.
- Touched-file head: `WORKTREE`.
- Worktree dirty: `True`.
- CRAP JSON SHA-256: `c511a02f1c46d034d0e415cc96130c24ec1c71130ff4606905b2b081bb0328a8`.
- LCOV SHA-256: `03b305372ca9fd9e7e065083f25d4c708b4c3ffcfc3c85a5d4a44edba3dacf74`.
- Production source manifest SHA-256: `a25f5fb5048612447ea50262cecd0520ce1bc548451a9f1358b50b84bc74035d`.
- Adjudication registry SHA-256: `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.

## Touched Production Files

| Status | Path |
| --- | --- |
| `M` | `crates/openwepp-assurance/src/cli.rs` |
| `M` | `crates/openwepp-assurance/src/lib.rs` |
| `M` | `crates/openwepp-assurance/src/v2.rs` |
| `M` | `crates/openwepp-assurance/src/v2/assembly.rs` |
| `M` | `crates/openwepp-assurance/src/v2/confined.rs` |
| `U` | `crates/openwepp-assurance/src/v2/lifecycle.rs` |
| `U` | `crates/openwepp-assurance/src/v2/publication.rs` |

## Adjudicated Rows

| File | Function | Line | CC | Coverage | CRAP | Adjudication |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `crates/openwepp-meteorology/src/error.rs` | `MeteorologyError::fmt` | 35 | 7 | 0 | 56 | `CQR-LOW-L08` |
| `crates/openwepp-sim-contract/src/symbols.rs` | `SymbolAliasRegistryError::fmt` | 67 | 9 | 0 | 90 | `CQR-LOW-L11` |

## Actionable Rows

| File | Function | Line | CC | Coverage | CRAP | Adjudication |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `crates/openwepp-assurance/src/cli.rs` | `execute_publish` | 177 | 17 | 20 | 164.968 | `` |
| `crates/openwepp-assurance/src/cli.rs` | `parse_options` | 222 | 31 | 95.5224 | 31.0863 | `` |
| `crates/openwepp-assurance/src/v2.rs` | `validate_report_structure` | 1525 | 33 | 92.5 | 33.4594 | `` |
| `crates/openwepp-assurance/src/v2.rs` | `validate_review` | 2308 | 26 | 71.0526 | 42.3974 | `` |
| `crates/openwepp-assurance/src/v2/confined.rs` | `open_ambient_platform` | 237 | 14 | 50 | 38.5 | `` |
| `crates/openwepp-assurance/src/v2/publication.rs` | `install_receipt` | 1906 | 19 | 61.7021 | 39.2783 | `` |
| `crates/openwepp-assurance/src/v2/publication.rs` | `verify_snapshot_content` | 2234 | 27 | 69.2308 | 48.2362 | `` |

## Invalid Or Stale Adjudications

None.
