# R6J Worker Handoff

Evidence class: Static plus Ran.

## Closed

- Closed inherited manifest writer blocker: direct cutover manifests now use
  direct publication provenance, empty replay candidates, direct row
  keys/counts, unique-OFE area, output checksums, and run-local direct runtime
  counters.
- Closed current-fixture HBP/WAT/PASS/loss parity and public direct writes.
- Closed production cutover compatibility-oracle removal: direct writer uses
  `DirectPublicationArtifacts` only.
- Closed review blockers for shadow manifest provenance leakage, global counter
  snapshots, day-multiplied manifest area, missing direct erosion operand
  defaults, disk checksum proof, and Parquet disk readback proof.
- Closed H2637 scale blockers: direct PASS projects the outlet public surface,
  direct HBP reads HBP-specific producer-authoritative erosion operands, and
  PASS Parquet emits stable Arrow schema metadata.
- Preserved default-disabled H2637 gate and passed fresh same-binary H2637
  cutover parity:
  - default: `640.41 s / 227396 KiB`;
  - direct cutover: `637.53 s / 349400 KiB`;
  - HBP/WAT/PASS/loss/plot byte identity;
  - WAT rows `235961` vs `235961`, zero bidirectional DuckDB differences;
  - PASS rows `12419` vs `12419`, zero bidirectional DuckDB differences;
  - direct manifest source `direct-publication-frame`;
  - all direct runtime counters, including `compatibility_edge_invocations`,
    are `0`.

## Remaining R6J Blockers

None.

R6J final disposition is `COMPLETE-R6-DIRECT-PUBLICATION-CUTOVER`.

## Follow-Up Candidates

- Split `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
  before adding more runner publication scope; it is at `2997` lines.
- Split publication-row construction in
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` before more
  direct-runtime publication work; it is at `2922` lines.
- Treat default activation, broader nonzero erosion process authority, and any
  post-R6 performance tuning as separate packages with their own gates.
