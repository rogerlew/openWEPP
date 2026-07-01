# Consumer-Path Evidence

Status: `EXECUTED`

W3 consumer-path proof for the real public `openwepp-cli-watershed` runner.

Evidence class: `Static:` plus `Ran:` focused CLI tests.

- Producer source: `build_watershed_run_plan` constructs `HillslopeJob` values
  and `PassInventoryExpectation` values from the resolved public runfile in
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs:539`.
- Worker-pool state or frame object: `WatershedRunPlan::execute_hillslope_jobs`
  prepares every generated runfile, then calls `HillslopeWorkerPool::execute`
  in `crates/openwepp-runner/src/watershed_supervisor.rs:82`.
- Bounded execution: `HillslopeWorkerPool` caps active workers to
  `min(--jobs, job_count)`, launches real `std::process::Command` children via
  explicit argv, stops launching pending jobs on the first hard failure, and
  joins launched workers before returning in
  `crates/openwepp-runner/src/watershed_supervisor.rs:131`.
- Runner handoff: the public CLI calls
  `run_plan.execute_hillslope_jobs(...)` before pass inventory validation in
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs:348`.
- Pass inventory handoff: the same public path immediately calls
  `run_plan.validate_pass_inventory()` and `PassInventoryEntry::validate`
  requires generated freshness/timing, parses HBP with expected hillslope id,
  and requires latest `EventPayload` in
  `crates/openwepp-runner/src/watershed_supervisor.rs:480`.
- Downstream routing/publication consumer: validated pass entries populate the
  watershed runtime surface in
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs:352`, then
  `execute_watershed_dispatch_with_kernel` and
  `write_watershed_interchange_outputs` consume that state in
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs:462`.
- Output/API surface: `--jobs` is parsed by the public CLI at
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs:118`; positive
  `N > 1` is accepted by `parse_jobs_arg` at
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs:506`; CLI help now
  advertises `[--jobs N]` at
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs:2239`.
- Positive evidence: `cargo test -p openwepp-runner --test
  watershed_cli_behavior_contract wshedw3 -- --nocapture` passed `3` W3 tests,
  including `--jobs 1`/`--jobs 3` output row-equivalence, per-job artifact
  isolation, child-failure pending-job skip, and missing-pass inventory failure
  before routing.
- Negative proof that old shell-loop/shared-output orchestration is not used:
  generated jobs are rewritten into per-hillslope output roots, stale pass,
  manifest, timing, freshness, stdout, and stderr artifacts are removed before
  launch in `crates/openwepp-runner/src/watershed_supervisor.rs:341`, and the
  tests assert generated mode does not rely on shared `run_dir/H*.hbp` files.
- What still reads the old path: the routed watershed stage still uses the old
  `WatershedWritebackSurface` runtime surface by design; W4 owns typed
  watershed network-frame replacement. W3 only claims bounded hillslope
  subprocess orchestration before that routed-stage consumer.

Producer-only, test-only, timing-only, or counter-only evidence cannot close the
W3 consumer-path gate.
