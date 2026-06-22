# R6J Blocker Ledger

Evidence class: Static plus Ran.

Every reproduced blocker was treated as an iteration target until direct
publication cutover closed:

`COMPLETE-R6-DIRECT-PUBLICATION-CUTOVER`.

## Loop 1 - Inherited Manifest Writer Blocker

- Blocker: `R6J-DIRECT-PUBLICATION-CUTOVER-BLOCKER`.
- Marker or symptom: `manifest direct projection is not wired to the production
  manifest writer`.
- Evidence: reproduced with
  `cargo test -p openwepp-runner r6i_cutover_candidate_clears_pmet_layer_ulp_then_fails_manifest_cutover -- --nocapture`
  before correction. The inherited marker passed as the expected fail-closed
  boundary.
- Output family: manifest.
- File / row / field / metadata / byte span: manifest
  `/execution_provenance/publication_source`,
  `/wb13_publication/source`, `/wb13_publication/replay_candidate_surfaces`,
  `/wb13_publication/row_count`, and output checksum map.
- Direct operand: retained `DirectRunPublicationFrame`.
- Producer: cutover simulation-output direct publication adapter.
- Consumer: production run-manifest writer.
- Authority: R6 canonical publication operand ledger in
  `docs/architecture/array-native-runtime-specification.md`.
- In-envelope verdict: in envelope; manifest writer wiring is explicitly in
  scope.
- Correction: `build_hillslope_publication_provenance` now uses
  `build_direct_publication_manifest_provenance` when cutover direct
  publication artifacts are present. Execution provenance reports
  `direct-publication-frame`. Replay candidate surfaces are empty, and manifest
  row keys/counts come from direct publication rows.
- Tests / fixtures: R6J runner cutover test and CLI cutover contract.
- Validation: focused R6J tests passed; explicit CLI evidence run wrote all
  outputs and manifest with direct provenance.
- Result: closed.

## Loop 2 - PASS Parquet Optional Output Blocker

- Blocker: direct cutover did not exercise optional PASS output before manifest
  wiring was closed.
- Marker or symptom: after enabling `pass_parquet`, cutover reduced to
  `PASS row identity failed: direct_rows=2 compatibility_rows=2
  reduced_fields=year,peakro`.
- Evidence: focused R6J test with temp `case.run` appending
  `pass_parquet = "output/H5.pass.parquet"` exposed the mismatch.
- Output family: PASS.
- File / row / field / metadata / byte span: PASS row fields `year` and
  `peakro`.
- Direct operand: `publication.calendar.year` and
  `publication.erosion.peak_runoff_m3_s`.
- Producer: direct publication projection.
- Consumer: PASS parquet writer.
- Authority: existing PASS schema and R6 ledger rows for PASS `peakro` and
  calendar identity.
- In-envelope verdict: in envelope; PASS projection is R6J scope.
- Correction: direct PASS projection now uses simulation-year numbering and
  direct erosion `peak_runoff_m3_s` without falling back to HBP/runoff peak
  fields.
- Tests / fixtures: `r6j_cutover_parity_evidence_covers_hbp_wat_pass_and_loss`
  and CLI cutover contract both enable PASS parquet.
- Validation:
  `cargo test -p openwepp-runner r6j_cutover_parity_evidence_covers_hbp_wat_pass_and_loss -- --nocapture`
  passed.
- Result: closed.

## Loop 3 - Runtime Compatibility Oracle In Cutover Gate

- Blocker: after public writes succeeded, the production cutover gate still
  built compatibility HBP/loss/PASS/WAT artifacts as an in-run parity oracle.
- Marker or symptom: static scan found compatibility builders inside
  `require_direct_publication_cutover_gates`.
- Evidence: pre-correction production gate invoked `build_hbp_output`,
  `build_loss_output_json`, `build_hillslope_wat_rows(&execution.wb13_rows)`,
  and compared direct PASS rows to `execution.pass_rows`.
- Output family: HBP, WAT, PASS, loss.
- File / row / field / metadata / byte span: production cutover gate.
- Direct operand: direct publication artifacts already built from
  `DirectRunPublicationFrame`.
- Producer: direct projection consumers.
- Consumer: direct cutover writer.
- Authority: R6 terminal state requires no compatibility authority in the
  cutover path; parity belongs in test/evidence gates, not production authority.
- In-envelope verdict: in envelope; consumer path closure is explicitly in
  scope.
- Correction: production cutover gate now validates direct producer presence,
  non-empty direct HBP/loss artifacts, and direct optional projection row counts
  only. Compatibility parity checks were moved to focused tests as evidence.
- Tests / fixtures: `r6j_cutover_candidate_writes_direct_outputs_and_manifest`
  asserts public writes and manifest counters; parity evidence test compares
  direct HBP/WAT/PASS/loss against compatibility outside production cutover.
- Validation: focused tests and CLI contract passed.
- Result: closed.

## Loop 4 - Manifest Direct Runtime Counter Gap

- Blocker: direct manifest provenance did not include direct runtime counters.
- Marker or symptom: architecture ledger names `direct_runtime_counters`, but
  manifest initially omitted that field.
- Evidence: explicit CLI evidence run showed direct publication provenance but
  no `direct_runtime_counters` object.
- Output family: manifest.
- File / row / field / metadata / byte span: manifest
  `/direct_runtime_counters/*`.
- Direct operand: `direct_runtime_audit_snapshot`.
- Producer: direct runtime audit counters.
- Consumer: production manifest writer.
- Authority: R6 ledger row for run manifest provenance/counters.
- In-envelope verdict: in envelope; manifest provenance/counter wiring is R6J
  scope.
- Correction: manifest now serializes optional `direct_runtime_counters` for
  `DirectPublicationFrameCutover` only, skipped for compatibility manifests.
- Tests / fixtures: R6J runner and CLI tests assert
  `run_frame_constructions=1`, `skeleton_runs=0`,
  `publication_capture_runs=1`, and `compatibility_edge_invocations=0`.
- Validation: focused tests passed after the counter field was added.
- Result: closed.

## Loop 5 - Review: Manifest Area Day-Multiplication

- Blocker: direct manifest `publication_area_m2` summed every lane-day row.
- Marker or symptom: correctness review identified a two-day single-lane run
  would report twice the hillslope area.
- Output family: manifest.
- Direct operand: `DirectRunPublicationFrame.rows[*].area_m2`.
- Producer: direct publication row capture.
- Consumer: `build_direct_publication_manifest_provenance`.
- In-envelope verdict: in envelope; direct manifest provenance is R6J scope.
- Correction: manifest facts now collect one stable area per OFE, reject
  inconsistent repeated row areas, require observed OFE count to match
  `identity.lane_count`, and sum unique OFE areas.
- Tests / fixtures: `r6j_direct_manifest_provenance_accepts_multiofe_direct_rows`
  asserts two-OFE/two-day area `1200.0` rather than the day-multiplied `2400.0`.
- Result: closed.

## Loop 6 - Review: Run-Local Counter Provenance

- Blocker: direct manifest counters used the global audit snapshot.
- Marker or symptom: review noted long-lived runner processes could accumulate
  prior direct runtime activity and falsely claim run-local cutover counters.
- Output family: manifest.
- Direct operand: direct runtime audit counter baseline/end snapshot.
- Producer: direct runtime audit counters.
- Consumer: production manifest writer.
- In-envelope verdict: in envelope; R6J added this manifest field.
- Correction: `execute_hillslope_run_with_runtime_selection` captures a
  pre-run baseline and serializes a saturating run-local delta for direct
  cutover manifests.
- Tests / fixtures:
  `r6j_manifest_direct_runtime_counters_are_run_local_after_prior_activity`
  dirties global counters with a shadow run, then verifies the subsequent
  cutover manifest still reports one run frame and one publication capture.
- Result: closed.

## Loop 7 - Review: Direct Output Producer Authority Defaults

- Blocker: direct HBP/PASS consumers could default missing erosion publication
  fields to zero.
- Marker or symptom: review found missing per-family producer authority could
  still pass if hydrology was nonzero.
- Output family: HBP and PASS.
- Direct operand: `DirectPublicationErosionOperands`.
- Producer: direct publication row capture.
- Consumer: direct HBP and PASS projection consumers.
- In-envelope verdict: in envelope for the current output contract.
- Correction: direct publication rows carry explicit zero-authority erosion
  operands for the current engine output contract. Direct HBP/PASS consumers
  now require producer-authoritative erosion fields and reject missing,
  non-finite, or negative values instead of defaulting absent values.
- Tests / fixtures: R6J focused parity, CLI contract, full workspace test, and
  H2637 direct-cutover endpoint all exercised the required fields.
- Result: closed for the current output contract; broader nonzero erosion
  process authority remains outside R6J.

## Loop 8 - Review: Shadow Manifest Provenance Leak

- Blocker: direct manifest provenance was selected whenever
  `execution.direct_publication` existed, including shadow mode.
- Marker or symptom: review found `DirectPublicationFrameShadow` could write
  compatibility outputs with direct manifest provenance.
- Output family: manifest.
- Direct operand: runtime selection.
- Producer: runtime selection dispatch.
- Consumer: manifest publication provenance builder.
- In-envelope verdict: in envelope.
- Correction: `build_hillslope_publication_provenance` now gates direct
  provenance on `DirectPublicationFrameCutover` only. Shadow mode remains
  compatibility-provenanced.
- Tests / fixtures: existing R6A shadow test plus full workspace test.
- Result: closed.

## Loop 9 - H2637 Multi-OFE Public Output Parity

- Blocker: H2637 opt-in direct cutover completed but HBP/WAT/PASS did not yet
  satisfy the R6 byte/Arrow identity gate.
- Marker or symptom: initial H2637 runs exposed scale-only mismatches after
  current-fixture parity passed.
- Evidence before correction:
  - direct cutover was functional and direct-only with manifest source
    `direct-publication-frame` and `compatibility_edge_invocations = 0`;
  - direct PASS initially projected one row per OFE-day while current public
    PASS is outlet-day;
  - direct HBP differed because HBP needed HBP-specific erosion operands rather
    than PASS zero-authority sediment operands;
  - after row identity was fixed, PASS Parquet still had data-equal byte
    differences caused by nondeterministic Arrow schema metadata ordering.
- Output family: HBP, WAT, PASS.
- File / row / field / metadata / byte span: HBP detachment/deposition/sediment
  operands, PASS outlet-row projection, and PASS Parquet footer/schema
  metadata bytes.
- Direct operand: `DirectRunPublicationFrame` rows plus HBP-specific
  producer-authoritative runtime scalars.
- Producer: cutover simulation-output direct publication adapter from
  `HillslopeClimateExecution` runtime surface and simulation-owned WB13 rows.
- Consumer: direct HBP/PASS projection consumers and PASS Parquet writer.
- Authority: current public PASS is outlet-day; HBP is run-level/outlet event
  output; WAT remains per-OFE-day. R6 allows direct publication consumers to
  project typed direct rows to existing public output shapes without changing
  public schema semantics.
- Correction:
  - direct PASS projection now filters to the outlet lane while preserving WAT
    per-OFE-day projection;
  - direct publication rows carry HBP-specific detachment, deposition, and
    sediment concentration operands sourced from runtime scalars;
  - direct HBP reads HBP-specific erosion operands while PASS retains current
    zero-authority sediment columns;
  - PASS Parquet now uses the stable Arrow schema metadata path already used
    by WAT and has a byte-stability regression test.
- Validation:
  - `cargo test -p openwepp-hillslope-output hillslope_pass -- --nocapture`
    passed and asserted byte-stable PASS writer output;
  - focused R6J runner and CLI cutover tests passed;
  - fresh release default H2637 ran in `640.41 s / 227396 KiB`;
  - fresh release direct H2637 cutover ran in `637.53 s / 349400 KiB`;
  - HBP, WAT parquet, PASS parquet, loss JSON, and plot parquet are
    byte-identical between fresh default and direct H2637 outputs;
  - DuckDB WAT row counts are `235961` vs `235961` with zero bidirectional
    `EXCEPT ALL` differences;
  - DuckDB PASS row counts are `12419` vs `12419` with zero bidirectional
    `EXCEPT ALL` differences;
  - direct manifest reports `direct-publication-frame`, direct row count
    `235961`, publication area `206522.26699999993`, and all direct runtime
    counters including `compatibility_edge_invocations` at `0`.
- Result: closed.
