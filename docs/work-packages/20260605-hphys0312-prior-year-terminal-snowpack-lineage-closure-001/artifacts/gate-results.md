# Gate Results

Status: complete

Evidence mode: ran

Static:

- HPHYS0312 gate evidence is recorded from local validation commands run after
  diagnostic generation.

Ran:

- `.venv/bin/python` compiled
  `docs/work-packages/20260605-hphys0312-prior-year-terminal-snowpack-lineage-closure-001/artifacts/hphys0312_prior_year_terminal_snowpack_lineage.py`
  to `/tmp/hphys0312_prior_year_terminal_snowpack_lineage.pyc`.
- `.venv/bin/python docs/work-packages/20260605-hphys0312-prior-year-terminal-snowpack-lineage-closure-001/artifacts/hphys0312_prior_year_terminal_snowpack_lineage.py`
  generated HPHYS0312 artifacts.
- `cargo fmt --check` passed.
- `bash tools/release/check_authority_suite_antievasion.sh` passed.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`
  passed.
- `cargo test --test hphys0311_snow_carry_source_line_parity_contract -- --nocapture`
  passed.
- `cargo test --test hphys0312_prior_year_terminal_snowpack_lineage_contract -- --nocapture`
  passed with `6` tests.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed with pre-existing duplicate/unmatched-license
  warnings for `getrandom`, `hashbrown`, `twox-hash`, `ISC`, and
  `Unicode-DFS-2016`.
- `git diff --check` passed.
- Package cache scan found no `__pycache__`, `.pyc`, `.pyo`, `.pytest_cache`,
  `.mypy_cache`, or `.ruff_cache` artifacts.
- `jq` confirmed `6` groups, `57` represented HPHYS0309 rows, route counts
  `3/3`, and `0` authorized production edits.
