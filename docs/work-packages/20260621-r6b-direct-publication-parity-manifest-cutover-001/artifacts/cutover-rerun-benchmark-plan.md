# Cutover Rerun and Benchmark Plan

Status: executed-hold.
Evidence mode: Static + Ran.

## Handoff Item 5

Re-run the cutover candidate until HBP, WAT, PASS, loss, and manifest gates
pass, then run default-disabled and endpoint/RSS benchmarks.

## Required Runs

- `DirectPublicationFrameCutover` fixture with HBP byte identity.
- WAT Arrow row/schema/metadata parity.
- PASS Arrow row/schema/metadata parity with a PASS parquet configured fixture.
- Loss JSON byte-normalized parity.
- Manifest schema/checksum/provenance/counter parity.
- H2637 default-disabled timing and protected-output identity/equivalence.
- Direct-publication endpoint/RSS benchmark against accepted R5E/R6 baseline.

## Gate

BLOCKED. R6B reran the cutover candidate and confirmed the first output-family
gate still fails before public writes. Default-disabled and endpoint/RSS
benchmarks were not run because no valid direct-publication endpoint exists
while item 1 is failed.
