# Gate Results

Status: completed

Evidence mode: ran

Ran:

- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test -p openwepp-runner hphys0262 -- --nocapture`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass; emitted existing warnings for duplicate lockfile
  entries and unmatched license allowance rows, but final advisories, bans,
  licenses, and sources checks were `ok`.
- `bash tools/release/check_authority_suite_antievasion.sh`: pass.
- `cargo test --test auth11_required_suite_obligation_guards_contract`: pass.
- `git diff --check`: pass.
- `wctl doc-lint --path docs/work-packages/20260603-hphys0262-wb17-pmet-demand-seeding-lineage-closure-001`:
  pass; final post-artifact rerun reported zero files validated.
- `/workdir/wepppy/.venv/bin/python docs/work-packages/20260603-hphys0262-wb17-pmet-demand-seeding-lineage-closure-001/artifacts/hphys0262_diagnostics.py --run-root /tmp/hphys0262_20260603T055648Z`:
  pass.
