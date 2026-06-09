# Gate Results

Evidence mode: Ran

Focused gates run from `/workdir/openWEPP`:

| Command | Result |
|---|---|
| `.venv/bin/python -m py_compile tools/owcmp/owcmp tools/owcmp/*.py` | pass |
| `cargo fmt --check` | pass |
| `cargo test --test owcmp_cli_contract` | pass, `12` tests |
| `cargo test --test owcmp_agent_config_contract` | pass, `2` tests |
| `git diff --check` | pass |
| `.venv/bin/python -m json.tool tools/owcmp/suites/n-idaho-single-ofe-ksflag0.json >/dev/null` | pass |
| `.venv/bin/python -m json.tool tools/owcmp/suites/minnesota-corn-ksflag1.json >/dev/null` | pass |
| `.venv/bin/python -m json.tool tools/owcmp/suites/wa-cascades-mofe-ksflag0.json >/dev/null` | pass |
| `bash tools/release/check_authority_suite_antievasion.sh` | pass |
| `cargo test --test auth11_required_suite_obligation_guards_contract` | pass |
| `find tools/owcmp -type d -name __pycache__ -print` after cleanup | pass, no output |

Full workspace `cargo test`, clippy, and `cargo deny check` were not run. Scope
rationale: OWCMP04 edits Python comparator tooling, agent config/docs, seeded
manifests, and focused integration contracts. It does not change Rust production
kernel code, workspace crate APIs, dependency manifests, formulas, thresholds, or
science contracts.

