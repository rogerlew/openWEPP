# Gate Results

Status: complete

Evidence mode: ran

Static:

- Primary gate ledger:
  `artifacts/fixed-baseline-semantic-suite-ledger.json`.

Ran:

- `/workdir/wepppy/.venv/bin/python artifacts/hphys0304_fixed_comparator_semantic_rerun.py --python /workdir/wepppy/.venv/bin/python`:
  pass.
- `python3 -m py_compile artifacts/hphys0304_fixed_comparator_semantic_rerun.py`:
  pass.
- `cargo fmt --check`: pass.
- First `cargo test --test hphys0304_fixed_comparator_semantic_rerun_contract -- --nocapture`:
  failed due brittle test phrase checks against line-wrapped package/prompt
  text; test assertions were narrowed to authored package semantics.
- Final `cargo test --test hphys0304_fixed_comparator_semantic_rerun_contract -- --nocapture`:
  pass, 3 tests.
- Post-review `python3 -m py_compile artifacts/hphys0304_fixed_comparator_semantic_rerun.py`:
  pass.
- Post-review `cargo fmt --check`: pass.
- Post-review `cargo test --test hphys0304_fixed_comparator_semantic_rerun_contract -- --nocapture`:
  pass, 3 tests.
- Post-review `cargo test --test hphys0303_adr0016_comparator_ratification_contract -- --nocapture`:
  pass, 3 tests.
- Post-review `git diff --check`: pass.
