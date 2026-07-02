# Protected Output Evidence

Status: `PASS-WITH-SCOPED-COMMITTED-FIXTURE-LIMITATION`

Evidence class: `Ran` and `Static`.

Public CLI output evidence:

- Command:
  `cargo test -p openwepp-runner --test watershed_cli_behavior_contract --
  --nocapture`.
- Result: 24 tests passed.
- The behavior contract runs the public `openwepp-cli-watershed` binary.
- It verifies required watershed interchange Parquet outputs are emitted.
- It decodes Parquet rows through `SerializedFileReader`.
- `wshedw3_watershed_cli_worker_pool_matches_jobs1_outputs_and_isolates_artifacts`
  compares all decoded watershed output rows between jobs=1 and jobs=N and
  requires identical row order/content.
- `wshedw2_watershed_cli_serial_supervisor_generates_pass_inventory_and_routes`
  verifies generated HBP pass payloads are consumed by watershed publication
  with positive peak/runoff volume.
- Negative tests prove missing/stale/generated-pass failure modes skip routing
  and publication.

Protected public output files checked by the behavior contract:

- `ebe_pw0.parquet`
- `chan.out.parquet`
- `chanwb.parquet`
- `chnwb.parquet`
- `soil_pw0.parquet`
- `totalwatsed3.parquet`
- `loss_pw0.hill.parquet`
- `loss_pw0.chn.parquet`
- `loss_pw0.out.parquet`
- `loss_pw0.class_data.parquet`
- `loss_pw0.all_years.hill.parquet`
- `loss_pw0.all_years.chn.parquet`
- `loss_pw0.all_years.out.parquet`
- `loss_pw0.all_years.class_data.parquet`

Committed fixture evidence:

- Command:
  `cargo test --test infile_watershed_structure_parser_contract
  carnivorous_adobo_committed_fixture_is_repo_local_32_hillslope_gate --
  --nocapture`.
- Result: passed.
- This proves the committed carnivorous-adobo watershed parser/input substrate
  remains repo-local and valid.

Scoped limitation:

- `tests/fixtures/watershed/carnivorous-adobo/README.md` states the fixture is
  not a current `openwepp-cli-watershed` end-to-end execution fixture because
  that CLI requires a schema-versioned TOML watershed `.run` with HBP pass
  bindings.
- No carnivorous output identity claim is made in this artifact.
