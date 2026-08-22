# Physical scenario matrix

| Scenario | Required result | Status |
| --- | --- | --- |
| snow persists for full support | covered executor advances | `HOLD / integration test ignored; released Stage-3 shortwave/soil boundary custody is incomplete` |
| sublimation/deposition | signed vapor ledger is valid | `NOT RUN` |
| rain on snow/refreeze/melt | phase and energy custody valid | `NOT RUN` |
| meltout at start/end/interior | chronology and remainder valid | `NOT RUN` |
| coalesced event | one accepted event and parcel | `NOT RUN` |
| positive physiology under snow | real V11 calls execute | `NOT RUN` |
| litter/infiltration/ponding/overflow | real receivers consume liquid | `NOT RUN` |
| multi-tile/multi-lane/cross-midnight | identities remain separated | `NOT RUN` |
| snow disappearance/reappearance | owner chronology remains valid | `NOT RUN` |
| restart before/at/after event | equivalent and nonreplaying | `NOT RUN` |

`Static:` Current evidence proves the typed owner-derived carrier,
destination-receipt projection, and rollback plumbing through unit/regression
coverage. It does not prove a passing 1,800-second physical covered support;
the integration case is intentionally held at the missing Stage-3 shortwave/
soil boundary custody and therefore does not pass the physical scenario
matrix or the distinct lower-surface operator gate.
