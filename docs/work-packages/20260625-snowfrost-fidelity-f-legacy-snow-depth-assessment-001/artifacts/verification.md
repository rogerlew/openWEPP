# Verification

Evidence mode: Ran.

Commands:

- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
  - Result: PASS.
- `.venv/bin/python -m py_compile tools/snowfreeze_observed/legacy_snow_compare.py`
  - Result: PASS.
- `.venv/bin/python tools/snowfreeze_observed/legacy_snow_compare.py --observations-dir tests/fixtures/snowfreeze_observed/observations --output-dir target/snowfrost_fidelity_f_legacy_compare --output-json docs/work-packages/20260625-snowfrost-fidelity-f-legacy-snow-depth-assessment-001/artifacts/legacy_snow_comparison.json --output-md docs/work-packages/20260625-snowfrost-fidelity-f-legacy-snow-depth-assessment-001/artifacts/legacy_snow_comparison.md`
  - Result: PASS; all five sites ran.
- `rg -n "treal\\(73\\)|treal\\(75\\)|snodpy\\(iplane\\)|tmpvr7=snodpt|densgt|Snow depth \\(mm\\)|daily winter|Snow-Water" /home/workdir/wepp-forest_260430_baseline/src/bigout.for /home/workdir/wepp-forest_260430_baseline/src/bighdr.for /home/workdir/wepp-forest_260430_baseline/src/outfil.for /home/workdir/wepp-forest_260430_baseline/src/winter.for`
  - Result: PASS; source-line provenance found.
- `rg -n "qwet|Qwet|frzftp" crates || true`
  - Result: PASS; no production crate matches.
- `git diff --check`
  - Result: PASS.

Not run:

- Full workspace Rust tests. This package added Python diagnostic tooling and
  documentation only, and its acceptance gate is the all-site comparator run.
