# Publication Cutover Plan

Status: blocked.
Evidence mode: Static.

## Ordered Cutover

| Order | Output family | Cutover target | Required identity gate |
|---:|---|---|---|
| 1 | HBP | Typed direct projection only | Byte identity. |
| 2 | WAT | Typed direct projection only | Arrow row/schema/metadata identity; byte identity where stable. |
| 3 | PASS | Typed direct projection only | Arrow row/schema/metadata identity; byte identity where stable. |
| 4 | loss JSON | Typed direct projection only | Byte-normalized JSON identity or reviewed key-order-only normalization. |
| 5 | run manifest | Typed direct projection only | Schema ID, checksum, provenance, counter, and metadata parity. |

## Cross-Family Requirements

- Anti-alias fixtures must exist before each family is accepted.
- Independent reconstruction must exist before each conservation-sensitive
  family is accepted.
- No output family may read compatibility runtime symbols after its cutover.
- Metadata parity is a current-scope gate, not a post-cutover cleanup.

## Gate

BLOCKED after ledger promotion. Publication cutover cannot start until the
runner has a run-bound direct publication frame populated from typed direct
state. The cutover order remains valid once that blocker is closed.
