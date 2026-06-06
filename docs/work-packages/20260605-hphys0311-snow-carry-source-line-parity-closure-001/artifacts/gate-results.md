# Gate Results

Status: complete

Evidence mode: ran

Static:

- Focused and broad validation were run after diagnostic generation,
  post-review source-line/density/settling repairs, and cache cleanup.

Ran:

- `.venv/bin/python -m py_compile docs/work-packages/20260605-hphys0311-snow-carry-source-line-parity-closure-001/artifacts/hphys0311_snow_carry_source_line_parity.py`
  passed.
- `.venv/bin/python docs/work-packages/20260605-hphys0311-snow-carry-source-line-parity-closure-001/artifacts/hphys0311_snow_carry_source_line_parity.py`
  generated HPHYS0311 artifacts.
- `cargo fmt --check` initially failed on integration-test formatting; `cargo
  fmt` was run and the final `cargo fmt --check` passed.
- `cargo test --test hphys0311_snow_carry_source_line_parity_contract -- --nocapture`
  passed with `5` tests.
- `jq` confirmed `7` groups, `58` represented HPHYS0309 rows, route counts
  `6/1`, and `0` authorized production edits.

Post-verification test-hardening rerun:

- `cargo fmt` ran.
- `cargo fmt --check` passed.
- `cargo test --test hphys0311_snow_carry_source_line_parity_contract -- --nocapture`
  passed with `6` tests after adding explicit source-lineage, density
  inheritance, and settling-threshold assertions.
- `git diff --check` passed.
- Package cache scan found no `__pycache__`, `.pyc`, `.pyo`, `.pytest_cache`,
  `.mypy_cache`, or `.ruff_cache` artifacts.

Final broad closeout rerun:

- `.venv/bin/python` compiled
  `docs/work-packages/20260605-hphys0311-snow-carry-source-line-parity-closure-001/artifacts/hphys0311_snow_carry_source_line_parity.py`
  to `/tmp/hphys0311_snow_carry_source_line_parity.pyc`.
- `.venv/bin/python docs/work-packages/20260605-hphys0311-snow-carry-source-line-parity-closure-001/artifacts/hphys0311_snow_carry_source_line_parity.py`
  regenerated HPHYS0311 artifacts after review repairs.
- `rm -rf docs/work-packages/20260605-hphys0311-snow-carry-source-line-parity-closure-001/artifacts/__pycache__`
  removed transient Python bytecode when present.
- `cargo fmt` ran.
- `cargo fmt --check` passed.
- `bash tools/release/check_authority_suite_antievasion.sh` passed.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`
  passed.
- `cargo test --test hphys0310_prior_day_snow_carry_divergence_contract -- --nocapture`
  passed.
- `cargo test --test hphys0311_snow_carry_source_line_parity_contract -- --nocapture`
  passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed with pre-existing duplicate/unmatched-license
  warnings for `getrandom`, `hashbrown`, `twox-hash`, `ISC`, and
  `Unicode-DFS-2016`.
- `git diff --check` passed.
- Package cache scan found no `__pycache__`, `.pyc`, `.pyo`, `.pytest_cache`,
  `.mypy_cache`, or `.ruff_cache` artifacts.
- `jq` confirmed `7` groups, `58` represented HPHYS0309 rows, route counts
  `6/1`, and `0` authorized production edits.
