# OWCMP04 Disposition

Evidence mode: Ran

Status: complete.

Outcome:

- Added declarative suite manifests for the user-named North Idaho, Minnesota
  corn, and WA Cascades validation cohorts.
- Added `owcmp manifest list/show` and `owcmp env --manifest`.
- Made suite discovery and preflight visible to `comparator_suite_runner`.
- Added reusable work-package prompt guidance and an artifact-retention policy
  that keeps raw comparison outputs local by default.
- Added focused contract tests for the new CLI/config surface.

Final focused gates:

- `.venv/bin/python -m py_compile tools/owcmp/owcmp tools/owcmp/*.py`: pass.
- `cargo fmt --check`: pass.
- `cargo test --test owcmp_cli_contract`: pass, `12` tests.
- `cargo test --test owcmp_agent_config_contract`: pass, `2` tests.
- `git diff --check`: pass.
- JSON manifest parse checks: pass for all three suite files.
- `tools/owcmp/owcmp manifest list --json`: pass, `3` suites.
- `tools/owcmp/owcmp env --manifest ...`: pass for all three suite manifests.
- `bash tools/release/check_authority_suite_antievasion.sh`: pass.
- `cargo test --test auth11_required_suite_obligation_guards_contract`: pass.

Line-count governance:

- Touched `.rs` files are integration tests and remain below the `2000` line
  warning threshold.

Review disposition:

- No review findings.
- No undispositioned findings remain.

Scope note:

- Full workspace `cargo test`, clippy, and `cargo deny check` were not run. This
  package is scoped to Python comparator tooling, agent config/docs, seeded
  manifests, and focused integration-test contracts; no Rust production kernel,
  workspace crate API, dependency manifest, or science contract changed.

