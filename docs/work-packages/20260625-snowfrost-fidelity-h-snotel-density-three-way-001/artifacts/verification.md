# Verification

Evidence mode: Ran unless noted.

Completed:

- `.venv/bin/python -m py_compile tools/snowfreeze_observed/snotel_density_three_way.py tools/snowfreeze_observed/pysnobal_compare.py`
- `.venv/bin/python -m py_compile tools/snowfreeze_observed/*.py`
- `.venv/bin/python tools/snowfreeze_observed/snotel_density_three_way.py --observations-dir tests/fixtures/snotel_observed/observations validate`
- SNOTEL AWDB fetch and normalization completed.
- Release builds completed before comparison:
  - `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
  - `cargo build --release -p openwepp-runner --bin openwepp-snowbench`
- H comparison rerun completed and emitted
  `target/snowfrost_fidelity_h/three_way_comparison.{json,md}`.
- PySnobal segmented SNOTEL run emitted
  `target/snowfrost_fidelity_h/pysnobal_snotel_summary.{json,md}`.
- `git diff --check`
- `cargo fmt --check`
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo test --test auth11_required_suite_obligation_guards_contract`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo deny check`
- `cargo test --workspace`

All listed gates passed. Package execution is closed as complete-with-disposition:
the CSS PySnobal WY2017 C-core failure is dispositioned in
`pysnobal-css-wy2017-disposition.md` as an upstream PySnobal/SNOBAL thin-snow
numerical instability, not an openWEPP defect or export/forcing artifact.
