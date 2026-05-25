# SIMIMPL20 Ep/Es/Er Full-Fidelity Migration Risk Register

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
| risk_id | severity | statement | evidence anchor | mitigation / owner |
|---|---|---|---|---|
| `SIMIMPL20-RISK-001` | high | Full ET stage-memory state (`s1`, `s2`, `tu`, `tv`) is not represented as first-class WB17 runtime symbols, preventing strict baseline parity of `Es` transitions. | `evap.for:458-555`; `SC-EVAP-001 GAP-EVAP-005` | Amend `SC-EVAP-001` symbol tables and invariants; add stage-transition tests before kernel edits. |
| `SIMIMPL20-RISK-002` | high | Root-zone layer uptake (`UPi`, `Ui`) and compensation logic in `swu` are absent from current WB17 ET phase, so `Ep` and `Ws` are simplified. | `swu.for:122-191`; `lib.rs:4355-4499` | Add WB17/WB11 companion symbols and contract-derived layer extraction vectors; implement dedicated transpiration extraction phase or equivalent authority-preserving sequencing. |
| `SIMIMPL20-RISK-003` | high | Execution ordering differs from baseline (`swu` after lateral/drain in baseline vs transpiration withdrawal before WB18/WB19 in openWEPP). | `watbal.for:552-921`; scheduler order `lib.rs:10003-10028` | Contract amendment wave must lock ordering authority and branch gates before production edits. |
| `SIMIMPL20-RISK-004` | medium | Scalar `wb11_soil_water` ET updates can hide layer-specific depletion defects even when aggregate closure appears valid. | `lib.rs:4355-4499`; `SC-WATBAL-001 INV-WATBAL-009` | Introduce layer-aware ET validation vectors and publication checks tying aggregate to per-layer lineage. |
| `SIMIMPL20-RISK-005` | medium | Existing WB17 integration tests assert surrogate outputs and may encode non-authoritative expectations for full-fidelity migration. | `tests/integration/wb17_et_physics_kernel_contract.rs`; `tests/integration/wb11_hydrology_kernel_contract.rs` | Replace/add contract-derived vectors keyed to baseline day traces and stage-memory scenarios before ET code migration. |
| `SIMIMPL20-RISK-006` | medium | Companion contract maturity gaps (`GAP-WATBAL-002`, `GAP-EVAP-003`, `GAP-PLANT-004`, `GAP-SOIL-002`, `GAP-SYSTEM-001/002`) can stall promotability even after code changes. | canonical `SC-*` gap registers | Queue starts with contract closure wave; enforce dual review + verification at each contract package. |
| `SIMIMPL20-RISK-007` | medium | WB13 `Ep`/`Es`/`Er` publication may remain semantically inconsistent with migrated runtime lineage if output aliases are not updated with contract changes. | `outfil.for:623-643`; `hillslope_wat.rs:184-233`; `writers.rs:368-410` | Add output-consumer contract tests and alias continuity checks in follow-on replay rerun package. |

## Risk Posture
- No risk above is safely mitigable inside SIMIMPL20 because this package is
  planning-only and forbids production kernel edits.
- Queue in `soil-water-et-baseline-auth-queue.md` is the approved mitigation
  path.

## Ran
- `rg -n "GAP-EVAP-005|GAP-WATBAL-002|GAP-PLANT-004|GAP-SOIL-002|GAP-SYSTEM-00" docs/specifications/science-contracts/contracts/SC-*.md`
- `sed -n '4310,4715p' crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `sed -n '1,320p' tests/integration/wb17_et_physics_kernel_contract.rs`
- `sed -n '1,300p' tests/integration/wb11_hydrology_kernel_contract.rs`
