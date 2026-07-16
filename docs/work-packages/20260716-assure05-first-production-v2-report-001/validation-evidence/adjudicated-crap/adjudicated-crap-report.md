# Adjudicated CRAP Gate Report

Status: `PASS`

- Acquisition mode: `fresh`.
- Eligible for current-source closure: `True`.
- Debt assessment: `PASS`.
- Threshold: CRAP strictly greater than `30` is raw debt.
- Production entries assessed: `9262`.
- Raw rows over threshold: `2`.
- Currently adjudicated rows: `2`.
- Actionable rows: `0`.
- Touched production files: `0`.
- Actionable rows in touched files: `0`.
- Actionable rows outside touched files: `0`.
- Source HEAD: `01ed70550a4e371e99afe35c4bdd4d9b667e812c`.
- Touched-file base: `01ed70550a4e371e99afe35c4bdd4d9b667e812c`.
- Touched-file head: `WORKTREE`.
- Worktree dirty: `True`.
- CRAP JSON SHA-256: `413bc7035a416db70298bb341f1330891b43a82d5b10bccd0aa479369740ff3e`.
- LCOV SHA-256: `d116cab75263f0163e64a42b8506096569cc31fdda6ac3fe6f0fc2977816d399`.
- Production source manifest SHA-256: `5f0446b67c84ecc1606a8adc6527adf75734ab82bda0df7ee62265635f593fcd`.
- Adjudication registry SHA-256: `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.

## Adjudicated Rows

| File | Function | Line | CC | Coverage | CRAP | Adjudication |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `crates/openwepp-meteorology/src/error.rs` | `MeteorologyError::fmt` | 35 | 7 | 0 | 56 | `CQR-LOW-L08` |
| `crates/openwepp-sim-contract/src/symbols.rs` | `SymbolAliasRegistryError::fmt` | 67 | 9 | 0 | 90 | `CQR-LOW-L11` |

## Actionable Rows

None.

## Invalid Or Stale Adjudications

None.
