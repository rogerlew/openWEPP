# Review Disposition

Status: complete.

## Review A

- Static/Ran: accepted. The direct HBP consumer path now reads the
  `DirectRunPublicationFrame` and fail-closes on missing producer-authoritative
  HBP sediment operands. The fresh H2637 5-day direct run reports
  `publication_source = direct-publication-frame` and
  `compatibility_edge_invocations = 0`.

## Review B

- Static/Ran: accepted. The package preserved protected output identity:
  HBP/loss/PASS/PLOT/WAT bytes match between fresh default and direct outputs,
  and parsed HBP latest-event fields match for peak, duration, detachment,
  deposition, sediment concentration, and particle fraction.

## Finding Disposition

- Fixed: R7D7 HBP EROD15 sediment-export alias residual. Direct HBP now
  publishes producer-authoritative `total_detachment_kg`,
  `total_deposition_kg`, and HBP sediment concentration aliases from typed
  direct erosion publication state.
- Fixed: final-gate R6J PASS `peakro` mismatch. The cutover adapter indexes
  PASS publication scalars by simulation day instead of applying one stale
  final runtime scalar to all rows.
- Fixed: stale R6I direct/compatibility PMET parity expectation. The current
  R7D test asserts direct WB14 lineage is preserved and not forced through
  compatibility stale infiltration authority.
- Verified: H2637 5-day default/direct output identity and direct manifest
  no-compatibility-edge evidence under `/tmp/r7d8ad-h2637-5day`.
- Verified: final closure gates passed:
  `cargo fmt --check`, `git diff --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `cargo deny check`, and release CLI build.
- Dispositioned: line-count governance. `04_direct_publication.rs` is below
  WARN; `direct_publication/day_input_and_helpers.rs` is WARN-only at 2421
  lines, below the 3000-line closure block, with mechanical follow-on split
  intent recorded in `artifacts/line-count.md`.
- Process note: delegated subagent review is not claimed; the package lacks the
  explicit subagent authorization language required by
  `docs/work-packages/AGENTS.md`.
