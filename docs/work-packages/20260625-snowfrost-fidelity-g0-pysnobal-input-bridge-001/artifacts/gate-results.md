# Gate Results

Status: executed-hold

Evidence mode: Ran.

| Gate | Status | Evidence |
| --- | --- | --- |
| Rust exporter build | PASS | `cargo build -p openwepp-runner --bin openwepp-snowbench` |
| Rust schema/unit tests | PASS | `cargo test --test snowfrost_fidelity_g0_pysnobal_bridge_contract`; `cargo test -p openwepp-runner snowbench::tests` |
| Python harness compile | PASS | `.venv/bin/python -m py_compile tools/snowfreeze_observed/pysnobal_compare.py` |
| One-site PySnobal run | PASS | Site 1 all three `Tg` lanes passed and wrote `pysnobal_site_summary_site1.*`. |
| All-site PySnobal run | HOLD | Current rerun printed `PYSNOBAL_HARNESS_EXIT=1`; 14 of 15 lanes passed; Site 4 `tg_neg0p5c_zg0p10m` failed PySnobal C guard, route `HOLD-PYSNOBAL-SANITY-FAILURE`. |
| Anti-alias gates | PASS | Focused contract and lineage/audit files reject snow-depth/SWE, frost-surface/ground-temperature, and daily-radiation/hourly-Wm2 aliases; precipitation mass reconstruction matches audit totals. |
| `cargo fmt --check` | PASS | Ran after formatting. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Ran after exporter/test edits. |
| `cargo test --workspace` | PASS | Full workspace test completed. |
| `cargo deny check` | PASS | Full deny check completed. |
| `git diff --check` | PASS | Whitespace check completed. |
| Source scan for `qwet|Qwet|frzftp` | PASS | `rg -n "qwet|Qwet|frzftp" crates || true` returned no matches. |
