# Verification

Ran:

```bash
cargo fmt --check
cargo test -p openwepp-runner snowbench_physics_bulk
cargo test --test snowdensity03_physics_bulk_offline_contract
cargo build -p openwepp-runner --bin openwepp-snowbench
.venv/bin/python -m py_compile tools/snowfreeze_observed/physics_bulk_snotel_profile.py
.venv/bin/python tools/snowfreeze_observed/physics_bulk_snotel_profile.py \
  --observations-dir tests/fixtures/snotel_observed/observations \
  --output-dir target/snowdensity03_physics_bulk_rerun \
  --snowbench-binary target/debug/openwepp-snowbench
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
git diff --check
```

Results:

- `cargo fmt --check`: passed.
- `cargo test -p openwepp-runner snowbench_physics_bulk`: passed,
  `6 passed`.
- `cargo test --test snowdensity03_physics_bulk_offline_contract`: passed,
  `2 passed`.
- `cargo build -p openwepp-runner --bin openwepp-snowbench`: passed.
- Python compile: passed.
- Five-site SNOTEL profile generation: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `cargo deny check`: passed.
- `git diff --check`: passed.

Notes:

- The workspace test output includes intentional checksum-failure text from
  tamper-detection tests; the tests themselves passed.
- No anti-evasion source-level authority-suite script was required because this
  package did not modify external-authority fixture bindings or required-case
  suite posture.
