# Verification

Status: executed-held.

## Static

- Static: `DirectPublicationDayInput` now carries
  `erosion_producer_required`.
- Static: `DirectPublicationDayRow::from_day_frame` calls
  `direct_publication_erosion_operands(day_input)` and fails with
  `MissingDirectUpstream` when erosion authority is required but no direct
  sediment producer has populated publication operands.
- Static: `DirectPublicationDayInputBuilder` has an erosion guard that is
  active only for `DirectProductionExecutor`; shadow/cutover diagnostics remain
  compatibility-owned.
- Static: direct production calls
  `DirectPublicationDayInputBuilder::new_with_seed_surfaces_and_erosion_guard`
  with `erosion_guard_active = true`.
- Static: no compatibility WB13 row, HBP byte, public-output builder, or
  runtime-surface sediment alias is accepted as direct production authority.

## Ran

- Ran: `cargo fmt --check` passed.
- Ran: `cargo test -p openwepp-hillslope-orchestrator
  r7d5_erosion_active_publication_fails_closed_without_direct_sediment_producer --lib`
  passed.
- Ran: `cargo test -p openwepp-hillslope-orchestrator
  r7d4_publication_capture_copies_mofe_carry_to_downstream_lane_before_r4j --lib`
  passed.
- Ran: `cargo test -p openwepp-hillslope-orchestrator
  r4j_runon_carry_consumes_dynamic_transfer_arrays_and_feeds_total_runon --lib`
  passed.
- Ran: `cargo test -p openwepp-runner direct_production --lib` passed.
- Ran: `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
  passed.
- Ran: H2637 direct production failed closed at the R7D5 missing producer guard
  with exit code `1`.
- Ran: `git diff --check` passed.
- Ran: trace-marker scan over `crates/` for R7D temporary debug markers
  returned no matches.

## Not Run

- Full Rust closure gates (`cargo clippy --workspace --all-targets -- -D
  warnings`, `cargo test --workspace`, `cargo deny check`) were not run because
  the package is intentionally executed-held before R7D closure.
- Scoped Markdown lint wrapper was unavailable in this repo context; package
  Markdown was updated directly and will be covered by the repository docs
  tooling when available.
