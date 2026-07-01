# Consumer-Path Evidence

Status: `EXECUTED-COMPLETE`

Static:

- Producer source:
  `crates/openwepp-runner/src/watershed_supervisor.rs:19` defines
  `WatershedRunPlan`; `:110` defines `HillslopeJob`; `:261` defines
  `PassInventory`; `:281` defines `PassInventoryEntry`.
- Serial supervisor execution:
  `crates/openwepp-runner/src/watershed_supervisor.rs:72` executes
  hillslope jobs serially for `--jobs 1`; `:196` builds explicit child argv
  through `build_hillslope_argv`; `:206` launches the child with
  `Command::new` and explicit args; `:198`/`:199` bind stdout/stderr to
  per-job log files; `:235` writes per-job timing JSON.
- Generated-artifact freshness:
  `crates/openwepp-runner/src/watershed_supervisor.rs:131` removes stale
  generated pass/manifest/timing/marker artifacts before launching a child;
  `:201` writes a freshness marker immediately before launch; `:364` validates
  generated pass, manifest, and timing artifacts against that marker before
  routing.
- Public runner handoff:
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs:123` parses
  `--jobs`; `:186` builds the `WatershedRunPlan`; `:348` executes generated
  hillslope jobs; `:349` validates the typed pass inventory before routing.
- Downstream consumer:
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs:351` iterates
  `pass_inventory.entries()` and seeds the watershed runtime surface from
  validated `EventPayload` fields; `:462` calls
  `execute_watershed_dispatch_with_kernel`; `:482` writes watershed outputs.
- Reuse mode:
  reuse blocks must explicitly set `use_existing_pass_file = true`, must
  declare `pass_file`, and cannot also declare `run_file`. Generated mode
  requires `use_existing_pass_file = false` plus `run_file`.
- Negative proof:
  no package-local shell loop was introduced; generated jobs use
  `std::process::Command` with explicit argv. Generated pass/manifest/log/timing
  files are written under `--output-dir/hillslope-jobs/H{id}/`, not a shared
  run-directory pass output.
- Latest-event handling:
  `crates/openwepp-runner/src/watershed_supervisor.rs:337` rejects pass
  inventory entries with no latest `EventPayload`, citing absent canonical
  `NoEvent` authority instead of synthesizing zeros.

Ran:

- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract wshedw2 -- --nocapture`
  passed after reviewer fixes: `7 passed`.
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract -- --nocapture`
  passed after reviewer fixes: `20 passed`.

Output/API proof:

- `wshedw2_watershed_cli_serial_supervisor_generates_pass_inventory_and_routes`
  invokes the public `openwepp-cli-watershed` binary with
  `--hillslope-binary <real openwepp-cli-hill>` and `--jobs 1`.
- The test asserts the supervisor-created job artifacts exist:
  `H1.run.toml`, `H1.hbp`, `H1.manifest.json`, `H1.stdout.log`,
  `H1.stderr.log`, and `H1.timing.json`.
- The same test asserts watershed interchange outputs are emitted after pass
  inventory validation and that no pre-existing `run_dir/H1.hbp` is used.
- Additional focused tests prove relative `--output-dir` generated execution,
  explicit reuse selector enforcement, ambiguous reuse block rejection, and
  stale generated pass cleanup/fail-closed behavior.
