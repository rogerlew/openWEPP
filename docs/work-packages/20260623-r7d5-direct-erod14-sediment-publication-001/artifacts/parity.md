# Parity Evidence

Status: executed-held.

## Focused Fixtures

- Ran: `cargo test -p openwepp-hillslope-orchestrator
  r7d5_erosion_active_publication_fails_closed_without_direct_sediment_producer --lib`
  passed. The fixture sets `erosion_producer_required = true` on a typed direct
  day input and asserts that direct publication fails before emitting
  `DirectPublicationErosionOperands::zero_authority()`.
- Ran: `cargo test -p openwepp-hillslope-orchestrator
  r7d4_publication_capture_copies_mofe_carry_to_downstream_lane_before_r4j --lib`
  passed after the guard change.
- Ran: `cargo test -p openwepp-hillslope-orchestrator
  r4j_runon_carry_consumes_dynamic_transfer_arrays_and_feeds_total_runon --lib`
  passed after the guard change.
- Ran: `cargo test -p openwepp-runner direct_production --lib` passed
  (`2 passed`) after the direct production guard was wired through the runner.

## H2637

- Ran: `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
  passed.
- Ran: focused H2637 direct production command:
  `target/release/openwepp-cli-hill --run-dir /tmp/r7d4-h2637-5day/run
  --run-file /tmp/r7d4-h2637-5day/direct.run --output-dir
  /tmp/r7d4-h2637-5day/manifests/direct-r7d5-failclosed
  --direct-production-executor`.
- Observed exit: `1`.
- Observed stderr:
  `CLIHILL-E-011 runtime surface failure for r7c_direct_production_executor:
  HS-SIMPIPE-E-001 direct runtime upstream span R7D5 direct EROD14/EROD15
  sediment producer must execute before this span`.
- Runtime evidence: `direct elapsed=0.24 rss_kb=51456`.

## Residuals

- R7D4 residual before this guard: WAT and PASS byte-identical, loss/plot
  differing only by `run_name`, HBP differing on sediment-family payload bytes
  at offsets `928`, `936`, and `944`.
- R7D5 intentionally changes the active-sediment direct-production behavior
  from silent zero erosion publication to fail-closed. HBP/PASS sediment parity
  is therefore not claimed by this package.
- Current blocker:
  `HOLD-R7D5-DIRECT-EROD13-EROD14-EROD15-TYPED-PRODUCER-ABSENT`.
