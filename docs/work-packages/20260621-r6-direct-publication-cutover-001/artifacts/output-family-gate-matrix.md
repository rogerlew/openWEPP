# Output Family Gate Matrix

Status: executed-hold.
Evidence mode: Static + Ran.

| Output family | Required gates | Status |
|---|---|---|
| HBP | Typed projection only; byte identity; HBP metadata/provenance parity; anti-alias fixtures for peak runoff, event duration, detachment/deposition, and sediment concentration; independent reconstruction where sediment/water-sensitive. | FAIL: cutover candidate reaches HBP parity and fails closed with equal-length but byte-different output (`1654` vs `1654`). |
| WAT | Typed projection only; Arrow row/schema/field metadata identity; calendar/identity parity; water-balance operand anti-alias fixtures; independent water-balance reconstruction. | BLOCKED: direct WAT rows exist, but HBP parity stops candidate before WAT acceptance; full Arrow/metadata parity not run. |
| PASS | Typed projection only; Arrow row/schema/field metadata identity; runoff/lateral/peak/sediment volume basis parity; outlet-area anti-alias fixtures; independent volume reconstruction. | BLOCKED: direct PASS rows exist, but current CLI fixture has no PASS parquet output and HBP parity stops candidate before PASS acceptance. |
| loss JSON | Typed projection only; byte-normalized JSON identity; loss-source anti-alias fixtures; independent reconstruction for reported conservation-sensitive totals. | BLOCKED: direct loss helper now emits `openwepp-hillslope-loss-v1` JSON shape, but loss parity is behind failed HBP identity. |
| run manifest | Typed projection only; schema ID, checksums, direct-runtime counters, execution provenance, warning metadata, output policy, and publication ledger provenance parity. | BLOCKED: production manifest writer remains compatibility-provenance based; cutover candidate returns a blocker if earlier families ever pass. |

## Gate

BLOCKED. R6 cannot close while any output family has `FAIL`, `BLOCKED`, or
unrun current-scope acceptance gates.
