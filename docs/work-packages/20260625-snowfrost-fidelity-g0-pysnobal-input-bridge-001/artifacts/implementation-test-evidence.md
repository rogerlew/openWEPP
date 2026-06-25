# Implementation And Test Evidence

Status: executed-hold

Evidence mode: Static + Ran.

Static: added `openwepp-snowbench export-pysnobal` as a diagnostic
`openwepp-runner` binary. The exporter discovers a single legacy `.run` file,
generates a TOML runfile under `target/`, resolves existing sidecars through
runner machinery, builds static runtime surfaces, and emits PySnobal forcing
lanes from SIMIMPL28 hourly forcing.

Static: production `direct_winter_hourly_forcing` remains trigger-gated. The
new `diagnostic_winter_hourly_forcing` wrapper is documented as diagnostic-only
and forces complete export rows for external snow-model input.

Static: added `tools/snowfreeze_observed/pysnobal_compare.py`. The harness
requires an explicit `PYSNOBAL_PYTHON`, validates lineage, invokes
`pysnobal.pysnobal.run_snobal` in a child interpreter, checks finite
nonnegative SWE/depth, checks positive-depth density under `700 kg/m3`, and
fails when positive snow precipitation never creates snow.

Static: after review, the harness now probes PySnobal importability before
site execution and routes import failures to `HOLD-PYSNOBAL-UNAVAILABLE`
instead of treating every child failure as a lane-level sanity failure.

Ran: `cargo build -p openwepp-runner --bin openwepp-snowbench` passed.

Ran: all five fixture exports completed under `target/snowfrost_fidelity_g0`.
Hourly rows per lane: Site 1 `333120`, Site 2 `394488`, Site 3 `298056`,
Site 4 `394488`, Site 5 `394488`.

Ran: `.venv/bin/python -m py_compile tools/snowfreeze_observed/pysnobal_compare.py`
passed.

Ran: one-site PySnobal run for Site 1 passed all three lanes. Max physical
snow depth was `0.983461 m`, `0.988197 m`, and `1.005909 m`; max SWE was
`158.297797`, `158.834102`, and `160.657981 kg/m2`.

Ran: all-site PySnobal run produced metric-bearing summaries for 14 of 15
lanes and closed fail-closed at `HOLD-PYSNOBAL-SANITY-FAILURE` because Site 4
`tg_neg0p5c_zg0p10m` aborted inside PySnobal C code.

Ran: `cargo test --test snowfrost_fidelity_g0_pysnobal_bridge_contract`
passed.

Ran: `cargo test -p openwepp-runner snowbench::tests` passed, covering
calendar continuity, leap-day continuity, non-uniform daily-date rejection, and
invalid calendar day rejection.

Ran: review-driven bridge contract hardening passed: allowed source-class
values are enforced and `precip_mass_mm * snow_precip_fraction` reconstructs
the exported audit snow-precipitation total.
