# Output Family Gate Matrix

Status: executed-hold.
Evidence mode: Static.

| Output family | Required gates | Status |
|---|---|---|
| HBP | Typed projection only; byte identity; HBP metadata/provenance parity; anti-alias fixtures for peak runoff, event duration, detachment/deposition, and sediment concentration; independent reconstruction where sediment/water-sensitive. | BLOCKED: no run-bound direct publication frame supplies HBP operands. |
| WAT | Typed projection only; Arrow row/schema/field metadata identity; calendar/identity parity; water-balance operand anti-alias fixtures; independent water-balance reconstruction. | BLOCKED: WAT rows still build from compatibility WB13 rows. |
| PASS | Typed projection only; Arrow row/schema/field metadata identity; runoff/lateral/peak/sediment volume basis parity; outlet-area anti-alias fixtures; independent volume reconstruction. | BLOCKED: PASS rows still derive from compatibility WB13/outlet rows. |
| loss JSON | Typed projection only; byte-normalized JSON identity; loss-source anti-alias fixtures; independent reconstruction for reported conservation-sensitive totals. | BLOCKED: no direct loss projection exists. |
| run manifest | Typed projection only; schema ID, checksums, direct-runtime counters, execution provenance, warning metadata, output policy, and publication ledger provenance parity. | BLOCKED: no direct manifest projection exists. |

## Gate

BLOCKED. The blocker is package-level and prevents completion after ledger
promotion.
