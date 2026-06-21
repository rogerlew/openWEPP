# Identity Metadata Reconstruction Gates

Status: executed-hold.
Evidence mode: Static + Ran.

## Required Evidence Classes

| Gate | Requirement | Current result |
|---|---|---|
| Byte identity | HBP and byte-stable JSON outputs match accepted baseline bytes. | FAIL: HBP direct and compatibility outputs are both `1654` bytes but byte-different. |
| Arrow identity | WAT/PASS row values, schema, field metadata, dataset metadata, and producer metadata match. | NOT RUN: HBP fail-closed gate stops candidate first. |
| Metadata parity | Calendar, row identity, schema IDs, units/descriptions, output policy, warnings, checksums, and execution provenance match. | BLOCKED: manifest writer still uses compatibility provenance. |
| Anti-alias fixtures | Fixtures fail if a wrong alias supplies an accepted output field. | NOT RUN for cutover acceptance; R6A has frame-consumer anti-alias evidence only. |
| Independent reconstruction | Rebuild conservation-sensitive output operands without calling the production direct projection builder under test. | NOT RUN for cutover acceptance; current direct operands are not parity-grade. |

## Ran Evidence

```text
cargo test -p openwepp-runner r6_ -- --nocapture
```

Result: PASS. The focused test proves the cutover candidate fails closed with
`R6-DIRECT-PUBLICATION-PARITY`.

```text
cargo run -p openwepp-runner --bin openwepp-cli-hill -- \
  --run-dir /tmp/r6cutover.wv66Ba \
  --run-file case.run \
  --output-dir /tmp/r6cutover.wv66Ba/output \
  --direct-publication-frame-cutover
```

Result: exit status `1` with
`HBP byte identity failed: direct=1654 bytes compatibility=1654 bytes`.

## Gate

BLOCKED. Identity, metadata, anti-alias, and reconstruction gates are
current-scope R6 acceptance gates and cannot be deferred into a completion
claim.
