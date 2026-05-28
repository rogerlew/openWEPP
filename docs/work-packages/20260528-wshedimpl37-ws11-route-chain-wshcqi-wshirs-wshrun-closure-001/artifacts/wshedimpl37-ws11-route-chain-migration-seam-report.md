# WSHEDIMPL37 WS11 Route-Chain Migration Seam Report

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Scope
- Baseline-authoritative WS11 route-chain migration closure for
  `wshcqi/wshirs/wshrun` runtime behavior:
  - runon partition assembly and publication,
  - duration max-rule continuity publication,
  - runoff/transmission-loss case publication and branch continuity,
  - `ipeak` lane threshold and wave-routing continuity.

## Static
- Runtime seam surfaces added in WS10 channel outputs:
  - `ws10_channel_{id}_{rvolat,rvotop,rvolon}`
  - `ws10_channel_{id}_{durlat,durtop,durrunon,durchan,durirrig,watdur}`
  - `ws10_channel_{id}_{rofc,tl,ws11_runoff_case,ws11_qci,ws11_qcf,ws11_runvol}`
- Contract traceability updated with `GAP-ROUTE-008` disposition to `closed`.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl37_` -> pass
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` -> pass
