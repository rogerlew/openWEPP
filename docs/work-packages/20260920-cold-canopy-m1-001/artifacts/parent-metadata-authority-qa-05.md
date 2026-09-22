**Static QA PASS — authority encoding/custody correction.** No blocking schema or receipt-cycle defect remains.

The static payload now excludes derived GSI receipts; `run_identity` binds its full payload hash, the GSI receipt is minted only after run derivation, and the forcing receipt binds the ordered 30 derived GSI-receipt identities for its parent slice. This is acyclic and rejects missing, stale, foreign, or reordered receipts before staging.

The full projection digest is explicitly defined and binds source joins. The retained `1.0` endpoint operand is selected through a hash-bound constructor/output adapter, without reusing transaction-41 receipt/support identity or claiming climate-derived GSI provenance. `VEG-E-144`, BEI, contract version, and change log align with the new provider admission boundary.

Non-blocking follow-up: the contract-derived control cut should include dedicated missing/foreign/reordered GSI-support-receipt cases, alongside the already required payload and source-join mutations.

No Rust or physical work was run or approved.
