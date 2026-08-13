# Test Vector Ledger

Ran: `reference_calculator_v7.py --verify` reproduces
`openwepp_c3_woody_v7_vectors.json` byte-for-byte.

| Family | Independent evidence |
|---|---|
| Six-tissue preparation | Distinct C/N display/storage/transfer values and exact `0.5*S0` operands |
| First onset interval | Existing plus prepared transfer, fraction `0.5` |
| Multi-interval onset | Fractions `0.5, 2/3, 1`; one preparation; exact exhaustion |
| Event branches | Crossing, equality, no crossing, already-onset, active |
| Allocation exclusion | Post-onset E19 allocation remains in ending storage |
| Evergreen | Signed-zero acceptance and non-one/nonzero rejection |
| Migration | Non-identity preservation, no event, invalid-source and evergreen rejection |
| Poisons | Executed discriminating alternatives return typed failures |
| Rollback | Full owner snapshots hash identically before/after each injection |

