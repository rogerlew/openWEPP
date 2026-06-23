# Verification

Status: complete.

## Static

- Static: `build_hbp_output_from_direct_publication` consumes
  `DirectRunPublicationFrame` rows and requires producer-authoritative direct
  erosion operands for HBP detachment, deposition, and sediment concentration.
  Missing or invalid operands fail closed instead of silently zeroing.
- Static: `direct_publication_last_hbp_sediment_row` selects the latest direct
  publication row with positive HBP sediment authority, preventing a later
  no-runoff row from erasing the current event sediment payload.
- Static: `DirectFrameExecutor::new(DirectExecutorMode::ProductionDirect)`
  still records direct counters, and the fresh direct manifest reports
  `compatibility_edge_invocations = 0`.
- Static: R6J PASS `peakro` cutover now indexes simulation-owned PASS rows by
  `sim_day_index`; it no longer reuses a final runtime scalar across all direct
  publication rows.
- Static: R7D PMET boundary test now asserts direct day-2 PMET seed state is
  reconstructed from direct carried layer state and intentionally diverges from
  compatibility stale-infiltration lineage.
- Static: trace/debug marker scan returned no matches for the temporary
  R6I/R7D8 trace marker family.
- Static: line-count governance is dispositioned in
  `artifacts/line-count.md`. The entry file is below the WARN threshold; the
  split include file is above the WARN threshold but below the 3000-line
  refactor-block threshold, with follow-on split intent recorded.

## Ran

- Ran: `cargo fmt --check` passed.
- Ran: `git diff --check` passed.
- Ran: `cargo clippy --workspace --all-targets -- -D warnings` passed.
- Ran: `cargo test --workspace` passed.
- Ran: `cargo deny check` passed:
  `advisories ok, bans ok, licenses ok, sources ok`.
- Ran: `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
  passed.
- Ran: `markdown-doc lint --path docs/work-packages/20260623-r7d8-direct-hbp-erod15-export-alias-parity-001 --path docs/work-packages/README.md --path docs/architecture/array-native-runtime-specification.md --format json`
  scanned 10 files with 0 errors and 0 warnings.
- Ran: `cargo test -p openwepp-hillslope-orchestrator r7d6_typed_erosion_producer_populates_publication_operands --lib -- --nocapture`
  passed.
- Ran: `cargo test -p openwepp-runner hillslope::tests::r6j_cutover_parity_evidence_covers_hbp_wat_pass_and_loss --lib -- --nocapture`
  passed.
- Ran: `cargo test -p openwepp-runner hillslope::tests::r7d_direct_day_two_pmet_seed_keeps_direct_wb14_lineage_boundary --lib -- --nocapture`
  passed.
- Ran: fresh H2637 5-day default/direct parity under
  `/tmp/r7d8ad-h2637-5day` passed with default exit `0`, direct exit `0`,
  HBP/loss/PASS/PLOT/WAT byte identity, and direct
  `compatibility_edge_invocations = 0`.
- Ran: temporary `/tmp/r7d8_hbp_payload_dump` helper compiled against the
  public `openwepp_input_contract::parsers::hbp` parser and confirmed parsed
  HBP latest-event field parity for peak, duration, total detachment, total
  deposition, sediment concentration, and particle flow fraction.

## Not Run

- No required R7D8 closure gate was skipped.
- Delegated subagent review was not claimed because this package does not
  contain explicit subagent authorization language.
