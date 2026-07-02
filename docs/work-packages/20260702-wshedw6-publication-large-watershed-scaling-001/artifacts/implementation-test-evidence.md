# Implementation and Test Evidence

Status: `passed`

Evidence mode: `Ran:`

## Implementation Summary

- Added `write_typed_publication_parquet_outputs` in
  `crates/openwepp-watershed-output/src/writers.rs`.
- Public watershed CLI now writes outputs directly from
  `WatershedPublicationFrame`.
- Removed public CLI publication conversion through compatibility-shaped row
  seed staging.
- Added focused writer tests proving nonzero typed publication fields populate
  the existing parquet schemas directly and unavailable typed operands remain
  null.
- Added full `onshore-xenophobia` committed fixture under
  `tests/fixtures/watershed/onshore-xenophobia/`.
- Added fixture-contract tests proving `onshore-xenophobia` is a committed full
  `1305`-hillslope gate and `carnivorous-adobo` includes committed W6 launch
  runfiles.
- Fixed existing-pass manifest publication so validated
  `publication_area_m2` is used when source runfiles are not present, with a
  focused regression asserting `Area`, `Runoff`, and `Q`.

## Focused Tests

| Command | Result |
| --- | --- |
| `cargo fmt` | PASS |
| `cargo test -p openwepp-watershed-output typed_publication_writer` | PASS |
| `cargo test -p openwepp-runner --test watershed_cli_behavior_contract wshedw5_public_cli_uses_typed_network_and_publication_frames` | PASS |
| `cargo test -p openwepp-runner --test watershed_cli_behavior_contract watershed_cli_mofe05_accepts_valid_multiofe_metadata_and_emits_outputs` | PASS |
| `cargo test --test cli03_runner_contract_derived_tests cli03_watershed_cli_surface_uses_runfile_pattern_with_legacy_discovery_flag` | PASS |
| `cargo test --test infile_watershed_structure_parser_contract committed_fixture` | PASS |
| `cd tests/fixtures/watershed/onshore-xenophobia && sha256sum --quiet -c input-manifest.sha256` | PASS |
| `cd tests/fixtures/watershed/carnivorous-adobo && sha256sum --quiet -c input-manifest.sha256` | PASS |

## Scaling Runs

| Fixture | Jobs | Result | Evidence |
| --- | ---: | --- | --- |
| `onshore-xenophobia` | `1` | PASS | `/tmp/wshedw6_onshore_scaling_final/jobs1-full` |
| `onshore-xenophobia` | `48` | PASS | `/tmp/wshedw6_onshore_scaling_final/jobs48-full` |
| `carnivorous-adobo` | `1` | PASS | `/tmp/wshedw6_carnivorous_scaling_final/jobs1-full` |
| `carnivorous-adobo` | `32` | PASS | `/tmp/wshedw6_carnivorous_scaling_final/jobs32-full` |

Summary artifacts:

- `artifacts/scaling/onshore-xenophobia-scaling-summary.json`
- `artifacts/scaling/carnivorous-adobo-scaling-summary.json`
- `artifacts/scaling/w6-scaling-summary.csv`

Final closure commands are tracked in `artifacts/gate-results.md`.
