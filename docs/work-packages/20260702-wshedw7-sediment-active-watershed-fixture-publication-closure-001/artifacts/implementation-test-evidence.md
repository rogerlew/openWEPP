# Implementation and Test Evidence

Status: `passed`

Evidence mode: `Ran:`

## Implementation

Original W7 retained a public watershed supervisor path-resolution fix:
generated hillslope runfiles canonicalize input file paths before child CLI
execution. W7R adds the committed sediment-active fixture and focused guard.

Changed source/test/fixture paths:

- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/fixtures/watershed/p102-sediment-active/`

Retained historical source edit:

- `crates/openwepp-runner/src/watershed_supervisor.rs`

Focused regressions:

- `wshedw7_watershed_cli_generated_mode_accepts_relative_run_dir`
- `wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity`

Focused W7R command:

```sh
cargo test -p openwepp-runner --test watershed_cli_behavior_contract \
  wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity \
  -- --nocapture
```

Result: `1 passed; 0 failed; 25 filtered out`.

## Release Fixture Replay

- `--jobs 1`: passed on `tests/fixtures/watershed/p102-sediment-active/runs`.
- `--jobs 4`: passed on the same fixture.
- All required decoded public parquets are schema/row identical.
