# Worker Handoff

Status: `executed-complete`

Evidence mode: `Static:` handoff plus `Ran:` command evidence.

## Current State

W7R closes W7 as
`EXECUTED-COMPLETE-W7R-SEDIMENT-ACTIVE-PUBLICATION-CLOSURE`.

Retained historical production edit from original W7:

- `crates/openwepp-runner/src/watershed_supervisor.rs` canonicalizes generated
  hillslope child input paths.

New W7R test/fixture work:

- `tests/fixtures/watershed/p102-sediment-active/`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
  `wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity`

## Commands Run

- `cargo build --release -p openwepp-runner --bins`
- `target/release/openwepp-cli-hill --run-dir /tmp/wshedw7r_p102_producer --run-file p102.run --output-dir /tmp/wshedw7r_p102_producer/output --direct-production-executor`
- `cargo nextest run --test erosion_multi_ofe_p102_chain`
- `target/release/openwepp-cli-watershed --run-dir tests/fixtures/watershed/onshore-xenophobia/runs --run-file case.run --output-dir /tmp/wshedw7r_onshore_jobs8 --policy compat --jobs 8 --hillslope-binary target/release/openwepp-cli-hill`
- `target/release/openwepp-cli-watershed --run-dir tests/fixtures/watershed/p102-sediment-active/runs --run-file case.run --output-dir /tmp/wshedw7r_p102_fixture_jobs1 --policy compat --jobs 1 --hillslope-binary target/release/openwepp-cli-hill`
- `target/release/openwepp-cli-watershed --run-dir tests/fixtures/watershed/p102-sediment-active/runs --run-file case.run --output-dir /tmp/wshedw7r_p102_fixture_jobs4 --policy compat --jobs 4 --hillslope-binary target/release/openwepp-cli-hill`
- `(cd tests/fixtures/watershed/p102-sediment-active && sha256sum -c input-manifest.sha256)`
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity -- --nocapture`

The onshore run generated all hillslope HBP files but failed later in WS10
channel dispatch with `WKERNEL-WS10-CHANNEL-E-003`; it was not used as the W7R
acceptance fixture.

## Next Concrete Action

Proceed to the next watershed-facing roadmap item. `WSHED-W7DC01` is historical
unless a fresh producer-side zero-sediment regression appears.
