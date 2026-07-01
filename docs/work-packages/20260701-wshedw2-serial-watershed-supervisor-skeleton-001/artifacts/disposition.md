# Disposition

Status: `EXECUTED-COMPLETE-WSHED-W2`

Final disposition: `EXECUTED-COMPLETE-WSHED-W2`.

Static:

- Added `WatershedRunPlan`, `HillslopeJob`, `PassInventoryExpectation`,
  `PassInventory`, and `PassInventoryEntry` in
  `crates/openwepp-runner/src/watershed_supervisor.rs`.
- Public `openwepp-cli-watershed` now accepts `--jobs 1` and
  `--hillslope-binary`, rejects `--jobs 0`, invalid values, and `--jobs >1`
  until W3.
- Generated full-run mode uses explicit `use_existing_pass_file = false`
  hillslope blocks, creates isolated per-hillslope output/log/timing paths,
  launches the real hillslope CLI with explicit argv, validates a typed pass
  inventory, and routes only after validation.
- Routed-stage reuse remains available only through explicit
  `use_existing_pass_file = true` blocks; ambiguous reuse blocks with `run_file`
  fail closed.
- Missing latest-event payloads fail closed with `CLIWAT-E-045`; no `NoEvent`
  authority was admitted.
- Generated pass freshness is checked through stale-output cleanup plus a
  per-job freshness marker; stale generated artifacts cannot route.

Ran:

- `cargo check -p openwepp-runner --bins`: pass.
- `cargo clippy -p openwepp-runner --all-targets -- -D warnings`: pass.
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract wshedw2 -- --nocapture`:
  pass (`7 passed`).
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract -- --nocapture`:
  pass (`20 passed`).
- `git diff --check`: pass.
- Final comparator-suite closure:
  - `cargo fmt --check`: pass.
  - `cargo clippy --workspace --all-targets -- -D warnings`: pass.
  - `cargo nextest run --workspace --profile full`: pass
    (`1280 tests run: 1280 passed (1 slow), 1 skipped`).
  - `cargo deny check`: pass.

Review:

- `rust_code_reviewer` and `rust_qa_reviewer` findings were accepted and fixed.
- Final `comparator_suite_runner` gate run passed.

Residual follow-on:

- W3 remains responsible for bounded parallel `--jobs N`.
- W4/W5 remain responsible for typed watershed network-frame routing and old
  runtime deletion.
- Carnivorous-adobo remains an adopted input/runfile fixture, not an
  end-to-end generated HBP-output gate.
