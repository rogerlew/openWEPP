# CAL-07C Gate Evidence

Evidence class: `Ran`

Commands were run from `/home/workdir/openWEPP`.

| Gate | Command | Result |
| --- | --- | --- |
| CAL-07/07B pre-commit sync | `git fetch origin main`; `git rev-list --left-right --count HEAD...origin/main` | `0 0` before commit. |
| CAL-07 hold validation | `.venv/bin/python docs/work-packages/20260728-canopy-cal-07-southern-hemisphere-robustness-001/tools/validate_hold.py` | PASS: 3 negative VPD days; no partial canopy result. |
| CAL-07B validation | `.venv/bin/python docs/work-packages/20260728-canopy-cal-07b-hourly-vpd-aggregation-diagnostic-001/tools/validate.py` | PASS: 72 positive blocker-date hourly products; 3 daily-summary mismatch attributions. |
| CAL-07/07B docs lint | `markdown-doc lint --path ...` for both packages, roadmap, and catalog | PASS before commit `ab6d84ac`. |
| CAL-07C Python syntax | `.venv/bin/python -m py_compile .../tools/*.py` | PASS. |
| CAL-07C source/admission preparation | `.venv/bin/python .../tools/prepare_inputs.py` | PASS; generated source/admission/forcing artifacts. |
| CAL-07C Rust format | `cargo fmt --manifest-path .../tools/executor/Cargo.toml -- --check` | PASS. |
| CAL-07C Rust check | `cargo check --manifest-path .../tools/executor/Cargo.toml` | PASS. |
| CAL-07C execution | `.venv/bin/python .../tools/execute.py` | PASS; executor output plus focused Rust checks. |
| Producer phase transform | `cargo test -p openwepp-plant-phenology --test native_canopy_contract full_wrapped_nh_climate_phase_flip_preserves_sh_canopy_and_limb_order -- --exact --nocapture` | PASS, 1 test. |
| Real consumer ordering | `cargo test -p openwepp-runner --lib native_forest_yaml_executes_through_the_direct_production_consumer -- --nocapture` | PASS, 1 test. |
| CAL-07C analysis and figures | `.venv/bin/python .../tools/analyze.py`; `.venv/bin/python .../tools/plot.py` | PASS. |
| CAL-07C independent validation | `.venv/bin/python .../tools/validate.py` | PASS: 349 negative hourly components retained; 0 negative admitted daily VPD rows; max VPD residual `0.000e+00 Pa`; max mass residual `2.082e-17 kg m-2`. |
| Figure render check | `rsvg-convert .../artifacts/figures/*.svg -o /tmp/cal07c-figures/*.png` | PASS; visual inspection completed for all four figures. |
| Markdown lint | `markdown-doc lint --path .../CAL-07C`; `markdown-doc lint --path docs/planning/canopy-phenology-assurance-roadmap.md`; `markdown-doc lint --path docs/work-packages/README.md` | PASS: 25 package files, roadmap, and catalog validated with 0 errors and 0 warnings. |
| SVG XML/render | `xmllint --noout .../figures/*.svg`; `rsvg-convert .../figures/*.svg -o /tmp/cal07c-figures/*.png` | PASS for all four figures. |
| Diff hygiene | `git diff --check` | PASS. |
| Terminal review A | `artifacts/review-agent-a.md`; `artifacts/verification-agent-a.md` | PASS for bounded CAL-07C evidence/claim calibration; Order 7 hold retained. |
| Terminal review B | `artifacts/review-agent-b.md`; `artifacts/verification-agent-b.md` | PASS for source custody, admission table, executor path, roadmap/catalog status, no clipping, and no production OBL replacement; Order 7 hold retained. |

No accepted terminal-review finding remains open.
