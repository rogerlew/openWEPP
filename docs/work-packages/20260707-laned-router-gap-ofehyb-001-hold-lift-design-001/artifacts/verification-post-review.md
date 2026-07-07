# Post-Review Verification

Status: **PASS**. Evidence mode: **Ran**.

Post-review amendments:

- Shared the source-memory cooldown multiplier between production and Case-4
  harness code.
- Added the multi-burst reset vector.
- Removed stale copied H2637 run logs from package scratch.
- Updated `SC-OFEROUTE-002` and package disposition artifacts.

Verification:

- `cargo nextest run -p openwepp-hillslope-orchestrator hybrid_source_memory --profile quick`:
  4 passed.
- `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing --profile quick`:
  89 passed in `153.339 s`.
- `bash tools/release/check_authority_suite_antievasion.sh`: PASS.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`:
  2 passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo nextest run --workspace --profile full`: 1432 passed, 4 skipped in
  `585.981 s`.
- `cargo deny check`: PASS.

Final release-binary H2637 timing:

- Binary: `target/release/openwepp-cli-hill`
- SHA-256: `628486b358b94bf87f09880c0e3b687a924b33502967e08fba5145b0a8e72f51`
- Command: `OPENWEPP_LANED_ACTIVE=1 OPENWEPP_LANED_ACTIVE_IMPLICIT=1 OPENWEPP_LANED_SHADOW_PROFILE=1 /usr/bin/time -v taskset -c 4 target/release/openwepp-cli-hill --run-dir docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/h2637-scratch --run-file p2637.run.toml --output-dir docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/h2637-scratch/output`
- Result: PASS, `37.96 s` user, `0:37.99` wall.
