# Contract Test Implementation Evidence

Status: W-C executed

Evidence mode: Ran + Static

W-A authored no tests because the increment forbade production edits. W-B
implemented the required parser/CLI red tests.

W-B tests added:

- `tests/fixtures/infile/watershed_impoundment/strict_zero_impoundments.imp`
  with supported datver `99.1` and `jpond=0`.
- Parser assertions:
  - strict and compatibility success when
    `expected_structural_count=Some(0)`,
  - typed `IMP-E-007` mismatch when
    `expected_structural_count=Some(1)` and `jpond=0`,
  - bare strict parse still fails as `IMP-E-004`.
- Existing malformed-count, active-payload, and runtime-seed tests preserved.
- Watershed CLI regression:
  `watershed_cli_accepts_explicit_zero_impoundment_file_when_structure_has_none`
  proves the CLI accepts an explicit `jpond=0` input when structure has no
  impoundments and does not emit `CLIWAT-E-010`.

Red evidence:

- `cargo test --test infile_watershed_impoundment_parser_contract zero_impoundments`
  failed before production edits: three new assertions hit the old
  `DomainError { field: "jpond", allowed: ">= 1" }`.

Green evidence:

- `cargo test --test infile_watershed_impoundment_parser_contract`: `18`
  passed.
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract watershed_cli_accepts_explicit_zero_impoundment_file_when_structure_has_none`:
  `1` passed.
- `cargo test -p openwepp-watershed-orchestrator watershed_impoundment_runtime_seed`:
  `3` passed.

Required W-C tests:

- Assert all 14 watershed outputs exist.
- Assert `totalwatsed3.parquet` is not the one-row default writer surface for a
  real routed run.
- Assert reported depth columns match volume-derived depths.
- Add an anti-placeholder gate for required water-balance operands.

W-C tests added:

- `wshed01_wc_zero_sediment_hillslope_payload_allows_zero_fraction_support`
  proves complete zero-sediment HBP payloads do not fail
  `WKERNEL-WS10-CHANNEL-E-003`.
- `wshed01_wc_nchnum_zero_disables_channel_detail_output_without_blocking_routing`
  proves `nchnum=0` is an output-selection state, not a routing domain
  violation.
- `writer_preserves_multiple_watershed_daily_rows_and_wat_fields` proves the
  interchange writer preserves multiple daily rows and maps WAT fields.

W-C red evidence:

- The WS10 zero-sediment test failed before production edits with
  `WKERNEL-WS10-CHANNEL-E-003`.
- The `nchnum=0` test failed before production edits at channel validation.

W-C green evidence:

- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshed01_wc_ -- --nocapture`:
  `2` passed.
- `cargo test -p openwepp-watershed-output writers::tests::writer_ -- --nocapture`:
  `2` passed.
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract watershed_cli_emits_watershed_output_parquet_files -- --nocapture`:
  `1` passed.
- Real arboreal-dendrite CLI runs emit all `14` parquet outputs and a
  `2192`-row `totalwatsed3.parquet`.
