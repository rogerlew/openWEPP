# R6J Output Parity And Reconstruction

Evidence class: Static plus Ran.

## HBP

- Byte identity: `r6j_cutover_parity_evidence_covers_hbp_wat_pass_and_loss`
  compares direct HBP bytes and parsed latest-event payload against the
  compatibility HBP builder for the current fixture.
- Anti-alias fixture: current CLI fixture exercises two publication days and
  direct nonzero liquid/storage operands; nonzero peak-runoff/event-duration
  erosion fixture remains broader R6/R7 hardening risk, not a current R6J
  blocker because R6J did not change erosion process authority.
- Independent reconstruction: direct HBP is reconstructed from the retained
  `DirectRunPublicationFrame` artifact, then compared to compatibility outside
  the production cutover path.
- H2637 scale status: closed. Fresh same-binary default and direct cutover
  HBP are byte-identical: size `5254`, SHA-256
  `a66563877fd9bda4121b5e2c24b9026de611f086d90fd6f7e32e36e9e6d65f72`.

## WAT

- Arrow row/schema/metadata/value parity:
  `r6j_cutover_parity_evidence_covers_hbp_wat_pass_and_loss` compares direct
  WAT rows to compatibility WAT rows with empty reduced mismatch fields, then
  asserts full row equality.
- Multi-OFE identity: `r6j_direct_manifest_provenance_accepts_multiofe_direct_rows`
  constructs a two-OFE/two-day `DirectRunPublicationFrame` and proves direct
  WAT row projection preserves per-OFE identity, day identity, and row count.
  Existing R6F/R6G/R6H marker tests remain as regression coverage for exact WAT
  identity-field reductions.
- Anti-alias fixture: WAT marker tests cover identity/year/ET/storage/profile
  mismatch classifier shape; current cutover fixture exercises direct
  publication row projection and checksum output.
- Independent reconstruction: direct WAT rows are reconstructed from typed
  direct publication rows and compared to compatibility rows only in test
  evidence.
- H2637 scale status: closed. Fresh same-binary default and direct cutover WAT
  parquet are byte-identical: size `17291602`, SHA-256
  `bcd4915c55565e8d0ecb7cfef82aca9b999804061fcb62cc9c7d4165cf506b73`.
  DuckDB row counts are `235961` vs `235961` with zero bidirectional
  `EXCEPT ALL` differences.

## PASS

- Arrow row/schema/metadata/value parity: R6J enabled PASS parquet in the
  focused runner and CLI cutover tests. The parity evidence test compares
  direct PASS rows to `execution.pass_rows` and requires empty reduced mismatch
  fields.
- Anti-alias fixture: R6J specifically exposed and fixed the PASS `year` and
  `peakro` alias gap by using simulation-year numbering and direct erosion
  `peak_runoff_m3_s` only.
- Independent reconstruction: direct PASS rows are reconstructed from typed
  direct publication rows; compatibility PASS is comparison evidence only.
- H2637 scale status: closed. Direct PASS projection is outlet-only for the
  public PASS surface, and PASS Parquet now uses stable Arrow schema metadata.
  Fresh same-binary default and direct cutover PASS parquet are byte-identical:
  size `326858`, SHA-256
  `c792d2f86c495bf7048cf828172acf58458b5d612faa67d07f33153cb6bec439`.
  DuckDB row counts are `12419` vs `12419` with zero bidirectional
  `EXCEPT ALL` differences.

## Loss

- JSON identity: `r6j_cutover_parity_evidence_covers_hbp_wat_pass_and_loss`
  compares direct loss JSON against the compatibility loss JSON builder for
  the current fixture.
- Anti-alias fixture: current fixture distinguishes parsed run/climate/soil
  and sidecar-derived loss fields; broader sidecar anti-alias expansion remains
  R7 hardening risk.
- Independent reconstruction: direct loss JSON is reconstructed from direct
  publication metadata plus parsed static input counts and sidecar state.
- H2637 scale status: direct cutover loss JSON matches the current default
  output: size `350`, SHA-256
  `cf2b28338105224afc053404f838868cdfee17cab9cede9e09ad9f0fed6b864f`.

## Manifest

- Provenance parity: explicit CLI evidence run
  `/tmp/r6j_evidence_oLbost` produced
  `/execution_provenance/publication_source = direct-publication-frame`,
  `/wb13_publication/source = direct-publication-frame`,
  replay candidate count `0`, row count `2`, and direct runtime counters with
  `compatibility_edge_invocations = 0`.
- Metadata parity: direct manifest preserves the existing manifest schema and
  writes output checksums for all public outputs: HBP, loss, PASS parquet, WAT
  parquet, and plot. The multi-OFE direct publication test verifies direct
  manifest row count, per-OFE count, first/last row keys, and MOFE hourly carry
  activation for two-OFE direct rows.
- Checksum parity: independent `sha256sum` over the five output files matched
  the manifest `output_checksums` map exactly for the explicit CLI evidence
  run:
  `H5.hbp=cbe53a3e...4371760`,
  `H5.loss.json=36efc6c8...0676d5e`,
  `H5.pass.parquet=84c8f6ea...c7fe6d9`,
  `H5.plot.parquet=cef62349...68a20ea`,
  `H5.wat.parquet=2216771c...25b051e`.
- Independent reconstruction: manifest direct publication provenance is
  reconstructed from `DirectRunPublicationFrame` row keys/counts/unique OFE
  area and from the run-local direct runtime audit delta, then serialized
  through the production manifest writer.
- H2637 scale status: closed. Opt-in direct cutover manifest reports
  publication source `direct-publication-frame`, row count `235961`,
  publication area `206522.26699999993`, direct output checksums matching the
  five files on disk, and all direct runtime counters at `0`, including
  `compatibility_edge_invocations=0`.
