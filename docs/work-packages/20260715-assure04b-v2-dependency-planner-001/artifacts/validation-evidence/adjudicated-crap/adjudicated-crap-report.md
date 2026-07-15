# Adjudicated CRAP Gate Report

Status: `PASS`

- Acquisition mode: `fresh`.
- Eligible for current-source closure: `True`.
- Debt assessment: `PASS`.
- Threshold: CRAP strictly greater than `30` is raw debt.
- Production entries assessed: `8694`.
- Raw rows over threshold: `2`.
- Currently adjudicated rows: `2`.
- Actionable rows: `0`.
- Touched production files: `6`.
- Actionable rows in touched files: `0`.
- Actionable rows outside touched files: `0`.
- Source HEAD: `22fb7dfbafdb9e82a42afe0a5356b4c923a45232`.
- Touched-file base: `22fb7dfbafdb9e82a42afe0a5356b4c923a45232`.
- Touched-file head: `WORKTREE`.
- Worktree dirty: `True`.
- CRAP JSON SHA-256: `4bf27cfb60a385c2c8c65ff9b136fc6a028d12014ff6004deace2069187c2056`.
- LCOV SHA-256: `1fb6979da78043ba4e925e11eba8ce4cc3432ada08c85da1b27a0ce6f4a1b9a1`.
- Production source manifest SHA-256: `0f5f3193c6886135a72b1a8cb670743de99e67c4cfac79ffcc937299079d45cc`.
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

None.

## Invalid Or Stale Adjudications

None.
