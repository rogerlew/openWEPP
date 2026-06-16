# CQR31 Disposition

Disposition: accept.

Accepted changes:

- Behavior-preserving private decomposition of
  `build_simulation_owned_wb13_row_for_ofe`.
- Existing WB13 publication tests retained as characterization coverage.
- Target CRAP reduced from `251.62932776803854` to `16.0`.
- Highest newly extracted helper CRAP is `12.584884659264825`.
- Previous target-level `clippy::too_many_lines` suppressions removed.

Open findings: none.

Warnings:

- `cargo crap` reported the established `126` source-map warnings for LCOV
  entries on both before and after runs.
- `derive_profile_fc_store_from_authoritative_layers` remains above CRAP `30`
  in the same target file, but it is an existing out-of-scope function for this
  row.

Status: accepted pending package push.
