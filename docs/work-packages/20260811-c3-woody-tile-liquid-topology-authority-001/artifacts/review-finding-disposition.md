# Review Finding Disposition

Status: `all findings accepted and independently remediated`

Evidence mode: `Static + Ran`

No finding is rejected or deferred. Overlapping A/B findings share one
correction but retain their original identities and historical review evidence.

| Finding | Disposition | Exact correction | Focused evidence | Status |
|---|---|---|---|---|
| `A-CRITICAL-001` | accepted | `SC-VEGETATION-001@6` now binds potential top-to-bottom column solve, one immutable hydrology arbitration, and final top-to-bottom rebuild from original beginning state; descendants consume final upstream release and any failure is atomic. The oracle includes an upper-cap change that alters lower accepted state. | authority suite `14/14`; oracle all 31 checks true | remediated; final rereview PASS |
| `A-CRITICAL-002` | accepted | Added `SC-VEGETATIONTRANSACTION-001@1`; it binds exact occupancy/resource/area identities, receiving-owner duplicate/swap rejection, hydrology arbitration, energy reconstruction, and all-owner atomicity. `SC-VEGETATION-001` consumes it through `REF-VEGETATION-030`. | both contract unit lints pass; final transaction-contract digest `c94d3c5745fd801b092f992b46fb6f5d4684b70acf24f198c4d4d6fdc42785c8` | remediated; final rereview PASS |
| `A-HIGH-003` | accepted | Replaced the aggregate warm-start label with a field-level typed occupancy schema, units, domains, root-layer cardinality/order, null/prior transaction semantics, canonical serialization, state-digest coverage, and fail-closed migration requirements. | final V2 digest `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3` and schema authority-tested | remediated; final rereview PASS |
| `A-HIGH-004` | accepted | Expanded the independent Python oracle and committed fixture to 31 positive/poison checks, including reductions, permutation, rollback, area/routing, authorization, lane, exact V1 wet-energy/FvCB locality, C/N, and species/layer identities. Controlled vapor proves routing causality only; complete capped coupling is explicitly a Stage-B exact-oracle gate. | regenerated fixture SHA-256 `c02e5e2a2287d84cfc584a6e3ec9c499cf7168160bc71f2577323f19dcb50bf1`; authority suite `14/14` | remediated; final rereview PASS |
| `A-HIGH-005` | accepted | The contracts remained draft during remediation and were promoted only after final science rereviews; both are now consistently `approved/active`. | admission PASS `contracts=45`, receipt `464b2675f17f75a6a9e92c6de0a70dae76ef03ca092c23f29d2ad965d62be628` | remediated; promotion gate PASS |
| `B-CRITICAL-001` | accepted | Same correction as `A-CRITICAL-002`; the shared transaction contract is canonical cross-owner authority rather than a vegetation-producer assertion. | final transaction-contract digest `c94d3c5745fd801b092f992b46fb6f5d4684b70acf24f198c4d4d6fdc42785c8` bound in V2 | remediated; final rereview PASS |
| `B-CRITICAL-002` | accepted | Same expanded oracle/fixture correction as `A-HIGH-004`; Stage-A rollback and poison vectors are no longer deferred to Stage B. | oracle all 31 checks true; committed fixture digest bound | remediated; final rereview PASS |
| `B-HIGH-003` | accepted | Same exact schema correction as `A-HIGH-003`; all V1-to-V2 migrations require caller-supplied complete numerical lanes with null transaction identity. | canonical JSON schema and migration tests | remediated; final rereview PASS |
| `B-HIGH-004` | accepted | Added exact shared-stratum aggregation formulas and ordering. Shared maintenance/turnover/allocation/growth respiration execute once; mineral N remains keyed by stratum/layer/species after occupancy aggregation. | oracle asymmetric shared-C/N and mineral-N identity poisons pass | remediated; final rereview PASS |
| `B-HIGH-005` | accepted | Rebuilt V2 as single-line lexicographically sorted canonical JSON with an exact V1 base-definition digest and normative merge semantics; both copies are byte-identical. | final V2 digest `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3` | remediated; final rereview PASS |

The historical HOLD reviews remain unchanged. The final resource/transaction
rereview and final topology/energy rereview both returned PASS with no
unresolved material finding.

## Repeat-Review Findings

All repeat-review findings are accepted; none is deferred or rejected.

| Finding | Disposition and correction | Status |
|---|---|---|
| `A-RR-CRITICAL-001` / `B-REREVIEW-CRITICAL-001` | Replaced superficial booleans with explicit case records: the same lower stratum now occurs beneath distinct upper columns; a keyed potential-request/proportional-arbitration/fixed-cap final rebuild changes the upper coupled response and descendant incident; rollback serializes and isolates vegetation warm starts/shared C/N, water, BGC, energy, and transaction state; mineral N executes layer/species proportional arbitration and typed swap rejection; routing alternatives execute identity rejection; wetness and PAR poisons use E04 and digest-bound FvCB responses. Rust independently reconstructs local/stand closure, resource bounds, exact keys, rollback digest, state digest, nonlinear differences, and shared-transition poison. | remediated; final rereviews PASS |
| `A-RR-HIGH-002` | Selected `mm H2O` consistently with E14/E15, made MPa a typed wrong-unit failure, selected recursive lexical state-key serialization, added an exact serialized state/digest fixture, and removed displayed-order authority. | remediated; final rereview PASS |
| `A-RR-HIGH-003` | Decomposed the Rust fixture test into focused reconstruction helpers without lint suppression. | corrected; focused Clippy PASS |
| `A-RR2-CRITICAL-001` | Deleted the uncited cap-to-vapor response and simplified nonlinear helpers. The topology case now labels both vapor values controlled exogenous operands and makes no coupled-science claim; full capped E11--E15 conformance is a named Stage-B exact-oracle gate. Wet-energy and PAR locality use the existing digest-bound V1 authority oracle, including its admitted FvCB co-limitation and wet-surface energy solve. | remediated; topology rereview 3 PASS |
