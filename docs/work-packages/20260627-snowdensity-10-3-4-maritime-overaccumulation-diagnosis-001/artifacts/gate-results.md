# Gate Results

Evidence mode: Ran.

## Commands

| Gate | Result | Notes |
|---|---|---|
| `.venv/bin/python -m py_compile tools/snowfreeze_observed/maritime_overaccumulation_diagnosis.py` | PASS | Python diagnostic syntax check. |
| `cargo build -q -p openwepp-runner --bin openwepp-snowbench` | PASS | Built the diagnostic snowbench binary used by the package. |
| `.venv/bin/python tools/snowfreeze_observed/maritime_overaccumulation_diagnosis.py --output-dir target/snowdensity10_3_4_maritime_overaccumulation_diagnosis --snowbench-binary target/debug/openwepp-snowbench` | PASS | Generated JSON/Markdown reports and copied package artifacts. |
| `find target/snowdensity10_3_4_maritime_overaccumulation_diagnosis -maxdepth 4 -name openwepp-snowbench.stderr -size +0c -print` | PASS | No non-empty snowbench stderr files. |
| `cargo test --test snowdensity10_3_4_maritime_overaccumulation_diagnosis` | PASS | 3 tests passed. |
| `cargo clippy --test snowdensity10_3_4_maritime_overaccumulation_diagnosis -- -D warnings` | PASS | Focused clippy gate passed. |
| `cargo fmt --check` | PASS | Formatting gate passed. |
| `git diff --check` | PASS | Whitespace gate passed. |

## Source Scans

`rg -n "qwet|frzftp" tools/snowfreeze_observed/maritime_overaccumulation_diagnosis.py tests/integration/snowdensity10_3_4_maritime_overaccumulation_diagnosis.rs` returned no matches.

`rg -n "coe_shortwave_albedo_v1|DEFAULT_CANOPY_COVER_FRACTION|physics_bulk_density_compaction_v1" ...` found only the package non-scope text and the guard-test assertion that the diagnostic tool does not use `coe_shortwave_albedo_v1`.

`rg -n "compute_simimpl29_melt_hour|let dmelt|sub.?canopy|longwave|melt_dmelt_in|cancov" ...` confirmed the production CoE path carries `dmelt` rain heat and canopy attenuation, while `snowbench_coe_melt.rs` uses longwave only for cloud-fraction inversion.
