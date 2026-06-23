# Verification

Status: executed-held.

## Static

- Static: searched temporary R7D trace/debug markers across `crates` and
  `docs`; no matches remained for `OPENWEPP_R7D`, `R7DENTRY`, `R7DWB13`,
  `R7DPERCC`, `R7DPERCD`, `R7DLATC`, `R7DLATD`, `R7DSTORC`,
  `R7DSTORD`, `R7DTOPO`, `R7DTMPADJ`, or related R7D trace tokens.
- Static: reviewed direct HBP residual source. `DirectPublicationDayRow`
  still initializes direct-frame erosion operands from
  `DirectPublicationErosionOperands::zero_authority()`, while the runner
  direct-publication bridge only supplies narrow aggregate runtime scalars.
  No direct EROD14/EROD15 sediment producer exists in R7D4 scope.
- Static: `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  owns the HBP payload and MOFE01 M-G sediment-coupling hold authority,
  including explicit `total_detachment_kg`, `total_deposition_kg`, and
  `sediment_concentration_kg_m3,k` runtime/publication handoff obligations.

## Ran

- Ran: `cargo fmt --check` passed.
- Ran: `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
  passed after trace cleanup.
- Ran:
  `cargo test -p openwepp-hillslope-orchestrator r4j_runon_carry_consumes_dynamic_transfer_arrays_and_feeds_total_runon --lib`
  passed.
- Ran:
  `cargo test -p openwepp-hillslope-orchestrator r7d4_publication_capture_copies_mofe_carry_to_downstream_lane_before_r4j --lib`
  passed.
- Ran:
  `cargo test -p openwepp-hillslope-orchestrator r3c_lane_transfer_span_projects_multilane_topology --lib`
  passed.
- Ran: focused H2637 default/direct harness with label `cleaned-r7d4`.
  Default exit 0 at `0.70 s / 51088 KiB`; direct exit 0 at
  `1.12 s / 63312 KiB`.
- Ran: H2637 WAT/PASS value and byte comparison. WAT and PASS are
  byte-identical; no float residuals remain.
- Not run: full `cargo clippy --workspace --all-targets -- -D warnings`,
  full `cargo test --workspace`, and `cargo deny check`. R7D4 closes in a
  named hold before full R7D completion; full closure gates remain required
  for the follow-up that removes the HBP sediment hold.
