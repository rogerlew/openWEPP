# Publication Cutover Plan

Status: executed-hold.
Evidence mode: Static + Ran.

## Ordered Cutover

| Order | Output family | Cutover target | Required identity gate | Current status |
|---:|---|---|---|---|
| 1 | HBP | Typed direct projection only | Byte identity. | BLOCKED: candidate fails HBP byte identity before write. |
| 2 | WAT | Typed direct projection only | Arrow row/schema/metadata identity; byte identity where stable. | BLOCKED behind HBP parity; candidate direct rows exist but are not accepted. |
| 3 | PASS | Typed direct projection only | Arrow row/schema/metadata identity; byte identity where stable. | BLOCKED behind HBP parity; fixture run lacks PASS parquet output coverage. |
| 4 | loss JSON | Typed direct projection only | Byte-normalized JSON identity or reviewed key-order-only normalization. | BLOCKED behind HBP parity; helper now emits schema-shaped JSON. |
| 5 | run manifest | Typed direct projection only | Schema ID, checksum, provenance, counter, and metadata parity. | BLOCKED: production manifest writer still uses compatibility provenance surfaces. |

## Implemented Candidate

Ran:

- Added `HillslopeRuntimeSelection::DirectPublicationFrameCutover`.
- Added CLI flag `--direct-publication-frame-cutover`.
- Shared direct publication artifact construction between shadow and cutover
  candidate modes.
- Routed the candidate output boundary through direct HBP, WAT, PASS, and loss
  artifacts after fail-closed parity checks.
- Kept output writes blocked until all current gates pass.

## Gate

BLOCKED. The current direct publication frame is available, but its real-run
operands are not parity-grade. The first candidate gate fails HBP byte identity,
and manifest publication cutover is still not wired.
