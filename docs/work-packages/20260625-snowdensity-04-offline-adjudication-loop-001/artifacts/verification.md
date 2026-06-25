# Verification

Ran:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p openwepp-runner snowbench_physics_bulk
cargo test --test snowdensity03_physics_bulk_offline_contract
cargo test --test snowdensity03_physics_bulk_offline_contract physics_bulk_is_confined_to_snowbench_and_diagnostic_surfaces
cargo test --workspace
cargo deny check
wctl doc-lint --path docs/work-packages/README.md
wctl doc-lint --path docs/work-packages/20260625-snowdensity-04-offline-adjudication-loop-001/package.md
git diff --check
cargo build -p openwepp-runner --bin openwepp-snowbench
.venv/bin/python -m py_compile \
  tools/snowfreeze_observed/physics_bulk_snotel_profile.py \
  tools/snowfreeze_observed/physics_bulk_adjudication.py
target/debug/openwepp-snowbench physics-bulk \
  --run-dir tests/fixtures/snotel_observed/snotel_mica_creek_st_joe_id \
  --output-dir target/snowdensity04_smoke \
  --variant dense_slow_melt_v1
.venv/bin/python tools/snowfreeze_observed/physics_bulk_snotel_profile.py \
  --observations-dir tests/fixtures/snotel_observed/observations \
  --output-dir target/snowdensity04_profile_smoke \
  --snowbench-binary target/debug/openwepp-snowbench \
  --variant slow_melt_v1 \
  --site snotel_mica_creek_st_joe_id
.venv/bin/python tools/snowfreeze_observed/physics_bulk_adjudication.py \
  --observations-dir tests/fixtures/snotel_observed/observations \
  --output-dir target/snowdensity04_adjudication \
  --snowbench-binary target/debug/openwepp-snowbench \
  --h-comparator-json target/snowfrost_fidelity_h/three_way_comparison.json
```

Results:

- Formatter: passed.
- Clippy workspace/all-targets with warnings denied: passed.
- Python compile: passed.
- `snowbench_physics_bulk` unit tests: passed, `7 passed`.
- Confinement integration guard: passed.
- Full SNOWDENSITY-03 integration contract: passed, `2 passed`.
- Full workspace tests: passed.
- Cargo deny: passed.
- `wctl doc-lint --path docs/work-packages/README.md`: passed, `1 files
  validated, 0 errors, 0 warnings`.
- `wctl doc-lint --path docs/work-packages/20260625-snowdensity-04-offline-adjudication-loop-001/package.md`:
  passed with `0 files validated, 0 errors, 0 warnings`; use the README lint
  as the substantive Markdown lint evidence.
- Diff whitespace check: passed.
- CLI build: passed.
- Dense-slow-melt CLI smoke: passed.
- Slow-melt one-site profile smoke: passed.
- Four-variant/five-site adjudication: passed, `20` site-variant runs.

Note: `cargo test --workspace` printed an expected checksum mismatch warning
inside `soilauth03_injected_drift_vectors_fail_guards`; that test passed and
the warning is the injected negative-path evidence, not a package failure.
