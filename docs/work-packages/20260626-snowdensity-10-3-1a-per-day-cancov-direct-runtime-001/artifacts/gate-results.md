# Gate Results

Evidence class: Ran.

## Focused Compile

- `cargo check -p openwepp-runner`
- Result: pass.

## Focused Runtime/Contract Tests

- `cargo test --test snowdensity10_3_1a_per_day_cancov --test snowfrost_fidelity_g0_pysnobal_bridge_contract`
- Result: pass.
- Coverage:
  - package contract/source guards;
  - `canopy_series.csv` sidecar emission;
  - CoE melt replay consumes sidecar;
  - G0 PySnobal export schema remains valid.

Earlier focused regression run:

- `cargo test --test snowdensity10_3_1a_per_day_cancov --test snowfrost_fidelity_g0_pysnobal_bridge_contract --test snowdensity05g_harness_fidelity_rerun --test snowdensity06b_coe_bound_density_replay`
- Result: initial package wording marker failed, then fixed. Runtime portions passed.

## Required Gates

- `cargo fmt --check`
  - Initial result: failed on new test formatting.
  - Fix: ran `cargo fmt`.
  - Final result: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Initial result: failed on `export_pysnobal_inputs` line count and
    `usize as f64` precision-loss warning.
  - Fix: extracted `write_pysnobal_lane_exports`; changed canopy summary mean to
    checked `u32::try_from` plus `f64::from`.
  - Final result: pass.
- `cargo test --workspace`
  - Result: pass.
- `cargo deny check`
  - Result: pass (`advisories ok, bans ok, licenses ok, sources ok`).
- `git diff --check`
  - Result: pass.
- `rg -n "qwet|frzftp" crates || true`
  - Result: no hits.

## Notes

The snowbench tests are slower after this package because export now runs a
direct-production publication capture to obtain the real daily canopy
trajectory. The package accepts this diagnostic cost because it removes the
stale scalar canopy blocker for later mixed/deciduous melt adjudication.
