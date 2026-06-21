# R6F Pre-Implementation Contract Gate

Status: complete-for-R6F-hold.

## Contract Authority Ledger

| Output family | Field/operand | Proposed change | Contract/authority | Authority status | Decision |
|---|---|---|---|---|---|
| HBP | near-zero `peakro`, `watdur` | Emit direct near-zero operands from typed direct runoff instead of absent fallback. | R6 architecture ledger; WB16 near-zero constants already in code. | Sufficient for inherited R6E blocker; full R6 HBP still needs nonzero peak-runoff/event-duration fixture coverage. | Implemented. |
| WAT | climate `P` | Stop multiplying parsed daily `prcp` by 1000 in runner climate projection because parser value is already mm. | Climate parser semantics and WAT parity evidence. | Sufficient for unit correction. | Implemented. |
| WAT | `Es`, storage/profile fields | Add direct-runtime receiving structure and carry state, not production formulas. | R6 architecture ledger; `SC-EVAP-001`; `SC-SYSTEM-001`. | Sufficient for structural input/carry work, insufficient for production parsed-input producer closure. | Structural work implemented; producer deferred to R6G. |

## Contract Amendments Needed

| Gap | Required authority | Action | Status |
|---|---|---|---|
| Production parsed-input direct ET/storage/profile producer binding | Confirm or amend `SC-EVAP-001` and `SC-SYSTEM-001` for runner-side parsed-input to `DirectPublicationDayInput` mapping, including PMET/Priestley-Taylor branch operands and profile/storage lineage. | R6G. | Open. |

## Decision

R6F did not implement provisional process-physics formulas. It added the typed
direct receiving surface and stopped at a hold where the remaining work needs a
contract-authoritative production producer.
