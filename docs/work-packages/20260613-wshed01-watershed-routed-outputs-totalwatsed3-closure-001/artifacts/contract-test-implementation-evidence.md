# Contract Test Implementation Evidence

Status: T-B2-REDO2 executed

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

Required W-D tests:

- Assert exact totalwatsed3 volume fields are emitted as `m^3` while depth
  aliases remain mm.
- Assert MOFE `latqcc` aggregation uses only outlet-facing OFEs when OFE
  identifiers are present.
- Assert profile/interception fields pass through from WAT aggregation into
  `totalwatsed3`.

W-D tests added:

- `writer_preserves_multiple_watershed_daily_rows_and_wat_fields` now asserts
  volume-vs-depth mapping and profile/interception publication.
- `aggregate_file_rows_uses_outlet_lateral_and_preserves_optional_wat_fields`
  verifies outlet-only lateral aggregation and optional profile/interception
  pass-through in the WAT daily aggregator.
- `optional_f64_value_treats_all_null_column_as_absent_but_rejects_mixed_nulls`
  treats all-null optional WAT columns as absent-equivalent and rejects mixed
  null/value optional columns as typed null ingestion failures.

W-D green evidence:

- `cargo test -p openwepp-runner watershed_wat::tests -- --nocapture`:
  `2` passed.
- `cargo test -p openwepp-watershed-output writer_preserves_multiple_watershed_daily_rows_and_wat_fields -- --nocapture`:
  `1` passed.
- `cargo test --workspace`: pass.

W-D held gate:

- The real configured and legacy-discovery totalwatsed3 audits still fail
  conservation closure with `2950.498418 mm` whole-run residual. The T-arc
  supersedes W-D-REDO-in-watershed-CLI; T-B must add independent daily PASS
  `runvol` lineage and associated tests in the dedicated CLI.

Required T-B tests defined by T-A:

- Binary/contract test: `openwepp-cli-totalwatsed3` exists and rejects missing
  required PASS/WAT inputs with typed errors.
- Schema test: emitted `totalwatsed3.parquet` includes audit-required columns,
  exact hydrology volumes in `m^3`, and depth aliases in `mm`.
- Operand-lineage test: fixture PASS `runvol` differs from WAT
  `Q * Area / 1000`; output `runvol` and `Runoff` must come from PASS while
  WAT `Q` remains diagnostic.
- MOFE collapse test: internal non-outlet `latqcc` is excluded, outlet OFE
  `latqcc` is retained, and `QOFE` is summed by area-weighted volume.
- Optional-field test: missing `Interception` publishes `0.0`; present
  `Interception` and profile fields are area-weighted from WAT.
- Real-run test: arboreal-dendrite inputs produce a readable
  `totalwatsed3.parquet` without schema repair.

T-A added no Rust tests because it is a no-production-code design increment.

T-B tests added:

- `crates/openwepp-runner/tests/totalwatsed3_cli_contract.rs`:
  - proves the CLI fails closed with typed error `CLITW3-E-004` when required
    PASS input is missing;
  - builds a fixture where PASS `runvol` differs from WAT `Q * Area / 1000`
    and verifies output `runvol`/`Runoff` come from PASS while `Q` remains the
    WAT diagnostic depth;
  - verifies MOFE `latqcc` keeps only the outlet OFE contribution.
- `tests/integration/sim_contract_boundary_unit_registry.rs`:
  `wshed01_totalwatsed3_runoff_unit_lineage_is_pass_volume_publication`
  proves `watershed_totalwatsed3.Runoff` is a publication-only PASS-volume
  depth rather than `hillslope_wat.Q` lineage.

T-B red evidence:

- `cargo test -p openwepp-runner --test totalwatsed3_cli_contract` failed
  before production implementation because Cargo had no
  `openwepp-cli-totalwatsed3` binary target to populate
  `CARGO_BIN_EXE_openwepp-cli-totalwatsed3`.

T-B green evidence:

- `cargo test -p openwepp-runner --test totalwatsed3_cli_contract`: `2`
  passed.
- `cargo test --test sim_contract_boundary_unit_registry`: `15` passed.

T-B held-for-T-C evidence:

- The real arboreal-dendrite audit now reads the native output and reports
  zero profile violations, but the closure residual is still `57.409871 mm`.
  T-B therefore closes the implementation/test gate, not the package closure
  gate.

Required T-B2 tests:

- PASS publication test: MOFE `runvol` must equal terminal outlet routed
  runoff volume over hillslope publication area, not a per-OFE area-weighted
  sum and not WAT `Q`.
- Native reader test: totalwatsed3 must consume openWEPP per-hillslope
  `H*.pass.parquet`/`H*.wat.parquet` inputs when combined files are absent.
- Unit-registry test: `hillslope_pass` schema units must be present in the
  canonical output unit registry.

T-B2 tests added:

- `mofe01_tb2_pass_runvol_uses_terminal_outlet_transfer_volume_not_per_ofe_sum`
  builds a two-OFE fixture where outlet delivery over publication area differs
  from a per-OFE volume sum.
- `totalwatsed3_cli_reads_openwepp_per_hillslope_pass_and_wat_surfaces`
  verifies native per-hillslope PASS/WAT discovery and WAT per-file `wepp_id`
  override.
- `hphys0278_output_unit_registry_covers_output_schema_unit_metadata` now
  covers `hillslope_pass_schema`.

T-B2 red evidence:

- The PASS publication test initially failed to compile because the
  `hillslope_pass` module and `append_runoff_delivery_rows_to` method did not
  exist.
- The native reader test initially failed with `CLITW3-E-004` because the CLI
  required combined `H.pass.parquet`.

T-B2 green evidence:

- `cargo test -p openwepp-runner mofe01_tb2_pass_runvol_uses_terminal_outlet_transfer_volume_not_per_ofe_sum -- --nocapture`:
  `1` passed.
- `cargo test -p openwepp-runner --test totalwatsed3_cli_contract totalwatsed3_cli_reads_openwepp_per_hillslope_pass_and_wat_surfaces -- --nocapture`:
  `1` passed.
- `cargo test --test sim_contract_boundary_unit_registry hphys0278_output_unit_registry_covers_output_schema_unit_metadata -- --nocapture`:
  `1` passed.

T-B2 held-for-T-C evidence:

- T-B2 closes the native runoff-delivery publication gap. It does not claim
  totalwatsed3 conservation closure; that remains T-C.

Required T-B2-REDO tests:

- Replace the old `QOFE * publication area` fixture with a fixture that
  separates published `Q` from `QOFE`.
- Assert corrected PASS `runvol = Q * Area`.
- Assert the old `QOFE * publication area` and mismatched
  `Q * outlet-record area` formulas are rejected.
- Replace the hollow PASS identity with a real-run independent annual
  precipitation bound.

T-B2-REDO tests updated:

- `mofe01_tb2_redo_pass_runvol_uses_published_q_area_not_qofe_area` now uses a
  two-OFE fixture with `Q=2.5 mm`, `QOFE=5.0 mm`, and `Area=200 m2`; correct
  PASS `runvol` is `0.5 m3`.

T-B2-REDO red evidence:

- `cargo test -p openwepp-runner mofe01_tb2_redo_pass_runvol_uses_outlet_ofe_area_not_hillslope_area -- --nocapture`
  failed before the producer correction at the expected `0.5 m3` assertion.

T-B2-REDO green evidence:

- `cargo test -p openwepp-runner mofe01_tb2_redo_pass_runvol_uses_published_q_area_not_qofe_area -- --nocapture`:
  `1` passed.
- `cargo test -p openwepp-runner --test totalwatsed3_cli_contract totalwatsed3_cli_reads_openwepp_per_hillslope_pass_and_wat_surfaces -- --nocapture`:
  `1` passed.
- `cargo test --workspace`: pass.

T-B2-REDO real-run bound evidence:

- Corrected arboreal-dendrite PASS output under
  `/tmp/openwepp_wshed01_tb2_redo_qarea` matches `Q * Area` over `78912` rows
  with `max_abs_pass_minus_q_area_m3=0.0`.
- Water-year annual precipitation bound passes for all `252`
  hillslope-water-years: `violation_count=0`,
  `max_runvol_precip_ratio=0.9857497687436844`.

T-B2-REDO held-for-T-C evidence:

- The corrected totalwatsed3 output is readable, but the wepppy audit reports
  `closure_reconstructed_with_storage_total_mm=6948.564523`. T-C owns closure
  attribution and any further correction.
- T-B2-REDO is superseded by T-B2-REDO2 because the accepted
  `Q * outlet Area` formula crossed publication operands and under-scaled
  native PASS `runvol`.

Required T-B2-REDO2 tests:

- Invert the REDO fixture so it asserts `QOFE * outlet Area`, not
  `Q * outlet Area`.
- Prove the real native PASS surface matches independent WAT outlet
  `QOFE * Area / 1000`.
- Run the actual totalwatsed3 closure audit; acceptance is the day-1/ex-day-1
  closure split, not a one-sided precipitation ratio.

T-B2-REDO2 tests updated:

- `mofe01_tb2_redo2_pass_runvol_uses_qofe_outlet_area_not_q_outlet_area` uses
  the same two-OFE fixture with `Q=2.5 mm`, `QOFE=5.0 mm`, and
  outlet WAT row `Area=200 m2`; correct PASS `runvol` is now `1.0 m3`.
- The fixture passes a distinct `300 m2` publication-area argument so
  `QOFE * publication Area` is also rejected.

T-B2-REDO2 red evidence:

- `cargo test -p openwepp-runner mofe01_tb2_redo2_pass_runvol_uses_qofe_outlet_area_not_q_outlet_area -- --nocapture`
  failed before the producer correction at the expected `1.0 m3` assertion.

T-B2-REDO2 green evidence:

- `cargo test -p openwepp-runner mofe01_tb2_redo2_pass_runvol_uses_qofe_outlet_area_not_q_outlet_area -- --nocapture`:
  `1` passed.
- `cargo test -p openwepp-runner --test totalwatsed3_cli_contract totalwatsed3_cli_reads_openwepp_per_hillslope_pass_and_wat_surfaces -- --nocapture`:
  `1` passed.
- `cargo test --test sim_contract_boundary_unit_registry hphys0278_output_unit_registry_covers_output_schema_unit_metadata -- --nocapture`:
  `1` passed.
- `cargo test --workspace`: pass.

T-B2-REDO2 real-run closure evidence:

- Corrected arboreal-dendrite PASS output under
  `/tmp/openwepp_wshed01_tb2_redo2_qofearea_20260614T213618Z` matches WAT
  outlet `QOFE * Area / 1000` over `78912` rows with
  `max_abs_pass_minus_qofe_area_m3=0.0`.
- `openwepp-cli-totalwatsed3` wrote `2192` rows from those corrected
  per-hillslope PASS/WAT files; totalwatsed3/PASS `runvol` sum diff is
  `-4.0978193283081055e-08 m3`.
- wepppy closure audit reports
  `closure_reconstructed_with_storage_total_mm=30.544142`; day 1 is
  `+30.9533178099056 mm`, and excluding day 1 the basic-storage residual is
  `-0.409175395336963 mm` over `2191` days with `0` days above `1 mm`.
