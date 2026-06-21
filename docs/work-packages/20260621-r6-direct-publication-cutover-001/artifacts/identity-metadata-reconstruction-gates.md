# Identity Metadata Reconstruction Gates

Status: blocked.
Evidence mode: Static.

## Required Evidence Classes

| Gate | Requirement |
|---|---|
| Byte identity | HBP and byte-stable JSON outputs match accepted baseline bytes. |
| Arrow identity | WAT/PASS row values, schema, field metadata, dataset metadata, and producer metadata match. |
| Metadata parity | Calendar, row identity, schema IDs, units/descriptions, output policy, warnings, checksums, and execution provenance match. |
| Anti-alias fixtures | Fixtures fail if a wrong alias supplies an accepted output field. |
| Independent reconstruction | Rebuild conservation-sensitive output operands without calling the production direct projection builder under test. |

## Gate

NOT RUN. Identity, metadata, anti-alias, and reconstruction gates remain queued
because R6 stopped before output-family cutover. The blocker is the missing
run-bound direct publication frame, not the previous R5E prerequisite.
