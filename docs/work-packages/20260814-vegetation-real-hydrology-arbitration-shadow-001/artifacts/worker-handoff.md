# Worker Handoff

Status: `Child-2 implementation stable / terminal verification pending`

Child 3 may consume the production-owned snapshot, request authorization and
finalized-debit seams only after Child 1's land-surface authority evidence is
truthfully released. The current public bridge is deliberately default-off and
single-OFE.

Retain these boundaries:

- V7 root requests and any later ground requests must share one immutable
  production hydrology snapshot and one authorization pass.
- Only finalized use may debit the staged production-owned candidate.
- Production legacy ET and dispatch remain unchanged.
- Partial frost and routed multi-OFE execution are typed unsupported here.
- Child 4 owns routed scheduler integration, the exhaustive phase-injection
  matrix and real-consumer proof.
- Split `vegetation_real_hydrology_shadow.rs` into snapshot, arbitration,
  candidate and bridge modules before extending it in Child 4.
