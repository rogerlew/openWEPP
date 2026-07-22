# CRAP After

Ran: same-LCOV CRAP at corrected `d5af6207` reports zero production rows above
30 and a maximum of 5.024. Both originals and every extracted helper close.

| Original | Before CRAP | After CC / coverage / CRAP |
| --- | ---: | ---: |
| `mirror_node_checkpoint` | 305.5363 | 4 / 100% / 4 |
| `create_absolute_directories` | 132 | 5 / 100% / 5 |

| Artifact | SHA-256 |
| --- | --- |
| LCOV | `57c26bb7dba9e48bf37c5404e72d50bfb0ca9e6fc61cf6b96d9dd14144c5cb10` |
| CRAP JSON | `069a6c57ecb008ed615c97c8170be05920a1f065ecd75e7333da56bc6e7dabfb` |
| coverage JSON | `4da61b0007d43c70af8fcc1141d41dec00de2015878a176540f32ae91dc0e658` |
| run log | `6535b2efc29a78bc02d43d612f7252e337c04ba1bdc2d0732db8058d8ba9c48f` |
| raw function TSV | `1520e1d34befbc782595bcee4d40b500a847ea0bf2b2af9e9f70d07bee381550` |
| normalized package TSV | `d6a6e809a159e418caaee78dacd65716649cdb1fbe003645f7528eccab5553e3` |

The earlier `424a1a5c` measurement passed numeric thresholds but is
`INVALIDATED-REVIEW`: both reviewers found semantic drift. It was not reused as
closure evidence. The corrected-head traversal is authoritative.
