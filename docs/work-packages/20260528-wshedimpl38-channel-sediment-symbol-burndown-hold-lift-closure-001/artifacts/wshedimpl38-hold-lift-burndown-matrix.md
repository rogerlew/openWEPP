# WSHEDIMPL38 HOLD-Lift Burndown Matrix

Status: queued  
Evidence mode: static  
Date: 2026-05-28

## Static

| Blocker / Gap | Current condition | Package owner | Closure signal |
|---|---|---|---|
| `GAP-ROUTE-008` | WS11 hydrology routine-chain parity (`wshcqi/wshirs/wshrun`) not fully migrated | WSHEDIMPL37 | Baseline-authoritative route-chain behavior active in production lanes with passing contract vectors |
| `GAP-ROUTE-009` | Residual channel sediment parity scope remains open after prior WS20–WS35 waves | WSHEDIMPL38 | Residual `chnero/chnrt/detach` families closed or explicitly narrowed with evidence-backed ownership |
| `GAP-SED-006` | Companion watershed channel sediment closure remains non-promotable | WSHEDIMPL38 | Cross-contract sediment parity evidence supports updated promotability posture |
| `GAP-SYSTEM-008` | System-level watershed sediment integration hold remains active | WSHEDIMPL38 | End-to-end validation/comparator evidence supports GO/HOLD disposition update |

| Symbol burndown theme | Open surface | Package owner | Required closure evidence |
|---|---|---|---|
| Parser/runtime control projection | `ws10_channel_{id}_{rccoef,rcexp,rcoset}` under `icntrl==4` | WSHEDIMPL36 | Parser + runtime fail-closed vectors for payload-presence/domain continuity |
| Hydrology branch lineage | WS11 route-chain state transitions and branch semantics | WSHEDIMPL37 | Contract-derived branch vectors + parity assertions on routed publications |
| Residual sediment diagnostics | Unresolved fallback counters/symbols used during staged migration | WSHEDIMPL38 | Counters removed/retired or proven zero under finalized parity lanes with contract updates |

## Ran
- not run
