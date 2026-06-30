# Verification

Evidence class: Ran

## Commands

Focused compile and tests:

- `cargo check -p openwepp-runner` - PASS.
- `cargo test -p openwepp-runner direct_publication -- --nocapture` - PASS.
- `cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture`
  - PASS.

Direct RSS and identity:

- H2637 full-output baseline from prior package:
  `/tmp/typed-direct-stage0/h2637/output`.
- H2637 minimized-output baseline from prior package:
  `/tmp/typed-direct-stage0/h2637_min/output`.
- H2637 final full-output run:
  `target/release/openwepp-cli-hill --run-dir /tmp/perfho01/run-dirs/h2637
  --run-file /tmp/direct-publication-rss/stage-b3-h2637-full/h2637.run
  --output-dir /tmp/direct-publication-rss/stage-b3-h2637-full/output
  --manifest-path
  /tmp/direct-publication-rss/stage-b3-h2637-full/output/manifest.json`.
- H2637 final minimized-output run:
  `target/release/openwepp-cli-hill --run-dir /tmp/perfho01/run-dirs/h2637
  --run-file /tmp/direct-publication-rss/stage-b3-h2637-min/h2637_min.run
  --output-dir /tmp/direct-publication-rss/stage-b3-h2637-min/output
  --manifest-path
  /tmp/direct-publication-rss/stage-b3-h2637-min/output/manifest.json`.
- `cli01` final short run:
  `target/release/openwepp-cli-hill --run-dir
  /tmp/direct-publication-rss/stage-b3-cli01 --run-file case.run
  --output-dir /tmp/direct-publication-rss/stage-b3-cli01/output
  --manifest-path
  /tmp/direct-publication-rss/stage-b3-cli01/output/manifest.json`.
- `cmp -s` for H2637 full HBP/WAT/PASS/loss/plot - PASS.
- `cmp -s` for H2637 minimized HBP/loss - PASS.

Full gates:

- `cargo fmt --check` - PASS.
- `cargo clippy --workspace --all-targets -- -D warnings` - PASS.
- `cargo deny check` - PASS.
- `bash tools/release/check_authority_suite_antievasion.sh` - PASS.
- `cargo test --test auth11_required_suite_obligation_guards_contract` - PASS.
- `cargo nextest run --workspace --profile full` - FAIL-ENV:
  `1848` passed, `8` failed, `1` skipped.
- `markdown-doc lint --path
  docs/work-packages/20260630-direct-publication-rss-reduction-001 --path
  docs/work-packages/README.md` - PASS, `11` files validated.
- `markdown-doc validate --path
  docs/work-packages/20260630-direct-publication-rss-reduction-001 --path
  docs/work-packages/README.md` - PASS, `11` files validated.

## Nextest Failure Notes

The failed full-profile tests were environment-backed rather than focused on the
RSS edit:

- `hphys0311_runner_negative_fixture_fails_closed_on_missing_source_line`.
- `hphys0312_runner_negative_fixture_fails_closed_on_missing_source_line`.
- `hphys0313_runner_negative_fixture_fails_closed_on_missing_source_line`.
- `hphys0298_harness_rejects_historical_hrsnow_water_equiv_pairing`.
- Three `snowfrost_fidelity_c_diagnostics_contract` tests.
- `owcmp_env_checks_temp_manifest_and_rejects_inventory_run`.

The failing harness/diagnostic tests attempted to launch `.venv/bin/python`,
which was absent in this worktree. The package remains held; do not treat this
as a completed full-gate pass.
