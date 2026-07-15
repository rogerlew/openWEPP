# Adjudicated CRAP Gate Report

Status: `PASS`

- Acquisition mode: `fresh`.
- Eligible for current-source closure: `True`.
- Debt assessment: `PASS`.
- Threshold: CRAP strictly greater than `30` is raw debt.
- Production entries assessed: `8422`.
- Raw rows over threshold: `2`.
- Currently adjudicated rows: `2`.
- Actionable rows: `0`.
- Touched production files: `13`.
- Actionable rows in touched files: `0`.
- Actionable rows outside touched files: `0`.
- Source HEAD: `3352388465f8b288aed4636e8f9752ca6c1cceb9`.
- Touched-file base: `3352388465f8b288aed4636e8f9752ca6c1cceb9`.
- Touched-file head: `WORKTREE`.
- Worktree dirty: `True`.
- CRAP JSON SHA-256: `f093a86c129415309fefd99d41d25998a372ac620df185833a80174b29da3fe5`.
- LCOV SHA-256: `4a4dad862b50d3de3bfa6dd748ff5818696ced7898b23f4cc71f1cb8aa6b18a1`.
- Production source manifest SHA-256: `3a28ecde0c65f38b55b10cb58b5e0967ac82a88a013b0ee082cba08b4280a0e2`.
- Adjudication registry SHA-256: `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.

## Touched Production Files

| Status | Path |
| --- | --- |
| `D` | `crates/openwepp-assurance/src/authoring.rs` |
| `M` | `crates/openwepp-assurance/src/cli.rs` |
| `M` | `crates/openwepp-assurance/src/engine.rs` |
| `M` | `crates/openwepp-assurance/src/error.rs` |
| `D` | `crates/openwepp-assurance/src/graph.rs` |
| `M` | `crates/openwepp-assurance/src/hash.rs` |
| `M` | `crates/openwepp-assurance/src/lib.rs` |
| `D` | `crates/openwepp-assurance/src/model.rs` |
| `D` | `crates/openwepp-assurance/src/path.rs` |
| `D` | `crates/openwepp-assurance/src/publication.rs` |
| `D` | `crates/openwepp-assurance/src/render.rs` |
| `D` | `crates/openwepp-assurance/src/review.rs` |
| `D` | `crates/openwepp-assurance/src/snapshot.rs` |

## Adjudicated Rows

| File | Function | Line | CC | Coverage | CRAP | Adjudication |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `crates/openwepp-meteorology/src/error.rs` | `MeteorologyError::fmt` | 35 | 7 | 0 | 56 | `CQR-LOW-L08` |
| `crates/openwepp-sim-contract/src/symbols.rs` | `SymbolAliasRegistryError::fmt` | 67 | 9 | 0 | 90 | `CQR-LOW-L11` |

## Actionable Rows

None.

## Invalid Or Stale Adjudications

None.
