# Verification

Status: `EXECUTED-COMPLETE`

Static:

- Public binary handoff path:
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs:186`,
  `:348`, `:349`, `:351`, `:462`, `:482`.
- Supervisor module:
  `crates/openwepp-runner/src/watershed_supervisor.rs`.
- Focused public CLI tests:
  `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs:124`,
  `:199`, `:257`, `:288`, `:334`, `:375`, `:420`.

Ran locally:

| Command | Result |
| --- | --- |
| `cargo check -p openwepp-runner --bins` | `PASS` |
| `cargo fmt` | `PASS` |
| `cargo test -p openwepp-runner --test watershed_cli_behavior_contract wshedw2 -- --nocapture` | `PASS` (`7 passed`) |
| `cargo test -p openwepp-runner --test watershed_cli_behavior_contract -- --nocapture` | `PASS` (`20 passed`) |
| `cargo clippy -p openwepp-runner --all-targets -- -D warnings` | `PASS` |
| `markdown-doc lint --path docs/work-packages/20260701-wshedw2-serial-watershed-supervisor-skeleton-001 --path docs/ROADMAP.md --path docs/work-packages/README.md` | `PASS` (`14 files validated, 0 errors, 0 warnings`) |
| `git diff --check` | `PASS` |

Focused behavior verified:

- `--jobs 0`, non-integer `--jobs`, and `--jobs >1` values fail with
  `CLIWAT-E-041`.
- `--jobs 1` generated mode creates per-job output/log/timing artifacts under
  `--output-dir/hillslope-jobs/H1/`.
- The public watershed CLI launches the real hillslope CLI subprocess and then
  routes from the generated HBP pass inventory.
- Reuse-mode watershed CLI tests still pass.
- Reuse mode is explicit: omitted `use_existing_pass_file` fails, and reuse
  blocks cannot also declare `run_file`.
- Relative `--output-dir` generated runs succeed.
- Stale generated pass/manifest/timing artifacts are removed before child
  execution and a child that does not publish a fresh pass fails before routing.
- Parseable HBP no-event payloads fail closed with `CLIWAT-E-045` because no
  canonical `NoEvent` authority is cited.

Final delegated closure:

- `cargo fmt --check`: `PASS`
- `cargo clippy --workspace --all-targets -- -D warnings`: `PASS`
- `cargo nextest run --workspace --profile full`: `PASS`
  (`1280 tests run: 1280 passed (1 slow), 1 skipped`)
- `cargo deny check`: `PASS`
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract -- --nocapture`:
  `PASS` (`20 passed`)

Evidence:

- `artifacts/closure/command-log.json`
- `artifacts/closure/summary.md`
- `artifacts/closure/cargo-fmt-check.log`
- `artifacts/closure/cargo-clippy-full.log`
- `artifacts/closure/cargo-nextest-full.log`
- `artifacts/closure/cargo-deny-check.log`
- `artifacts/closure/focused-w2-gate.log`
