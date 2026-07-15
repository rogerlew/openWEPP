# Adjudicated CRAP Gate Report

Status: `PASS`

- Acquisition mode: `fresh`.
- Eligible for current-source closure: `True`.
- Debt assessment: `PASS`.
- Threshold: CRAP strictly greater than `30` is raw debt.
- Production entries assessed: `8768`.
- Raw rows over threshold: `2`.
- Currently adjudicated rows: `2`.
- Actionable rows: `0`.
- Touched production files: `14`.
- Actionable rows in touched files: `0`.
- Actionable rows outside touched files: `0`.
- Source HEAD: `00d985b1c0de77f1ea664df23a6f4999c4dad0cc`.
- Touched-file base: `00d985b1c0de77f1ea664df23a6f4999c4dad0cc`.
- Touched-file head: `WORKTREE`.
- Worktree dirty: `True`.
- CRAP JSON SHA-256: `ebfc0cbe5f8bbb711e0b80220667e6f7593205ba8ccffc1081932412d2501146`.
- LCOV SHA-256: `d687b96473afe5f4e759f087acad1a09ea9b4a3b29154614a2a73cb402955026`.
- Production source manifest SHA-256: `e5906851a8a962f4f5e89648fc592fee1602602b4950ac4c1160821abf3bfbfc`.
- Adjudication registry SHA-256: `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.

## Touched Production Files

| Status | Path |
| --- | --- |
| `U` | `crates/openwepp-assurance/src/authoring.rs` |
| `U` | `crates/openwepp-assurance/src/cli.rs` |
| `U` | `crates/openwepp-assurance/src/engine.rs` |
| `U` | `crates/openwepp-assurance/src/error.rs` |
| `U` | `crates/openwepp-assurance/src/graph.rs` |
| `U` | `crates/openwepp-assurance/src/hash.rs` |
| `U` | `crates/openwepp-assurance/src/lib.rs` |
| `U` | `crates/openwepp-assurance/src/main.rs` |
| `U` | `crates/openwepp-assurance/src/model.rs` |
| `U` | `crates/openwepp-assurance/src/path.rs` |
| `U` | `crates/openwepp-assurance/src/publication.rs` |
| `U` | `crates/openwepp-assurance/src/render.rs` |
| `U` | `crates/openwepp-assurance/src/review.rs` |
| `U` | `crates/openwepp-assurance/src/snapshot.rs` |

## Adjudicated Rows

| File | Function | Line | CC | Coverage | CRAP | Adjudication |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `crates/openwepp-meteorology/src/error.rs` | `MeteorologyError::fmt` | 35 | 7 | 0 | 56 | `CQR-LOW-L08` |
| `crates/openwepp-sim-contract/src/symbols.rs` | `SymbolAliasRegistryError::fmt` | 67 | 9 | 0 | 90 | `CQR-LOW-L11` |

## Actionable Rows

None.

## Invalid Or Stale Adjudications

None.
