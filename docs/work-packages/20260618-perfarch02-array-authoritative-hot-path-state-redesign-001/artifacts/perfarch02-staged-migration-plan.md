# PERFARCH02 Staged Migration Plan

Evidence class: Static plan from PERFIDX03, PERFIDX05, PERFIDX06, PERFARCH01,
and the PERFARCH02 floor prototype.

## Decision Gate

Proceed only as a conditional migration program. The next package must be an
integrated WB11 pilot. If that pilot cannot move real H2637 per-OFE-day cost
toward the <=10x budget, stop the program instead of continuing broad rewrites.

## Stage A - Contract Shell

Add the array-authoritative types without flipping production execution:

- `ArrayHotState` or equivalent dense state/flux authority keyed by `SymbolId`;
- `ArrayWritebackField` and `ArrayWritebackPayload` with id, value, and bounds;
- id-backed finite/domain evaluator preserving current message ids;
- logical materialization APIs for tests, output, and diagnostics.

Gates:

- unit tests for finite/domain parity with logical writeback;
- success-path map export identity;
- failure-path lazy subject parity;
- no production default flip.

## Stage B - WB11 Integrated Pilot

Port one representative WB11 daily flow to array-authoritative state. The pilot
must include real daily work, not only writeback application:

- runoff reconciliation;
- frost/snow relevant state access if active for the selected fixture;
- typed guards/conservation checks;
- scheduler apply step;
- outlet/publication materialization for the pilot assertion.

Gates:

- bit-identical output for the piloted flow against current production path;
- H2637 no-UI timing on the same machine used by PERFIDX06;
- perf evidence showing no per-day full `BTreeMap` export and no normal
  dual-write.

## Stage C - Scheduler Authority Flip

If Stage B passes, move lane state ownership to dense authority:

- lane state stores dense state/flux as mutable authority;
- transfer helpers insert/remove by id;
- persistent scheduler lifecycle moves dense state instead of cloning logical
  maps;
- logical surface appears only at validation/publication/debug boundaries.

Gates:

- MOFE/FARPOINT closure fixtures remain bit-identical for migrated paths;
- consumer-boundary validation can operate by id or via explicit materialized
  boundary views;
- H2637 timing improves relative to Stage B.

## Stage D - Kernel Family Expansion

Expand by family, not by isolated helper:

- WB11 runoff/frost/snow;
- plant/percolation/decomposition;
- lateral drainage;
- storage/erosion phases;
- watershed validation separately, because it has distinct surface authority.

Each family gets a shadow-then-flip window:

- shadow array result against logical output;
- assert identity;
- flip the family;
- delete or gate the shadow cost from normal timing.

## Stage E - Publication Boundary Cleanup

Move output builders and CLI report readers to explicit materialized views:

- HBP and parquet row builders receive logical views at output time;
- CLI watershed summaries read materialized report surfaces;
- no output writer reaches into hot-loop dense slots without a documented
  view boundary.

Gates:

- output schema unchanged;
- HBP/parquet identity against pre-migration production for required fixtures;
- output writing remains absent from H2637 dominant perf samples.

## Stage F - Remove Read-Mirror Remnants

After all hot kernel families use dense authority:

- remove normal-path `indexed_writeback_surface` mirror maintenance;
- retain conversion adapters only for tests/debug/publication;
- add anti-regression guards for full-map export inside daily loops and
  logical + indexed dual-write.

Gates:

- `cargo fmt --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `cargo deny check`;
- H2637 no-UI and with-UI ratio report;
- targeted science-contract suites touched by each family;
- line-count and complexity checks for touched large files.

## Stop Conditions

Stop and write a NO-GO disposition if the integrated WB11 pilot shows either:

- the correct array path remains above the <=10x per-OFE-day budget after
  removing logical dual-write and per-day export;
- required conservation/guard/publication work dominates enough that 73x is the
  honest floor;
- the implementation requires broad science-contract changes instead of a
  representation-only migration.
